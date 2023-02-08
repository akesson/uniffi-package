// swift-tools-version:5.7
import PackageDescription
let package = Package(
	name: "Mobile",
	products: [
		.library(
			name: "Mobile",
			targets: ["MobileLib"]),
	],
	dependencies: [],
	targets: [
		.binaryTarget(
			name: "Mobile",
			path: "Mobile.xcframework"
		),
		.target(
			name: "MobileLib",
			dependencies: ["Mobile"],
			cxxSettings: [.headerSearchPath("Headers")]
		)
	]
)
