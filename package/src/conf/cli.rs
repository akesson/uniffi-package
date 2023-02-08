use camino::Utf8PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Cli {
    /// Build artifacts in release mode, with optimizations.
    #[arg(short, long)]
    pub release: bool,

    /// Path to Cargo.toml.
    #[arg(long)]
    pub manifest_path: Option<Utf8PathBuf>,
}
