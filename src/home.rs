use std::path::Path;

use serde_json::{Map, Value, json};

use crate::adapter::{self, AdapterStatus, PluginStatus};
use crate::design;
use crate::error::SkaldError;
use crate::git;
use crate::output::Output;
use crate::paths;
use crate::repository::RepoContext;
use crate::store::{self, Store};

const ABOUT: &str = "skald keeps design intent for a repository outside its working tree, keyed to \
                     its git history";

const NEWEST_DESIGNS: usize = 5;

pub fn view(cwd: &Path) -> Result<Output, SkaldError> {
    let (repo, designs, designs_count) = match git::resolve(cwd)? {
        Some(info) => {
            let mut store = Store::open(&paths::data_dir()?)?;
            let tx = store.transaction()?;
            let repo = RepoContext::find_or_insert(&tx, info)?;
            let (designs, total) = design::summaries(&tx, &repo.id, NEWEST_DESIGNS)?;
            tx.commit()?;
            (Some(repo), designs, total)
        }
        None => (None, Vec::new(), 0),
    };
    let adapters = adapter::adapters(&paths::data_dir()?)?;
    let plugin = adapter::plugin(&paths::claude_plugins_dir()?);

    let mut view = Map::new();
    view.insert("bin".to_owned(), Value::from(bin()));
    view.insert("about".to_owned(), Value::from(ABOUT));
    view.insert("version".to_owned(), Value::from(env!("CARGO_PKG_VERSION")));
    let mut help = Vec::new();
    match &repo {
        Some(repo) => {
            view.insert(
                "repository".to_owned(),
                json!({"identity": repo.identity, "branch": repo.branch}),
            );
            help.push(design::SUBMIT_HELP);
        }
        None => {
            view.insert("repository".to_owned(), Value::Null);
            view.insert(
                "note".to_owned(),
                Value::from(format!(
                    "no git repository found at {}; designs are kept per repository",
                    cwd.display()
                )),
            );
            help.push("cd <repository> && skald");
        }
    }
    view.insert(
        "designs".to_owned(),
        designs.iter().map(|design| design.item(&[])).collect(),
    );
    view.insert("designs_count".to_owned(), json!(designs_count));
    view.insert("adapters".to_owned(), json!(adapters));
    view.insert("plugin".to_owned(), json!(plugin));
    if !designs.is_empty() {
        help.push("skald design get <id>");
    }
    if designs_count > designs.len() {
        help.push("skald design list");
    }
    let adapters_current = !adapters.is_empty()
        && adapters
            .iter()
            .all(|adapter| adapter.status == AdapterStatus::Current);
    if !adapters_current || plugin.status == PluginStatus::Missing {
        help.push("skald setup");
    }
    if plugin.status == PluginStatus::Outdated {
        help.push(store::UPGRADE);
    }
    Ok(Output::new(view).with_help(help.into_iter().map(String::from)))
}

/// What a session hook prints when the home view fails: one line, so that the session still
/// starts.
pub fn unavailable(err: SkaldError) -> Output {
    let mut view = Map::new();
    view.insert(
        "note".to_owned(),
        Value::from(format!("skald cannot show the home view: {}", err.message)),
    );
    Output::new(view)
}

fn bin() -> String {
    match std::env::current_exe() {
        Ok(exe) => paths::collapse_home(&exe, paths::home_dir().as_deref()),
        Err(_) => "skald".to_owned(),
    }
}
