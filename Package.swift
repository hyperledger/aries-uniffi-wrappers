// swift-tools-version: 5.7
import PackageDescription

let package = Package(
    name: "aries-uniffi-wrappers",
    platforms: [
        .macOS(.v10_15),
        .iOS(.v15),
    ],
    products: [
        .library(
            name: "Anoncreds",
            targets: ["Anoncreds"]),
        .library(
            name: "Askar",
            targets: ["Askar"]),
        .library(
            name: "IndyVdr",
            targets: ["IndyVdr"]),
    ],
    dependencies: [
    ],
    targets: [
        .target(
            name: "Anoncreds",
            dependencies: ["anoncreds_uniffiFFI"]),
        .testTarget(
            name: "AnoncredsTests",
            dependencies: ["Anoncreds"]),
        .binaryTarget(
            name: "anoncreds_uniffiFFI",
            url: "https://github.com/hyperledger/aries-uniffi-wrappers/releases/download/0.1.1-binary/anoncreds_uniffiFFI.xcframework.zip",
            checksum: "41f8d517d89f57ca28598d936b8743c8e378ef929a58ac00bdbf77dffe1e19b7"),
        .target(
            name: "Askar",
            dependencies: ["askar_uniffiFFI"]),
        .testTarget(
            name: "AskarTests",
            dependencies: ["Askar"]),
        .binaryTarget(
            name: "askar_uniffiFFI",
            url: "https://github.com/hyperledger/aries-uniffi-wrappers/releases/download/0.1.1-binary/askar_uniffiFFI.xcframework.zip",
            checksum: "761220bc486d14c371c84c30481c7b07b4e8dc53a0e61cdd3e67efa842bc73ab"),
        .target(
            name: "IndyVdr",
            dependencies: ["indy_vdr_uniffiFFI"]),
        .testTarget(
            name: "IndyVdrTests",
            dependencies: ["IndyVdr"],
            resources: [
                .copy("resources/genesis_sov_buildernet.txn")
            ]),
        .binaryTarget(
            name: "indy_vdr_uniffiFFI",
            url: "https://github.com/hyperledger/aries-uniffi-wrappers/releases/download/0.1.1-binary/indy_vdr_uniffiFFI.xcframework.zip",
            checksum: "076821ffbeb291e2541dbe8eb6186e2583cbadd68cde144c999cf0362fe9a19c"),
    ]
)
