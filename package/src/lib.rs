mod cmd;
mod conf;
mod ext;

use std::{env, path::Path};

use anyhow::Result;
pub use conf::Cli;

use crate::{
    cmd::{bindgen, cargo, lipo, xcodebuild},
    conf::Configuration,
};

pub fn run(cli: Cli) -> Result<()> {
    let conf = Configuration::load(cli)?;

    env::set_current_dir(&conf.dir)?;

    let out = "target/uniffi";
    if Path::new(&out).exists() {
        fs_err::remove_dir_all(&out)?;
    }

    cargo::build(&conf)?;
    bindgen::run(&conf)?;
    let libs = lipo::run(&conf)?;
    xcodebuild::run(&conf, libs)?;
    Ok(())
}
