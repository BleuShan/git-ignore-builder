use super::Configuration;
use crate::prelude::*;
use async_std::{
    sync::Arc,
    task,
};
use git2::{
    build::RepoBuilder as GitRepositoryBuilder,
    RemoteCallbacks as GitRemoteCallbacks,
    Repository as GitRepository,
};
use once_cell::sync::OnceCell;

pub struct Repository {
    config: Arc<Configuration>,
    inner: OnceCell<GitRepository>,
}

impl Repository {
    pub fn new(config: Arc<Configuration>) -> Self {
        Self {
            config,
            inner: OnceCell::default(),
        }
    }

    pub async fn open(&mut self) -> Result<()> {
        let configuration = self.config.clone();
        let _old = self.inner.take();
        let new = task::spawn_blocking(move || {
            let path = configuration.repository_dir();
            if path.exists() {
                GitRepository::open(path)
            } else {
                let url = configuration.repository_url();
                GitRepository::clone(&url, path)
            }
        })
        .await?;

        self.inner.set(new).map_err(|_| anyhow!("already opened."))
    }
}
