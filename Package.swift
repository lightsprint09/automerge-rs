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
                      url: "https://github.com/lightsprint09/automerge-rs/releases/download/v0.0.3/automerge-rs-swift-package.zip",
                      checksum: "ef58f8ffa0733103c859afccd2635597a60f64fcf35bf1912751b954da4851ab")
    ]
)
