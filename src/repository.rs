use std::path::PathBuf;

use rusqlite::{OptionalExtension, Transaction, params};

use crate::error::SkaldError;
use crate::git::{self, RepoInfo};
use crate::id;

#[expect(dead_code, reason = "nothing reads default_branch or root yet")]
pub struct RepoContext {
    pub id: String,
    pub identity: String,
    pub default_branch: String,
    pub branch: String,
    pub root: PathBuf,
}

impl RepoContext {
    pub fn find_or_insert(tx: &Transaction<'_>, info: RepoInfo) -> Result<Self, SkaldError> {
        let found = tx
            .query_row(
                "SELECT id, default_branch FROM repository WHERE identity = ?1",
                [&info.identity],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        let (id, default_branch) = match found {
            Some(row) => row,
            None => {
                let id = id::new('y');
                let default_branch = git::default_branch(&info.root)?;
                tx.execute(
                    "INSERT INTO repository (id, identity, default_branch, created_at)
                     VALUES (?1, ?2, ?3, strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))",
                    params![id, info.identity, default_branch],
                )?;
                (id, default_branch)
            }
        };
        Ok(Self {
            id,
            identity: info.identity,
            default_branch,
            branch: info.branch,
            root: info.root,
        })
    }
}
