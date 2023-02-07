use crate::{
    ext::{MetadataExt, PathBufExt},
    Cli,
};

use super::{UniFFI, UniFFISwift};
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use cargo_metadata::Metadata;

#[derive(Debug)]
pub struct Configuration {
    pub dir: Utf8PathBuf,
    pub package: String,
    pub release: bool,
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

        let mut dir = package.manifest_path.clone();
        dir.pop();

        let UniFFI { swift } = UniFFI::parse(&package.metadata)?;
        Ok(Self {
            dir,
            package: package.name.clone(),
            release: cli.release,
            manifest_path,
            swift,
        })
    }
}
