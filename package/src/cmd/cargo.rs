use std::process::Command;

use anyhow::{Context, Result};
use yansi::Paint;

use crate::conf::Configuration;

pub fn build(conf: &Configuration) -> Result<()> {
    let args = create_args(conf);

    let cmd = Paint::new(format!("cargo {}", args.join(" "))).dimmed();
    run_command(&args).context(cmd.clone())?;
    println!("{} finished {}", Paint::green("      UniFFI").bold(), cmd);
    Ok(())
}

fn run_command(args: &[&str]) -> Result<()> {
    let mut cmd = Command::new("cargo").args(args).spawn()?;
    let status = cmd.wait()?;
    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("cargo build failed with status: {:?}", status.code());
    }
}
fn create_args(conf: &Configuration) -> Vec<&str> {
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
    args
}
