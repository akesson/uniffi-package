mod cmd;
mod conf;
mod ext;

use std::env;

use anyhow::Result;
pub use conf::Cli;

use crate::{cmd::cargo, conf::Configuration};

pub fn run(cli: Cli) -> Result<()> {
    let conf = Configuration::load(cli)?;

    env::set_current_dir(&conf.dir)?;
    cargo::build(&conf)?;
    Ok(())
}
