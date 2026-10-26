use std::collections::{HashMap, HashSet};

use serde_json::{Map, Value};

use super::Problem;
use super::diff::{self, Record};
use crate::id;

/// Every kind of element that a typed record holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Element {
    Entity,
    Field,
    Relation,
    Operation,
    Input,
    Output,
    Error,
}

impl Element {
    pub fn noun(self) -> &'static str {
        match self {
            Self::Entity => "entity",
            Self::Field => "field",
            Self::Relation => "relation",
            Self::Operation => "operation",
            Self::Input => "input",
            Self::Output => "output",
            Self::Error => "error",
        }
    }

    pub fn assign(self, supplied: Option<&str>) -> String {
        let prefix = match self {
            Self::Entity => 'e',
            Self::Field => 'f',
            Self::Relation => 'r',
            Self::Operation => 'o',
            Self::Input | Self::Output => 'p',
            Self::Error => 'x',
        };
        supplied.map_or_else(|| id::new(prefix), str::to_owned)
    }

    pub fn for_diff<const N: usize>(
        self,
        id: &str,
        attributes: [(&'static str, Value); N],
    ) -> diff::Element {
        diff::Element {
            kind: self.noun(),
            id: id.to_owned(),
            attributes: attributes.into(),
        }
    }
}

/// Collects problems while tracking which ids the submission may keep and which it has used.
pub struct Check<'a> {
    /// The element kind of every id in the previous version.
    known: HashMap<String, &'static str>,
    used: HashSet<&'a str>,
    problems: Vec<Problem>,
}

impl<'a> Check<'a> {
    /// `previous` is the newest stored record of the design, whose ids a submission may keep.
    pub fn new(previous: Option<&impl Record>) -> Self {
        let known = previous
            .map(|record| record.elements())
            .unwrap_or_default()
            .into_iter()
            .map(|element| (element.id, element.kind))
            .collect();
        Self {
            known,
            used: HashSet::new(),
            problems: Vec::new(),
        }
    }

    pub fn report(&mut self, path: &str, id: Option<&str>, code: &'static str, message: String) {
        self.problems.push(Problem {
            path: path.to_owned(),
            id: id.map(str::to_owned),
            code,
            message,
        });
    }

    pub fn id(&mut self, element: Element, supplied: Option<&'a str>, path: &str) {
        let Some(id) = supplied else { return };
        let noun = element.noun();
        if !self.used.insert(id) {
            self.report(
                path,
                supplied,
                "duplicate-id",
                format!("id `{id}` is already used by an earlier element"),
            );
        } else if self.known.get(id) != Some(&noun) {
            self.report(
                path,
                supplied,
                "unknown-id",
                format!(
                    "the newest version of this design has no {noun} with id `{id}`; omit `id` \
                     to create a new {noun}"
                ),
            );
        }
    }

    pub fn name(
        &mut self,
        siblings: &mut HashSet<&'a str>,
        element: Element,
        name: Option<&'a str>,
        path: &str,
        id: Option<&str>,
    ) -> String {
        match name {
            Some(name) if !name.trim().is_empty() => {
                self.unique(siblings, element, name, path, id);
            }
            _ => self.report(
                path,
                id,
                "missing-name",
                format!("{} has no name", element.noun()),
            ),
        }
        name.unwrap_or_default().to_owned()
    }

    pub fn unique(
        &mut self,
        siblings: &mut HashSet<&'a str>,
        element: Element,
        name: &'a str,
        path: &str,
        id: Option<&str>,
    ) {
        if !siblings.insert(name) {
            self.report(
                path,
                id,
                "duplicate-name",
                format!("an earlier {} is already named `{name}`", element.noun()),
            );
        }
    }

    /// Returns `value`, and reports `code` when it is missing or blank.
    pub fn required(
        &mut self,
        element: Element,
        attribute: &str,
        code: &'static str,
        value: Option<&str>,
        path: &str,
        id: Option<&str>,
    ) -> String {
        let value = value.unwrap_or_default();
        if value.trim().is_empty() {
            let message = format!("{} has no {attribute}", element.noun());
            self.report(path, id, code, message);
        }
        value.to_owned()
    }

    pub fn unknown_keys(
        &mut self,
        unknown: &Map<String, Value>,
        path: &str,
        id: Option<&str>,
        noun: &str,
    ) {
        for key in unknown.keys() {
            let pointer = key.replace('~', "~0").replace('/', "~1");
            self.report(
                &format!("{path}/{pointer}"),
                id,
                "unknown-key",
                format!("`{key}` is not a key of {noun}"),
            );
        }
    }

    pub fn finish<T>(self, record: T) -> Result<T, Vec<Problem>> {
        if self.problems.is_empty() {
            Ok(record)
        } else {
            Err(self.problems)
        }
    }
}
