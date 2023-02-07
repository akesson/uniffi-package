use crate::{
    ext::{MetadataExt, PathBufExt},
    Cli,
};

use super::{UniFFI, UniFFISwift};
use anyhow::{anyhow, Context, Result};
use camino::Utf8PathBuf;
use cargo_metadata::Metadata;

#[derive(Debug)]
pub struct Configuration {
    pub dir: Utf8PathBuf,
    pub cargo_package_name: String,
    pub cargo_lib_target_name: String,
    pub udl_file: Utf8PathBuf,
    /// the name of the produced package
    pub package_name: String,
    pub release: bool,
    pub profile: String,
    pub manifest_path: Utf8PathBuf,
    pub swift: UniFFISwift,
}

impl Configuration {
    pub fn load(cli: Cli) -> Result<Self> {
        let manifest_path = cli
            .manifest_path
            .to_owned()
            .unwrap_or_else(|| Utf8PathBuf::from("Cargo.toml"))
            .resolve_home_dir()
            .context(format!("manifest_path: {:?}", &cli.manifest_path))?;

        let metadata = Metadata::load_cleaned(&manifest_path)?;
        let Some(package) = metadata.root_package() else {
            anyhow::bail!("Could not find root package in metadata");
        };

        let lib_name = package
            .targets
            .iter()
            .find(|t| t.kind.contains(&"staticlib".to_string()))
            .ok_or_else(|| anyhow!("Could not find a staticlib target"))?
            .name
            .clone();

        let profile = if cli.release { "release" } else { "debug" };
        let mut dir = package.manifest_path.clone();
        dir.pop();

        let UniFFI { swift, udl_file } = UniFFI::parse(&package.metadata)?;
        let package_name = udl_file.file_stem().unwrap().to_string();
        Ok(Self {
            dir,
            cargo_package_name: package.name.clone(),
            cargo_lib_target_name: lib_name,
            udl_file,
            package_name,
            profile: profile.to_string(),
            release: cli.release,
            manifest_path,
            swift,
        })
    }
}
