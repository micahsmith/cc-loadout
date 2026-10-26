mod common;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::process::{Child, Stdio};
use std::time::{Duration, Instant};

use assert_cmd::assert::OutputAssertExt;
use common::Sandbox;
use serde_json::Value;

const ORIGIN: &str = "https://github.com/acme/orders.git";

const MODEL_TOON: &str = "\
entities[2]:
  - name: Order
    fields[1]{name,type,nullable}:
      id,uuid,false
    relations[1]{target,cardinality}:
      Customer,one
  - name: Customer
    fields[1]{name,type,nullable}:
      id,uuid,false
";

/// A freeform page longer than the 500 characters a detail view shows.
fn sketch() -> String {
    format!(
        "<!doctype html>\n<html><body>\n<h1>Checkout sketch</h1>\n<p>caf\u{e9} &amp; tea</p>\n\
         <!-- {} -->\n</body></html>\n",
        "x".repeat(600)
    )
}

fn submit(sandbox: &Sandbox, dir: &Path, kind: &str, name: &str, contents: &str) -> Value {
    let file = sandbox.file(&format!("{name}.{kind}"), contents);
    let file = file.to_str().unwrap();
    sandbox.json(
        dir,
        &[
            "design", "submit", "--kind", kind, "--name", name, "--file", file,
        ],
    )
}

fn submit_id(sandbox: &Sandbox, dir: &Path, kind: &str, name: &str, contents: &str) -> String {
    let submitted = submit(sandbox, dir, kind, name, contents);
    submitted["design"]["id"].as_str().unwrap().to_owned()
}

/// A `skald serve` process, killed when the test ends.
struct Server {
    child: Child,
    url: String,
}

impl Server {
    fn start(sandbox: &Sandbox, dir: &Path, id: &str, idle_timeout: &str) -> Self {
        let mut child = sandbox
            .cmd_in(dir)
            .args(["serve", "--design", id, "--version", "1"])
            .args(["--idle-timeout", idle_timeout])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        // The pipe is dropped after the first line, as `design open` drops it.
        let mut line = String::new();
        BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut line)
            .unwrap();
        let url = line
            .strip_prefix("url: ")
            .and_then(|rest| rest.strip_suffix('\n'))
            .unwrap_or_else(|| panic!("first line: {line:?}"))
            .to_owned();
        Self { child, url }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

struct Response {
    status: u16,
    content_type: String,
    body: Vec<u8>,
}

fn get(url: &str, path: &str) -> Response {
    let address = url
        .strip_prefix("http://")
        .and_then(|rest| rest.strip_suffix('/'))
        .unwrap_or_else(|| panic!("url: {url}"));
    let mut stream = TcpStream::connect(address).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    write!(stream, "GET {path} HTTP/1.0\r\n\r\n").unwrap();
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).unwrap();

    let end = bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .expect("a blank line after the headers");
    let head = String::from_utf8(bytes[..end].to_vec()).unwrap();
    let mut lines = head.split("\r\n");
    let status = lines.next().unwrap().split(' ').nth(1).unwrap();
    let content_type = lines
        .filter_map(|line| line.split_once(':'))
        .find(|(name, _)| name.eq_ignore_ascii_case("content-type"))
        .map(|(_, value)| value.trim().to_owned())
        .unwrap_or_default();
    Response {
        status: status.parse().unwrap(),
        content_type,
        body: bytes[end + 4..].to_vec(),
    }
}

#[test]
fn freeform_submission_is_stored_and_read_back() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin(ORIGIN);
    let page = sketch();

    let submitted = submit(&sandbox, &repo, "freeform", "sketch", &page);
    assert_eq!(submitted["design"]["kind"], "freeform");
    assert_eq!(submitted["design"]["version"], 1);
    let id = submitted["design"]["id"].as_str().unwrap();

    let full = sandbox.json(&repo, &["design", "get", id, "--full"]);
    assert_eq!(full["design"]["version"], 1);
    assert_eq!(full["html"], page);

    let short = sandbox.json(&repo, &["design", "get", id]);
    let html = short["html"].as_str().unwrap();
    assert!(
        html.ends_with(&format!(
            " ... (truncated, {} chars total)",
            page.chars().count()
        )),
        "html: {html}"
    );
    assert!(
        short["help"]
            .to_string()
            .contains(&format!("skald design get {id} --full")),
        "help: {}",
        short["help"]
    );

    let assert = sandbox
        .cmd_in(&repo)
        .args(["design", "get", id, "--full"])
        .assert()
        .success();
    let toon = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(toon.contains("\nhtml: "), "stdout:\n{toon}");
    let from_toon: Value = toon_format::decode_default(&toon).unwrap();
    assert_eq!(from_toon, full);
}

#[test]
fn freeform_resubmission_is_versioned() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin(ORIGIN);
    let id = submit_id(&sandbox, &repo, "freeform", "sketch", "<p>one</p>\n");

    let again = submit(&sandbox, &repo, "freeform", "sketch", "<p>one</p>\n");
    assert_eq!(again["design"]["status"], "unchanged");
    assert_eq!(again["design"]["version"], 1);

    let next = submit(&sandbox, &repo, "freeform", "sketch", "<p>two</p>\n");
    assert_eq!(next["design"]["id"], id.as_str());
    assert_eq!(next["design"]["status"], "created");
    assert_eq!(next["design"]["version"], 2);
    assert!(!next["help"].to_string().contains("design diff"));

    let first = sandbox.json(&repo, &["design", "get", &id, "--version", "1"]);
    assert_eq!(first["html"], "<p>one</p>\n");
    let newest = sandbox.json(&repo, &["design", "get", &id]);
    assert_eq!(newest["html"], "<p>two</p>\n");
}

