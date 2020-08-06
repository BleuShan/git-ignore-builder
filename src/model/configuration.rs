use crate::prelude::*;
use async_std::fs;
use clap::Clap;
use std::{
    collections::HashSet,
    path::{
        Path,
        PathBuf,
    },
    sync::Arc,
};
use std::{
    env,
    path::PathBuf,
};
use once_cell::sync::Lazy;

pub static DEFAULT_CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::config_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(env::temp_dir);
    path.push(env!("CARGO_PKG_NAME"));
    path
});

pub static DEFAULT_IGNORE_REPO_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = DEFAULT_CONFIG_DIR.clone();
    path.push("gitignore");
    path
});

#[derive(Clap, Clone, Debug)]
#[clap(about)]
pub struct Configuration {
    /// Enable the inclusion of global .gitignore files templates.
    #[clap(short, long = "include_global")]
    global: bool,

    /// Enable the inclusion of community .gitignore files templates.
    #[clap(short, long = "include_community")]
    community: bool,

    /// Indicates if some options and flags should be save to a config file.
    /// Uses the file provided by the --config options if present.
    #[clap(short, long)]
    #[serde(skip)]
    save_config: bool,

    /// A list of .gitignore file names in the downloaded repository to include.
    #[clap(name = "NAMES")]
    ignore_names: Vec<String>,

    /// A path to a config file.
    #[clap(name = "config", long, value_name = "FILE")]
    #[serde(skip)]
    filepath: Option<PathBuf>,

    /// path to a gitignore repository clone directory.
    #[clap(long, value_name = "DIRECTORY")]
    repository_dir: Option<PathBuf>,

    /// url of the git remote.
    #[clap(long, value_name = "URL")]
    repository_url: Option<String>,
}

impl Configuration {
    pub async fn load() -> Result<Arc<Self>> {
        let config = Self::parse().fill_missing_defaults();
        let filepath = config.filepath();

        if filepath.exists() {
            let bytes = fs::read(filepath).await?;
            let loaded_config: Self = toml::from_slice(bytes.as_slice())?;
            Ok(loaded_config.merge(config).into())
        } else {
            Ok(config.into())
        }
    }

    pub async fn save_if_needed(&self) -> Result<()> {
        if self.save_config {
            let content = toml::to_string_pretty(self)?;
            let filepath = self.filepath();
            if let Some(parent) = filepath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).await?;
                }
            }

            fs::write(filepath, content).await.map_err(|e| e.into())
        } else {
            Ok(())
        }
    }

    pub fn include_global(&self) -> bool {
        self.global
    }

    pub fn include_community(&self) -> bool {
        self.community
    }

    pub fn filepath(&self) -> &Path {
        self.filepath
            .as_ref()
            .unwrap_or_else(|| &DEFAULT_CONFIG_FILE_PATH)
    }

    pub fn repository_dir(&self) -> &Path {
        self.repository_dir
            .as_ref()
            .unwrap_or_else(|| &DEFAULT_REPO_DIR)
    }

    pub fn repository_url(&self) -> String {
        self.repository_url
            .clone()
            .unwrap_or_else(|| DEFAULT_REPO_URL.to_owned())
    }

    fn merge(mut self, other: Self) -> Self {
        self.global = other.global || self.global;
        self.community = other.community || self.community;

        let new_url = other.repository_url();
        if self.repository_url() != new_url {
            self.repository_url.replace(new_url);
        }

        let new_repo_dir = other.repository_dir();
        if self.repository_dir() != new_repo_dir {
            self.repository_dir.replace(new_repo_dir.to_path_buf());
        }

        if self.ignore_names != other.ignore_names {
            let capacity = self.ignore_names.capacity() + other.ignore_names.capacity();
            let mut names: HashSet<String> = HashSet::with_capacity(capacity);
            for name in &self.ignore_names {
                names.insert(name.clone());
            }

            for name in other.ignore_names {
                names.insert(name);
            }

            self.ignore_names.clear();
            for name in names {
                self.ignore_names.push(name);
            }
        }

        self
    }

    fn fill_missing_defaults(mut self) -> Self {
        if self.filepath.is_none() {
            self.filepath.replace(DEFAULT_CONFIG_FILE_PATH.clone());
        }

        if self.repository_dir.is_none() {
            self.repository_dir.replace(DEFAULT_REPO_DIR.clone());
        }

        if self.repository_url.is_none() {
            self.repository_url.replace(DEFAULT_REPO_URL.to_owned());
        }

        if !self.save_config {
            let filepath = self.filepath();
            self.save_config = !filepath.exists() && *DEFAULT_CONFIG_FILE_PATH == *filepath;
        }

        self
    }
}
