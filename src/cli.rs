use crate::prelude::*;
use clap::Clap;
use once_cell::sync::Lazy;
use std::{
    env,
    path::{
        Path,
        PathBuf,
    },
};

static DEFAULT_CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::config_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(env::temp_dir);
    path.push(env!("CARGO_PKG_NAME"));
    path
});

static DEFAULT_REPO_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = DEFAULT_CONFIG_DIR.clone();
    path.push("gitignore");
    path
});

static DEFAULT_CONFIG_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = DEFAULT_CONFIG_DIR.clone();
    path.push("config.toml");
    path
});

#[derive(Serialize, Deserialize, Clap, Debug)]
#[clap(about)]
pub struct Configuration {
    /// Enable the inclusion of global .gitignore files templates
    #[clap(short, long)]
    global: bool,

    /// Enable the inclusion of community .gitignore files templates
    #[clap(short, long)]
    community: bool,

    /// A list of .gitignore file names in the downloaded repository to include
    #[clap(name = "NAMES")]
    #[serde(skip)]
    names: Vec<String>,

    /// A path to the config file
    #[clap(name = "config-filepath", long)]
    #[serde(skip)]
    filepath: Option<PathBuf>,

    /// path to a gitignore repository clone directory
    #[clap(long)]
    repository_dir: Option<PathBuf>,

    /// url of the git remote
    #[clap(long)]
    #[serde(skip)]
    repository_url: Option<String>,
}

impl Configuration {
    pub fn load() -> Result<Self> {
        let config = Self::parse();
        Ok(config)
    }

    pub fn global(&self) -> bool {
        self.global
    }

    pub fn community(&self) -> bool {
        self.community
    }

    pub fn filepath(&self) -> &Path {
        match self.filepath.as_ref() {
            Some(path) => path.as_path(),
            _ => DEFAULT_CONFIG_FILE_PATH.as_path(),
        }
    }

    pub fn repository_dir(&self) -> &Path {
        match self.repository_dir.as_ref() {
            Some(path) => path.as_path(),
            _ => DEFAULT_REPO_DIR.as_path(),
        }
    }
    pub fn repository_url(&self) -> Option<String> {
        self.repository_url.clone()
    }
}
