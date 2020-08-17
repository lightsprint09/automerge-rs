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
                      checksum: "3689f084c59358a4e069c9e10d1d81291a442fbe848bc1eaee46742119f5f730")
    ]
)
