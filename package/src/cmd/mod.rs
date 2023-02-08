use anyhow::{anyhow, Result};
use std::process::Command;
use yansi::Paint;

pub mod bindgen;
pub mod cargo;
pub mod lipo;
pub mod swift_package;
pub mod xcodebuild;

fn run_cargo(args: &[&str]) -> Result<()> {
    run("cargo", args)
}

fn run(program: &str, args: &[&str]) -> Result<()> {
    let mut cmd = Command::new(program).args(args).spawn()?;
    let status = cmd.wait()?;
    let cmd = Paint::new(format!("{} {}", program, args.join(" "))).dimmed();
    if status.success() {
        println!("{} finished {}", Paint::green("      UniFFI").bold(), cmd);
        Ok(())
    } else {
        println!("{} error {}", Paint::red("      UniFFI").bold(), cmd);
        Err(anyhow!("Command failed with status: {:?}", status.code()))
    }
}
