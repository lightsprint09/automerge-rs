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
        .binaryTarget(name: "AutomergeRSBackend", path: "./AutomergeRSBackend.xcframework")
    ]
)
