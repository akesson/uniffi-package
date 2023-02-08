use crate::conf::Configuration;
use anyhow::Result;

pub fn run(conf: &Configuration) -> Result<()> {
    create_package_swift(conf)?;
    Ok(())
}

fn create_package_swift(conf: &Configuration) -> Result<()> {
    let name = &conf.module_name;
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
			dependencies: ["{name}"],
			cxxSettings: [.headerSearchPath("Headers")]
		)
	]
)
"###
    );

    fs_err::write(format!("{name}.package/Package.swift"), contents)?;
    Ok(())
}
