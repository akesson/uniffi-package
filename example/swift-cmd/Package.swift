// swift-tools-version: 5.7
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "swift-cmd",
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        // .package(url: /* package url */, from: "1.0.0"),
        .package(path: "../rust-lib/Mobile.package")
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .executableTarget(
            name: "swift-cmd",
            dependencies: [
                .product(name: "Mobile", package: "Mobile.package")
            ]),
        .testTarget(
            name: "swift-cmdTests",
            dependencies: ["swift-cmd"]),
    ]
)
