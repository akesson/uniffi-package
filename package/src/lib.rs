mod conf;
mod ext;

use anyhow::Result;
pub use conf::Cli;

use crate::conf::Configuration;

pub fn run(cli: Cli) -> Result<()> {
    let conf = Configuration::load(cli)?;
    println!("{conf:?}");
    Ok(())
}
