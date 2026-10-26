use std::ffi::OsString;
use std::path::PathBuf;

use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{ArgAction, CommandFactory, Parser, Subcommand};

use crate::design::Kind;
use crate::error::SkaldError;

const HELP: &str = "\
Usage: skald [COMMAND] [FLAGS]

Records, versions, and renders design intent for the current git repository.

Commands:
  skald                   Show the home view for the current repository
  skald setup             Install the Claude Code session hook and the plugin
  skald hook claude-code  Print the home view for a Claude Code SessionStart hook
  skald design submit     Submit a design as its next version
  skald design list       List the designs of the current repository
  skald design get        Show one version of a design
  skald design diff       Compare two versions of a design
  skald design render     Write a design as a standalone HTML page
  skald design open       Open a design in the browser
  skald serve             Serve one design version (internal, used by `design open`)

Run `skald <command> --help` for the flags of one command.

Flags:
  --json             Print JSON instead of TOON
  -h, --help         Print help
  -V, -v, --version  Print the version

Output goes to stdout as TOON, errors included. stderr carries diagnostics only.
Every command ends with one of these exit codes:
  0  success, including no-ops
  1  error
  2  usage error: unknown flag, unknown command, or missing argument
";

#[derive(Debug, Parser)]
#[command(
    name = "skald",
    version,
    disable_version_flag = true,
    disable_help_subcommand = true,
    override_help = HELP
)]
pub struct Cli {
    #[arg(long, global = true)]
    json: bool,
    #[arg(short = 'V', short_alias = 'v', long, action = ArgAction::Version)]
    version: (),
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Setup {
        #[arg(long)]
        no_plugin: bool,
    },
    Hook {
        #[command(subcommand)]
        runtime: Runtime,
    },
    Design {
        #[command(subcommand)]
        command: DesignCommand,
    },
    // Test-only, and documented as such in CLAUDE.md.
    #[command(hide = true)]
    Dev {
        #[command(subcommand)]
        command: DevCommand,
    },
    #[command(hide = true)]
    Serve {
        #[arg(long)]
        design: String,
        #[arg(long)]
        version: u32,
        #[arg(long, value_name = "SECONDS")]
        idle_timeout: Option<u64>,
        #[arg(long, default_value_t = 0)]
        port: u16,
    },
}

#[derive(Debug, Subcommand)]
pub enum Runtime {
    ClaudeCode,
}

#[derive(Debug, Subcommand)]
pub enum DevCommand {
    Seed {
        #[arg(long, value_name = "N")]
        designs: usize,
    },
}

#[derive(Debug, Subcommand)]
pub enum DesignCommand {
    Submit {
        #[arg(long)]
        kind: Kind,
        #[arg(long)]
        name: String,
        #[arg(long, value_name = "PATH|-")]
        file: String,
    },
    List {
        #[arg(long, value_name = "N")]
        limit: Option<usize>,
        #[arg(long, value_delimiter = ',')]
        fields: Vec<String>,
    },
    Get {
        id: String,
        #[arg(long, value_name = "N")]
        version: Option<u32>,
        #[arg(long)]
        full: bool,
    },
    Diff {
        id: String,
        #[arg(long, value_name = "N")]
        from: u32,
        #[arg(long, value_name = "N")]
        to: u32,
    },
    Render {
        id: String,
        #[arg(long, value_name = "N")]
        version: Option<u32>,
        #[arg(long)]
        out: PathBuf,
    },
    Open {
        id: String,
        #[arg(long, value_name = "N")]
        version: Option<u32>,
        #[arg(long)]
        no_browser: bool,
        #[arg(long, value_name = "SECONDS")]
        idle_timeout: Option<u64>,
    },
}

pub enum Parsed {
    Run(Cli),
    Print(String),
}

pub fn parse(args: &[OsString]) -> Result<Parsed, SkaldError> {
    let attempt = Attempt::from_args(args);
    match Cli::try_parse_from(args) {
        Ok(cli) => Ok(Parsed::Run(cli)),
        Err(err) => match err.kind() {
            ErrorKind::DisplayHelp => Ok(Parsed::Print(err.render().to_string())),
            ErrorKind::DisplayVersion => {
                Ok(Parsed::Print(format!("{}\n", env!("CARGO_PKG_VERSION"))))
            }
            _ => Err(usage_error(&err, &attempt)),
        },
    }
}

/// The deepest command named in the arguments. It is found by walking the command tree rather
/// than by parsing, so that a failed parse can still name the command the caller tried to run.
struct Attempt {
    path: String,
    flags: Vec<String>,
    commands: Vec<String>,
}

impl Attempt {
    fn from_args(args: &[OsString]) -> Self {
        let mut root = Cli::command();
        root.build();
        let mut path = root.get_name().to_owned();
        let mut node = &root;
        let words = args.iter().skip(1).filter_map(|arg| arg.to_str());
        for word in words.filter(|word| !word.starts_with('-')) {
            let Some(sub) = node.find_subcommand(word) else {
                break;
            };
            path = format!("{path} {}", sub.get_name());
            node = sub;
        }
        let flags = node
            .get_arguments()
            .filter(|arg| !arg.is_hide_set())
            .filter_map(|arg| arg.get_long())
            .map(|long| format!("--{long}"))
            .collect();
        let commands = node
            .get_subcommands()
            .filter(|sub| !sub.is_hide_set())
            .map(|sub| sub.get_name().to_owned())
            .collect();
        Self {
            path,
            flags,
            commands,
        }
    }

