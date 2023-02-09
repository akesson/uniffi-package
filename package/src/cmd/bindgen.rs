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

    let ffi_name = &conf.ffi_module_name;
    let name = &conf.module_name;

    let dest_header_fldr = format!("{name}.package/Sources/{name}Lib/Headers");
    let dest_header_file = format!("{name}.package/Sources/{name}Lib/Headers/{ffi_name}.h");
    let dest_module_file = format!("{name}.package/Sources/{name}Lib/Headers/{ffi_name}.modulemap");
    let dest_swift_file_ = format!("{name}.package/Sources/{name}Lib/{name}.swift");

    let dest_include_folder = format!("target/uniffi/include");
    let dest_include_header = format!("target/uniffi/include/{ffi_name}.h");
    let dest_include_module = format!("target/uniffi/include/{ffi_name}.modulemap");

    let src_header = format!("target/uniffi/{ffi_name}.h");
    let src_module = format!("target/uniffi/{ffi_name}.modulemap");
    let src_swift_ = format!("target/uniffi/{name}.swift");

    fs_err::create_dir_all(&dest_include_folder)?;
    fs_err::create_dir_all(&dest_header_fldr)?;

    fs_err::copy(&src_header, &dest_include_header)?;
    fs_err::rename(&src_header, &dest_header_file)?;

    fs_err::copy(&src_module, &dest_include_module)?;
    fs_err::rename(&src_module, &dest_module_file)?;

    fs_err::copy(&src_swift_, &dest_swift_file_)?;

    Ok(())
}

pub fn write_uniffi_toml(_conf: &Configuration) -> Result<()> {
    // can be used to set various UniFFI options
    // see: https://mozilla.github.io/uniffi-rs/swift/configuration.html
    let contents = format!(
        r###"[bindings.swift]
"###,
    );
    fs_err::write("target/uniffi/uniffi.toml", contents)?;
    Ok(())
}
