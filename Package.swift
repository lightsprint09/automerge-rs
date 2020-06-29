// swift-tools-version:5.3

import PackageDescription

let package = Package(
    name: "AutomergeRSBackend",
    products: [
        .library(name: "AutomergeRSBackend", targets: ["AutomergeRSBackend"])
    ],
    dependencies: [
    ],
    targets: [
        .binaryTarget(name: "AutomergeRSBackend",
                      url: "https://github.com/lightsprint09/automerge-rs/releases/download/v0.0.3/AutomergeRSBackend.xcframework.zip",
                      checksum: "25cadcbfac65002af96fca00a8e53f675b02af4cb980c105372b090eb4716c07")
    ]
)
