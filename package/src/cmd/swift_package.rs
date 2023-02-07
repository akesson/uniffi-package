use crate::conf::Configuration;
use anyhow::Result;

pub fn run(conf: &Configuration) -> Result<()> {
    fs_err::create_dir_all(format!("target/uniffi/{}", &conf.package_name))?;
    create_package_swift(conf)?;
    Ok(())
}

fn create_package_swift(conf: &Configuration) -> Result<()> {
    let name = &conf.package_name;
    let contents = format!(
        r###"// swift-tools-version:5.7
import PackageDescription
let package = Package(
	name: "{name}",
	products: [
		.library(
			name: "{name}",
			targets: ["{name}Lib"]),
	],
	dependencies: [],
	targets: [
		.binaryTarget(
			name: "{name}",
			path: "{name}.xcframework"
		),
		.target(
			name: "{name}Lib",
			dependencies: ["{name}"]
		)
	]
)
"###
    );

    fs_err::write(format!("target/uniffi/{name}/Package.swift"), contents)?;
    Ok(())
}
