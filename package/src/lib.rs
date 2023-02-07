mod cmd;
mod conf;
mod ext;

use std::{env, path::Path};

use anyhow::Result;
pub use conf::Cli;

use crate::{
    cmd::{bindgen, cargo, lipo, swift_package, xcodebuild},
    conf::Configuration,
};

pub fn run(cli: Cli) -> Result<()> {
    let conf = Configuration::load(cli)?;

    env::set_current_dir(&conf.dir)?;

    let out = "target/uniffi";
    if Path::new(&out).exists() {
        fs_err::remove_dir_all(&out)?;
    }
    let pack = format!("{}.package", &conf.module_name);
    if Path::new(&pack).exists() {
        fs_err::remove_dir_all(&pack)?;
    }

    cargo::build(&conf)?;
    bindgen::run(&conf)?;
    let libs = lipo::run(&conf)?;
    xcodebuild::run(&conf, libs)?;
    swift_package::run(&conf)?;
    Ok(())
}
