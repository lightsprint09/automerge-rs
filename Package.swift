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
                      checksum: "d22cc828f854e058960969a375011bcb14edf69827baaff30172fee21c67f848")
    ]
)
