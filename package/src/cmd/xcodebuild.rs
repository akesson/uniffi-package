use crate::conf::Configuration;
use anyhow::{Ok, Result};

pub fn run(conf: &Configuration, libs: Vec<String>) -> Result<()> {
    let mut args = vec!["-create-xcframework"];
    let header = format!("target/uniffi/include");

    for lib in &libs {
        args.push("-library");
        args.push(&lib);
        args.push("-headers");
        args.push(&header);
    }

    let out = format!("target/uniffi/{}.xcframework", conf.package_name);
    args.push("-output");
    args.push(&out);

    super::run("xcodebuild", &args)?;
    Ok(())
}