#[test]
fn freeform_page_is_served_unchanged() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin(ORIGIN);
    let page = sketch();
    let id = submit_id(&sandbox, &repo, "freeform", "sketch", &page);

    let server = Server::start(&sandbox, &repo, &id, "5");
    assert!(
        server.url.starts_with("http://127.0.0.1:"),
        "{}",
        server.url
    );
    let response = get(&server.url, "/");
    assert_eq!(response.status, 200);
    assert!(
        response.content_type.starts_with("text/html"),
        "{}",
        response.content_type
    );
    assert_eq!(response.body, page.as_bytes());
}

#[test]
fn typed_page_served_equals_the_export() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin(ORIGIN);
    let id = submit_id(&sandbox, &repo, "data-model", "orders", MODEL_TOON);
    sandbox.json(&repo, &["design", "render", &id, "--out", "a.html"]);
    let exported = std::fs::read(repo.join("a.html")).unwrap();

    let server = Server::start(&sandbox, &repo, &id, "5");
    let response = get(&server.url, "/");
    assert_eq!(response.status, 200);
    assert_eq!(response.content_type, "text/html; charset=utf-8");
    assert_eq!(response.body, exported);
}

#[test]
fn server_exits_after_idle_timeout() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin(ORIGIN);
    let id = submit_id(&sandbox, &repo, "freeform", "sketch", "<p>one</p>\n");

    let mut server = Server::start(&sandbox, &repo, &id, "1");
    let deadline = Instant::now() + Duration::from_secs(3);
    let status = loop {
        if let Some(status) = server.child.try_wait().unwrap() {
            break status;
        }
        assert!(Instant::now() < deadline, "the server is still running");
        std::thread::sleep(Duration::from_millis(50));
    };
    assert_eq!(status.code(), Some(0));
}

#[test]
fn unknown_path_is_404() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin(ORIGIN);
    let id = submit_id(&sandbox, &repo, "freeform", "sketch", "<p>one</p>\n");

    let server = Server::start(&sandbox, &repo, &id, "5");
    assert_eq!(get(&server.url, "/nope").status, 404);
    assert_eq!(get(&server.url, "/").status, 200);
}

#[test]
fn open_prints_the_url_and_the_server_answers() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin(ORIGIN);
    let id = submit_id(&sandbox, &repo, "data-model", "orders", MODEL_TOON);

    let started = Instant::now();
    let opened = sandbox.json(
        &repo,
        &["design", "open", &id, "--no-browser", "--idle-timeout", "5"],
    );
    assert!(started.elapsed() < Duration::from_secs(2));
    let opened = &opened["opened"];
    assert_eq!(opened["design"], id.as_str());
    assert_eq!(opened["version"], 1);
    assert_eq!(opened["browser"], "skipped");
    let url = opened["url"].as_str().unwrap();
    assert!(url.starts_with("http://127.0.0.1:"), "url: {url}");
    assert_eq!(get(url, "/").status, 200);
}

#[cfg(unix)]
#[test]
fn open_without_a_launcher_is_still_a_success() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin(ORIGIN);
    let id = submit_id(&sandbox, &repo, "data-model", "orders", MODEL_TOON);
    let git = std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|dir| dir.join("git"))
        .find(|path| path.is_file())
        .expect("git on PATH");
    std::os::unix::fs::symlink(git, sandbox.bin.join("git")).unwrap();

    let output = sandbox
        .cmd_in(&repo)
        .env("PATH", &sandbox.bin)
        .args(["design", "open", &id, "--idle-timeout", "1", "--json"])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(0), "stdout:\n{stdout}");
    let opened: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(opened["opened"]["browser"], "unavailable");
}

#[test]
fn freeform_diff_is_refused() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin(ORIGIN);
    let id = submit_id(&sandbox, &repo, "freeform", "sketch", "<p>one</p>\n");

    let (code, error) =
        sandbox.json_with_code(&repo, &["design", "diff", &id, "--from", "1", "--to", "1"]);
    assert_eq!(code, 1, "stdout:\n{error:#}");
    assert_eq!(error["error"]["code"], "not-comparable");
    assert!(!error["help"].as_array().unwrap().is_empty());
}

#[test]
fn empty_freeform_submission_is_rejected() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin(ORIGIN);
    let file = sandbox.file("empty.html", "");

    let (code, error) = sandbox.json_with_code(
        &repo,
        &[
            "design",
            "submit",
            "--kind",
            "freeform",
            "--name",
            "sketch",
            "--file",
            file.to_str().unwrap(),
        ],
    );
    assert_eq!(code, 1, "stdout:\n{error:#}");
    assert_eq!(error["error"]["code"], "empty-submission");
    let list = sandbox.json(&repo, &["design", "list"]);
    assert_eq!(list["count"], "0 of 0 total");
}
