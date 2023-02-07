use crate::conf::Configuration;
use anyhow::Result;
use fs_err;

pub fn run(conf: &Configuration) -> Result<()> {
    fs_err::create_dir_all("target/uniffi")?;
    write_uniffi_toml(conf)?;
    let args = vec![
        "run",
        "--release",
        "--features=cli",
        "--bin=uniffi-bindgen",
        "generate",
        "--config=target/uniffi/uniffi.toml",
        "--language=swift",
        "--out-dir=target/uniffi",
        conf.udl_file.as_ref(),
    ];
    super::run_cargo(&args)?;
    let name = &conf.package_name;
    fs_err::create_dir("target/uniffi/include")?;
    fs_err::rename(
        format!("target/uniffi/{name}.h"),
        format!("target/uniffi/include/{name}.h"),
    )?;
    fs_err::rename(
        format!("target/uniffi/{name}.modulemap"),
        format!("target/uniffi/include/{name}.modulemap"),
    )?;

    fs_err::create_dir_all(format!("{name}.package/Sources/{name}Lib"))?;
    fs_err::rename(
        format!("target/uniffi/{name}.swift"),
        format!("{name}.package/Sources/{name}Lib/{name}.swift"),
    )?;

    Ok(())
}

pub fn write_uniffi_toml(conf: &Configuration) -> Result<()> {
    let contents = format!(
        r###"[bindings.swift]
ffi_module_name = "{}""###,
        conf.package_name
    );
    fs_err::write("target/uniffi/uniffi.toml", contents)?;
    Ok(())
}
