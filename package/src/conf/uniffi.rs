use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct UniFFI {
    #[serde(default)]
    pub swift: UniFFISwift,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct UniFFISwift {
    #[serde(default)]
    pub release_optimize_std_lib: bool,
}

impl UniFFI {
    pub fn parse(metadata: &serde_json::Value) -> Result<Self> {
        Ok(serde_json::from_value(metadata.clone())?)
    }
}
