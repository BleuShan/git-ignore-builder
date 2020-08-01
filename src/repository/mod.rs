mod builder;

pub use builder::RepositoryBuilder;

use crate::prelude::*;
use git2::Repository as GitRepository;
use std::path::PathBuf;

#[derive(Deref)]
pub struct Repository {
    #[deref]
    inner: GitRepository,
}

impl From<GitRepository> for Repository {
    fn from(inner: GitRepository) -> Self {
        Self { inner }
    }
}

impl Repository {
    pub fn builder() -> RepositoryBuilder {
        RepositoryBuilder::default()
    }
}
