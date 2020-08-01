#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]

mod cli;
mod prelude;
mod repository;

use crate::prelude::*;
use cli::*;
use repository::Repository;

#[async_std::main]
async fn main() -> Result<()> {
    let config = Configuration::load()?;
    let repo = Repository::builder().configure(&config).build()?;

    Ok(())
}