    fn flags_help(&self) -> String {
        format!("Valid flags: {}", self.flags.join(", "))
    }

    fn commands_help(&self) -> String {
        format!("Valid commands: {}", self.commands.join(", "))
    }
}

fn usage_error(err: &clap::Error, attempt: &Attempt) -> SkaldError {
    let path = &attempt.path;
    let invalid_arg = context(err, ContextKind::InvalidArg);
    match err.kind() {
        ErrorKind::UnknownArgument if invalid_arg.starts_with('-') => SkaldError::new(
            "unknown-flag",
            format!("unknown flag {invalid_arg} for `{path}`"),
        )
        .with_help(attempt.flags_help()),
        ErrorKind::UnknownArgument => SkaldError::new(
            "unexpected-argument",
            format!("unexpected argument {invalid_arg} for `{path}`"),
        )
        .with_help(attempt.flags_help()),
        ErrorKind::InvalidSubcommand => SkaldError::new(
            "unknown-command",
            format!(
                "unknown command {} for `{path}`",
                context(err, ContextKind::InvalidSubcommand)
            ),
        )
        .with_help(attempt.commands_help()),
        ErrorKind::MissingSubcommand | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
            SkaldError::new("missing-command", format!("missing command for `{path}`"))
                .with_help(attempt.commands_help())
        }
        ErrorKind::MissingRequiredArgument => SkaldError::new(
            "missing-argument",
            format!("missing {invalid_arg} for `{path}`"),
        )
        .with_help(attempt.flags_help()),
        ErrorKind::InvalidValue | ErrorKind::ValueValidation => {
            let code = if invalid_arg.starts_with("--kind ") {
                "invalid-kind"
            } else {
                "invalid-value"
            };
            let error = SkaldError::new(code, clap_message(err));
            match context(err, ContextKind::ValidValue) {
                valid if valid.is_empty() => error.with_help(attempt.flags_help()),
                valid => error.with_help(format!("Valid values: {valid}")),
            }
        }
        _ => SkaldError::new("invalid-usage", clap_message(err)).with_help(attempt.flags_help()),
    }
}

fn context(err: &clap::Error, kind: ContextKind) -> String {
    match err.get(kind) {
        Some(ContextValue::String(value)) => value.clone(),
        Some(ContextValue::Strings(values)) => values.join(", "),
        _ => String::new(),
    }
}

fn clap_message(err: &clap::Error) -> String {
    let rendered = err.render().to_string();
    let first_line = rendered.lines().next().unwrap_or_default();
    first_line
        .strip_prefix("error: ")
        .unwrap_or(first_line)
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_error(list: &[&str]) -> SkaldError {
        let args: Vec<OsString> = list.iter().map(OsString::from).collect();
        match parse(&args) {
            Err(err) => err,
            Ok(_) => panic!("{list:?} parsed"),
        }
    }

    #[test]
    fn clap_definitions_are_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn unknown_flag_names_the_command_and_its_flags() {
        let err = parse_error(&["skald", "design", "list", "--bogus"]);
        assert_eq!(err.code, "unknown-flag");
        assert_eq!(err.message, "unknown flag --bogus for `skald design list`");
        assert_eq!(err.help, ["Valid flags: --limit, --fields, --json, --help"]);
    }

    #[test]
    fn unknown_command_lists_visible_commands() {
        let err = parse_error(&["skald", "frobnicate", "design"]);
        assert_eq!(err.code, "unknown-command");
        assert_eq!(err.message, "unknown command frobnicate for `skald`");
        assert_eq!(err.help, ["Valid commands: setup, hook, design"]);
    }

    #[test]
    fn missing_command_is_a_usage_error() {
        let err = parse_error(&["skald", "design"]);
        assert_eq!(err.code, "missing-command");
        assert_eq!(err.exit_code, 2);
    }

    #[test]
    fn missing_argument_is_a_usage_error() {
        let err = parse_error(&["skald", "design", "diff", "d1", "--from", "1"]);
        assert_eq!(err.code, "missing-argument");
        assert_eq!(err.message, "missing --to <N> for `skald design diff`");
        assert_eq!(err.exit_code, 2);
    }

    #[test]
    fn invalid_kind_lists_valid_kinds() {
        let err = parse_error(&[
            "skald", "design", "submit", "--kind", "bogus", "--name", "n", "--file", "-",
        ]);
        assert_eq!(err.code, "invalid-kind");
        assert_eq!(err.exit_code, 2);
        assert_eq!(err.help, ["Valid values: data-model, interface, freeform"]);
    }

    #[test]
    fn invalid_value_names_the_flags() {
        let err = parse_error(&["skald", "design", "list", "--limit", "many"]);
        assert_eq!(err.code, "invalid-value");
        assert_eq!(err.exit_code, 2);
        assert_eq!(err.help, ["Valid flags: --limit, --fields, --json, --help"]);
    }

    #[test]
    fn positional_where_none_is_expected_is_unexpected() {
        let err = parse_error(&["skald", "design", "list", "orders"]);
        assert_eq!(err.code, "unexpected-argument");
    }
}
