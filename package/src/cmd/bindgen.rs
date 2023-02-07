use crate::conf::Configuration;
use anyhow::Result;
use fs_err;

pub fn run(conf: &Configuration) -> Result<()> {
    fs_err::create_dir_all("target/uniffi")?;
    let args = vec![
        "run",
        "--release",
        "--features=cli",
        "--bin=uniffi-bindgen",
        "generate",
        "--language=swift",
        "--out-dir=target/uniffi",
        conf.udl_file.as_ref(),
    ];
    super::run_cargo(&args)
}
