use crate::{
    prelude::*,
    Configuration,
    Repository,
};
use async_std::sync::Arc;
use once_cell::sync::OnceCell;

#[derive(Default)]
pub struct State {
    configuration: OnceCell<Arc<Configuration>>,
    repository: OnceCell<Repository>,
}

impl State {
    pub fn configuration(&self) -> Option<Arc<Configuration>> {
        self.configuration.get().cloned()
    }

    pub async fn load_configuration(&mut self) -> Result<Arc<Configuration>> {
        let configuration = Configuration::load().await?;
        self.configuration
            .set(configuration.clone())
            .map_err(|_| anyhow!("Already loaded"))?;
        Ok(configuration)
    }
}
