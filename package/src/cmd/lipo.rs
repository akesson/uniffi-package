use crate::conf::Configuration;
use anyhow::Result;

pub fn run(conf: &Configuration) -> Result<Vec<String>> {
    let ios_src = format!(
        "target/aarch64-apple-ios/{}/lib{}.a",
        &conf.profile, &conf.cargo_lib_target_name
    );
    fs_err::create_dir_all("target/uniffi/libs")?;
    let ios_dest = format!(
        "target/uniffi/libs/lib{}_ios.a",
        &conf.cargo_lib_target_name
    );
    fs_err::copy(ios_src, &ios_dest)?;
    Ok(vec![lipo_mac(&conf)?, lipo_ios_sim(&conf)?, ios_dest])
}

fn lipo_mac(conf: &Configuration) -> Result<String> {
    let profile = &conf.profile;
    let name = &conf.cargo_lib_target_name;

    let lib1 = format!("target/x86_64-apple-darwin/{profile}/lib{name}.a");
    let lib2 = format!("target/aarch64-apple-darwin/{profile}/lib{name}.a");

    let out = format!("target/uniffi/libs/lib{name}_macos.a");

    let args = vec!["-create", &lib1, &lib2, "-output", &out];
    super::run("lipo", &args)?;
    Ok(out)
}

fn lipo_ios_sim(conf: &Configuration) -> Result<String> {
    let profile = &conf.profile;
    let name = &conf.cargo_lib_target_name;

    let lib1 = format!("target/x86_64-apple-ios/{profile}/lib{name}.a");
    let lib2 = format!("target/aarch64-apple-ios-sim/{profile}/lib{name}.a");

    let out = format!("target/uniffi/libs/lib{name}_ios_sim.a");

    let args = vec!["-create", &lib1, &lib2, "-output", &out];
    super::run("lipo", &args)?;
    Ok(out)
}
