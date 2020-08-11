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
                      checksum: "029b5f064648e1c9248c64185cec6c728f896ebbf051bc2470e547d4b0215a4c")
    ]
)
