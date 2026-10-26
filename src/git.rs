use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::error::SkaldError;

pub struct RepoInfo {
    pub identity: String,
    pub branch: String,
    pub root: PathBuf,
}

/// Identifies the repository that holds `cwd`, or `None` outside a git worktree.
pub fn resolve(cwd: &Path) -> Result<Option<RepoInfo>, SkaldError> {
    let Some(paths) = git(
        cwd,
        &[
            "rev-parse",
            "--path-format=absolute",
            "--show-toplevel",
            "--git-common-dir",
        ],
    )?
    else {
        return Ok(None);
    };
    let mut lines = paths.lines();
    let (Some(root), Some(common_dir)) = (lines.next(), lines.next()) else {
        return Err(SkaldError::new(
            "git-failed",
            format!("git rev-parse printed unexpected output: {paths}"),
        ));
    };
    let identity = match git(cwd, &["remote", "get-url", "origin"])? {
        Some(url) => url,
        // The common directory is shared by every worktree, so its parent is the main worktree.
        None => Path::new(common_dir)
            .parent()
            .unwrap_or(Path::new(common_dir))
            .display()
            .to_string(),
    };
    // `rev-parse --abbrev-ref HEAD` fails on a branch with no commits; `symbolic-ref` does not.
    let branch = git(cwd, &["symbolic-ref", "--quiet", "--short", "HEAD"])?
        .unwrap_or_else(|| "detached".to_owned());
    Ok(Some(RepoInfo {
        identity,
        branch,
        root: PathBuf::from(root),
    }))
}

/// The target of `origin/HEAD`, otherwise `main` if that branch exists, otherwise `master`.
pub fn default_branch(root: &Path) -> Result<String, SkaldError> {
    let origin_head = git(
        root,
        &[
            "symbolic-ref",
            "--quiet",
            "--short",
            "refs/remotes/origin/HEAD",
        ],
    )?;
    if let Some(branch) = origin_head
        .as_deref()
        .and_then(|target| target.strip_prefix("origin/"))
    {
        return Ok(branch.to_owned());
    }
    let has_main = git(
        root,
        &["rev-parse", "--verify", "--quiet", "refs/heads/main"],
    )?;
    Ok(if has_main.is_some() { "main" } else { "master" }.to_owned())
}

/// Returns `None` when git exits non-zero.
fn git(dir: &Path, args: &[&str]) -> Result<Option<String>, SkaldError> {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .map_err(|err| SkaldError::new("git-unavailable", format!("cannot run git: {err}")))?;
    if !output.status.success() {
        return Ok(None);
    }
    Ok(Some(
        String::from_utf8_lossy(&output.stdout).trim().to_owned(),
    ))
}
