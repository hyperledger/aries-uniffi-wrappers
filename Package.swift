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
            url: "https://github.com/hyperledger/aries-uniffi-wrappers/releases/download/0.1.0-prepare/anoncreds_uniffiFFI.xcframework.zip",
            checksum: "fcde55bcdfa895f180066788e5b2447b1bc94a4eb56fd23fd199d0e2b9a851a5"),
        .target(
            name: "Askar",
            dependencies: ["askar_uniffiFFI"]),
        .testTarget(
            name: "AskarTests",
            dependencies: ["Askar"]),
        .binaryTarget(
            name: "askar_uniffiFFI",
            url: "https://github.com/hyperledger/aries-uniffi-wrappers/releases/download/0.1.0-prepare/askar_uniffiFFI.xcframework.zip",
            checksum: "82e61bb8bd1c645d1c027d67ea627d92631f68d682b4deb57218e78ead78ff44"),
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
            url: "https://github.com/hyperledger/aries-uniffi-wrappers/releases/download/0.1.0-prepare/indy_vdr_uniffiFFI.xcframework.zip",
            checksum: "bf1981bb854bb565a6803f917e67bf9238ed798cecbed9ffdf247cbc118b6f55"),
    ]
)
