use crate::prelude::*;
pub use clap::Clap;

/// A tool to build gitignore files from github's gitignore template repository
#[derive(Clap, Debug)]
pub struct CommandLineArgs {
    /// Enable the inclusion of global .gitignore files templates
    #[clap(short, long)]
    global: bool,
    /// Enable the inclusion of community .gitignore files templates
    #[clap(short, long)]
    community: bool,
    /// A list of .gitignore file names in the downloaded repository to include
    names: Vec<String>,
}
