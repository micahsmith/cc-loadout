mod common;

use std::path::Path;

use assert_cmd::assert::OutputAssertExt;
use common::Sandbox;
use rusqlite::Connection;
use serde_json::Value;

fn user_version(store: &Path) -> i64 {
    Connection::open(store)
        .unwrap()
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .unwrap()
}

#[test]
fn fresh_store_migrates_to_the_newest_version() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin("https://github.com/acme/orders.git");

    sandbox.cmd_in(&repo).assert().success();
    assert!(sandbox.store_path().exists());
    let version = user_version(&sandbox.store_path());
    assert!(version >= 1, "user_version is {version}");

    sandbox.cmd_in(&repo).assert().success();
    assert_eq!(user_version(&sandbox.store_path()), version);
}

#[test]
fn store_newer_than_the_program_is_refused() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin("https://github.com/acme/orders.git");
    std::fs::create_dir_all(&sandbox.data).unwrap();
    Connection::open(sandbox.store_path())
        .unwrap()
        .pragma_update(None, "user_version", 999)
        .unwrap();

    for args in [&["design", "list", "--json"][..], &["--json"]] {
        let output = sandbox.cmd_in(&repo).args(args).output().unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(output.status.code(), Some(1), "stdout:\n{stdout}");
        let value: Value = serde_json::from_str(&stdout).unwrap();
        assert_eq!(value["error"]["code"], "store-newer-than-program");
        let help = value["help"].to_string();
        assert!(help.contains("cargo install"), "help: {help}");
    }
    assert_eq!(user_version(&sandbox.store_path()), 999);
}

#[test]
fn data_directory_defaults_to_home_and_can_be_overridden() {
    let sandbox = Sandbox::new();
    let repo = sandbox.repo_with_origin("https://github.com/acme/orders.git");

    let alt = sandbox.root.join("alt");
    sandbox
        .cmd_in(&repo)
        .env("SKALD_HOME", &alt)
        .assert()
        .success();
    assert!(alt.join("skald.db").exists());
    assert!(!sandbox.home.join(".skald").exists());

    sandbox
        .cmd_in(&repo)
        .env_remove("SKALD_HOME")
        .assert()
        .success();
    assert!(sandbox.home.join(".skald/skald.db").exists());
}
