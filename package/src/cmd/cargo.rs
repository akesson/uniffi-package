use anyhow::Result;

use crate::conf::Configuration;

pub fn build(conf: &Configuration) -> Result<()> {
    let mut args = vec![];

    if conf.swift.release_optimize_std_lib {
        args.push("+nightly");
        args.push("-Z");
        args.push("build-std=std,panic_abort");
    }

    args.push("build");

    if conf.release {
        args.push("--release");
    }

    args.push("--lib");
    args.push("--target=aarch64-apple-ios");
    args.push("--target=aarch64-apple-ios-sim");
    args.push("--target=x86_64-apple-ios");
    args.push("--target=x86_64-apple-darwin");
    args.push("--target=aarch64-apple-darwin");

    super::run_cargo(&args)
}
