use super::ConfigEntry;
use crate::prelude::*;
use git2::Config as GitConfig;
use std::path::PathBuf;

#[derive(Debug, From)]
pub enum ConfigSetterKind {
    Bool(bool),
    I32(i32),
    I64(i64),
    Str(String),
}

impl ConfigSetterKind {
    fn set(self, mut source: GitConfig, name: &str) -> Result<()> {
        match self {
            Self::Bool(value) => source.set_bool(name, value),
            Self::I32(value) => source.set_i32(name, value),
            Self::I64(value) => source.set_i64(name, value),
            Self::Str(value) => source.set_str(name, value.as_str()),
        }
        .map_err(|e| e.into())
    }
}

pub trait WithConfigSetterKind {
    fn kind(self) -> ConfigSetterKind;
}

impl WithConfigSetterKind for PathBuf {
    fn kind(self) -> ConfigSetterKind {
        ConfigSetterKind::Str(self.as_path().display().to_string())
    }
}

impl<T> WithConfigSetterKind for T
where
    T: Into<ConfigSetterKind>,
{
    fn kind(self) -> ConfigSetterKind {
        self.into()
    }
}

pub trait ConfigEntrySetter: ConfigEntry
where
    Self::Value: WithConfigSetterKind,
{
    fn set(&self, source: GitConfig, value: Self::Value) -> Result<()>;
}

impl<T> ConfigEntrySetter for T
where
    T: ConfigEntry,
    Self::Value: WithConfigSetterKind,
{
    fn set(&self, source: GitConfig, value: Self::Value) -> Result<()> {
        value.kind().set(source, Self::NAME.as_str())
    }
}
