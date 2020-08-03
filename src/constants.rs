use once_cell::sync::Lazy;
use std::{
    env,
    path::PathBuf,
};

pub static DEFAULT_REPO_URL: &str = "https://github.com/github/gitignore.git";

pub static CONFIG_FILE_NAME: Lazy<String> =
    Lazy::new(|| format!("{}.config.toml", env!("CARGO_PKG_NAME")));

pub static DEFAULT_CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::config_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(env::temp_dir);
    path.push(env!("CARGO_PKG_NAME"));
    path
});

pub static DEFAULT_REPO_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = DEFAULT_CONFIG_DIR.clone();
    path.push("gitignore");
    path
});

pub static DEFAULT_CONFIG_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = DEFAULT_CONFIG_DIR.clone();
    path.push(CONFIG_FILE_NAME.as_str());
    path
});
