use crate::conf::Configuration;
use anyhow::Result;

pub fn run(conf: &Configuration) -> Result<()> {
    lipo_mac(&conf)?;
    lipo_ios_sim(&conf)
}

fn lipo_mac(conf: &Configuration) -> Result<()> {
    let profile = &conf.profile;
    let name = &conf.lib_name;

    let lib1 = format!("target/x86_64-apple-darwin/{profile}/lib{name}.a");
    let lib2 = format!("target/aarch64-apple-darwin/{profile}/lib{name}.a");

    fs_err::create_dir_all("target/uniffi")?;
    let out = format!("target/uniffi/lib{name}_macos.a");

    let args = vec!["-create", &lib1, &lib2, "-output", &out];
    super::run("lipo", &args)
}

fn lipo_ios_sim(conf: &Configuration) -> Result<()> {
    let profile = &conf.profile;
    let name = &conf.lib_name;

    let lib1 = format!("target/x86_64-apple-ios/{profile}/lib{name}.a");
    let lib2 = format!("target/aarch64-apple-ios-sim/{profile}/lib{name}.a");

    fs_err::create_dir_all("target/uniffi")?;
    let out = format!("target/uniffi/lib{name}_ios_sim.a");

    let args = vec!["-create", &lib1, &lib2, "-output", &out];
    super::run("lipo", &args)
}
