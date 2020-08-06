pub mod entry;
#[cfg(test)]
mod tests;

use crate::prelude::*;
use entry::{
    ConfigEntry,
    ConfigEntryGetter,
    ConfigEntrySetter,
    WithConfigSetterKind,
};
use git2::Config as GitConfig;
use std::path::{
    Path,
    PathBuf,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Config {
    path: Option<PathBuf>,
}

impl Config {
    pub fn new<PathRef>(path: PathRef) -> Self
    where
        PathRef: AsRef<Path>,
    {
        let path = path.as_ref().to_owned().into();
        Self { path }
    }

    pub fn get<Entry>(&self, entry: Entry) -> Result<Entry::Value>
    where
        Entry: ConfigEntry + ConfigEntryGetter,
    {
        let source = self.get_gitconfig()?.snapshot()?;
        entry.get(source)
    }

    pub fn get_or_default<Entry>(&self, entry: Entry) -> Entry::Value
    where
        Entry: ConfigEntry + ConfigEntryGetter,
    {
        self.get_gitconfig()
            .map(|source| entry.get(source))
            .flatten()
            .unwrap_or_else(|_| Entry::default_value())
    }

    pub fn set<Entry, Value>(&self, entry: Entry, value: Value) -> Result<()>
    where
        Entry: ConfigEntry + ConfigEntrySetter,
        Entry::Value: WithConfigSetterKind,
        Value: Into<Entry::Value>,
    {
        let source = self.get_gitconfig()?;
        entry.set(source, value.into())
    }

    fn get_gitconfig(&self) -> Result<GitConfig> {
        match self.path {
            Some(ref path) => GitConfig::open(path),
            None => GitConfig::open_default(),
        }
        .map_err(|e| e.into())
    }
}
