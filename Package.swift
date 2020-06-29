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
                      checksum: "b86e04e80597222c0a7fd5a9a8a67ba518c502f065c8d5013596dcfc9abf7beb")
    ]
)
