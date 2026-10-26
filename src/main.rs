mod adapter;
mod cli;
mod design;
mod error;
mod git;
mod home;
mod id;
mod output;
mod paths;
mod repository;
mod serve;
mod setup;
mod store;

use std::ffi::OsString;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use cli::{Cli, Command, DevCommand, Parsed, Runtime};
use error::SkaldError;
use output::{Format, Output};
use store::Store;

fn main() -> ExitCode {
    run(
        std::env::args_os(),
        &mut std::io::stdout(),
        &mut std::io::stderr(),
    )
}

fn run(
    args: impl IntoIterator<Item = OsString>,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> ExitCode {
    let args: Vec<OsString> = args.into_iter().collect();
    let format = Format::from_args(&args);
    let result = cli::parse(&args).and_then(|parsed| match parsed {
        Parsed::Print(text) => Ok((text, 0)),
        Parsed::Run(cli) => dispatch(cli, stdout).map(|output| {
            let exit_code = output.exit_code();
            (output.encode(format), exit_code)
        }),
    });
    let (text, exit_code) = result.unwrap_or_else(|err| {
        let exit_code = err.exit_code;
        (err.into_output().encode(format), exit_code)
    });
    if let Err(err) = stdout
        .write_all(text.as_bytes())
        .and_then(|()| stdout.flush())
    {
        let _ = writeln!(stderr, "skald: cannot write to stdout: {err}");
        return ExitCode::FAILURE;
    }
    ExitCode::from(exit_code)
}

fn dispatch(cli: Cli, stdout: &mut impl Write) -> Result<Output, SkaldError> {
    match cli.command {
        None => home::view(&current_dir()?),
        Some(Command::Setup { no_plugin }) => setup::run(no_plugin),
        Some(Command::Hook {
            runtime: Runtime::ClaudeCode,
        }) => Ok(current_dir()
            .and_then(|cwd| home::view(&cwd))
            .unwrap_or_else(home::unavailable)),
        Some(Command::Design { command }) => {
            // Every design command needs the store, so a store this program cannot read is
            // reported before anything else.
            let mut store = Store::open(&paths::data_dir()?)?;
            design::run(command, &mut store, &current_dir()?)
        }
        Some(Command::Dev {
            command: DevCommand::Seed { designs },
        }) => {
            let mut store = Store::open(&paths::data_dir()?)?;
            design::seed(&mut store, &current_dir()?, designs)
        }
        Some(Command::Serve {
            design,
            version,
            idle_timeout,
            port,
        }) => {
            let mut store = Store::open(&paths::data_dir()?)?;
            let (_, page) = design::served(&mut store, &current_dir()?, &design, Some(version))?;
            drop(store);
            serve::serve(page, port, idle_timeout, stdout)?;
            Ok(Output::Raw(String::new()))
        }
    }
}

fn current_dir() -> Result<PathBuf, SkaldError> {
    std::env::current_dir().map_err(|err| {
        SkaldError::new(
            "no-working-directory",
            format!("cannot read the working directory: {err}"),
        )
    })
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    fn read(path: &str) -> String {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("cannot read {}: {err}", path.display()))
    }

    fn read_json(path: &str) -> Value {
        serde_json::from_str(&read(path)).unwrap()
    }

    /// The runners a job runs on, with `${{ matrix.os }}` expanded from its matrix.
    fn runners(job: &serde_yaml::Value) -> Vec<String> {
        match job["runs-on"].as_str() {
            Some(runner) if !runner.contains("${{") => vec![runner.to_owned()],
            _ => {
                let matrix = &job["strategy"]["matrix"];
                let listed = matrix["os"].as_sequence().into_iter().flatten();
                let included = matrix["include"].as_sequence().into_iter().flatten();
                listed
                    .chain(included.map(|entry| &entry["os"]))
                    .filter_map(|os| os.as_str().map(str::to_owned))
                    .collect()
            }
        }
    }

    #[test]
    fn release_workflow_builds_version_tags_on_linux_and_macos() {
        let workflow: serde_yaml::Value =
            serde_yaml::from_str(&read(".github/workflows/release.yml")).unwrap();
        let tags = workflow["on"]["push"]["tags"]
            .as_sequence()
            .expect("on.push.tags is a list");
        assert!(
            tags.iter().any(|tag| tag.as_str() == Some("v*")),
            "tags: {tags:?}"
        );

        let jobs = workflow["jobs"].as_mapping().expect("jobs is a mapping");
        let runners: Vec<String> = jobs.values().flat_map(runners).collect();
        for runner in ["ubuntu-latest", "macos-latest"] {
            assert!(runners.iter().any(|r| r == runner), "runners: {runners:?}");
        }
    }

    #[test]
    fn marketplace_points_at_the_plugin_directory() {
        let marketplace = read_json(".claude-plugin/marketplace.json");
        assert_eq!(marketplace["name"], "skald");
        let plugins = marketplace["plugins"].as_array().unwrap();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0]["name"], "code-skillset");
        assert_eq!(plugins[0]["source"], "./plugin");

        let plugin = read_json("plugin/.claude-plugin/plugin.json");
        assert_eq!(plugin["metadata"]["skaldMinVersion"], "0.1.0");
    }
}
