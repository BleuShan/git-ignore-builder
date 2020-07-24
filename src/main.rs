#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]

mod cli;
mod prelude;

use crate::prelude::*;
use cli::*;

#[async_std::main]
async fn main() -> Result<()> {
    let args = CommandLineArgs::parse();
    println!("{:?}", args);
    Ok(())
}
