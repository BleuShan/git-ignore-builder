use super::*;
use crate::cli::Configuration;
use git2::{
    build::RepoBuilder as GitRepositoryBuilder,
    AutotagOption,
    FetchOptions as GitFetchOptions,
    RemoteCallbacks,
};

const DEFAULT_URL: &str = "https://github.com/github/gitignore.git";

#[derive(Default, Clone)]
pub struct RepositoryBuilder {
    url: Option<String>,
    path: Option<PathBuf>,
}

impl RepositoryBuilder {
    fn default_url() -> String {
        DEFAULT_URL.to_owned()
    }

    fn create_fetch_options(&self) -> GitFetchOptions<'_> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.transfer_progress(|progress| {
            println!(
                "{}/{}",
                progress.received_objects(),
                progress.total_objects()
            );
            true
        });

        let mut options = GitFetchOptions::new();
        options
            .remote_callbacks(callbacks)
            .download_tags(AutotagOption::All);
        options
    }

    pub fn url<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<String>,
    {
        let new = value.into();
        if !new.is_empty() {
            self.url = Some(new)
        }
        self
    }

    pub fn path<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<PathBuf>,
    {
        self.path = Some(value.into());
        self
    }

    pub fn configure(&mut self, config: &Configuration) -> &mut Self {
        if let Some(url) = config.repository_url() {
            self.url(url);
        }

        self.path(config.repository_dir());

        self
    }

    pub fn build(&mut self) -> Result<Repository> {
        let path = self
            .path
            .take()
            .ok_or_else(|| anyhow!("No path provided"))?;
        let url = self.url.take().unwrap_or_else(Self::default_url);
        let fetch_options = self.create_fetch_options();

        if path.exists() {
            let repo = GitRepository::open(&path)?;
            Ok(repo.into())
        } else {
            Ok(GitRepositoryBuilder::new()
                .fetch_options(fetch_options)
                .clone(&url, &path)?
                .into())
        }
    }
}
