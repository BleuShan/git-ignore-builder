mod getter;
mod setter;

use crate::{
    constants::GIT_CONFIG_SECTION_NAME,
    prelude::*,
};
pub use getter::ConfigEntryGetter;
use once_cell::sync::Lazy;
pub use setter::{
    ConfigEntrySetter,
    WithConfigSetterKind,
};
use std::{
    env,
    path::PathBuf,
};

pub trait ConfigEntry: Copy + Send + Sync + 'static {
    type Value: Sized;

    const SCOPED_NAME: &'static str;
    const NAME: Lazy<String> =
        Lazy::new(|| format!("{}.{}", GIT_CONFIG_SECTION_NAME, Self::SCOPED_NAME));

    fn default_value() -> Self::Value;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RemoteUrl;

impl ConfigEntry for RemoteUrl {
    type Value = String;
    const SCOPED_NAME: &'static str = "remoteUrl";

    fn default_value() -> Self::Value {
        "https://github.com/github/gitignore.git".to_owned()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IgnoreDirectory;

impl ConfigEntry for IgnoreDirectory {
    type Value = PathBuf;
    const SCOPED_NAME: &'static str = "ignoreDirectory";

    fn default_value() -> Self::Value {
        let mut path = dirs::config_dir().unwrap_or_else(|| env::temp_dir());
        path.push("git-ignore-builder");
        path
    }
}
