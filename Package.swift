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
            path: "anoncreds/out/anoncreds_uniffiFFI.xcframework"),
        .target(
            name: "Askar",
            dependencies: ["askar_uniffiFFI"]),
        .testTarget(
            name: "AskarTests",
            dependencies: ["Askar"]),
        .binaryTarget(
            name: "askar_uniffiFFI",
            path: "askar/out/askar_uniffiFFI.xcframework"),
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
            path: "indy-vdr/out/indy_vdr_uniffiFFI.xcframework"),
    ]
)
