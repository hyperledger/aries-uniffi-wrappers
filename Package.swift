// swift-tools-version: 5.7
import PackageDescription
import class Foundation.ProcessInfo

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
            path: "swift/Sources/Anoncreds"),
        .testTarget(
            name: "AnoncredsTests",
            dependencies: ["Anoncreds"],
            path: "swift/Tests/AnoncredsTests"),
        .binaryTarget(
            name: "anoncreds_uniffiFFI",
            url: "https://github.com/hyperledger/aries-uniffi-wrappers/releases/download/0.1.1-binary/anoncreds_uniffiFFI.xcframework.zip",
            checksum: "41f8d517d89f57ca28598d936b8743c8e378ef929a58ac00bdbf77dffe1e19b7"),
        .binaryTarget(
            name: "anoncreds_uniffiFFI_local",
            path: "anoncreds/out/anoncreds_uniffiFFI.xcframework"),
        .target(
            name: "Askar",
            path: "swift/Sources/Askar"),
        .testTarget(
            name: "AskarTests",
            dependencies: ["Askar"],
            path: "swift/Tests/AskarTests"),
        .binaryTarget(
            name: "askar_uniffiFFI",
            url: "https://github.com/hyperledger/aries-uniffi-wrappers/releases/download/0.1.1-binary/askar_uniffiFFI.xcframework.zip",
            checksum: "761220bc486d14c371c84c30481c7b07b4e8dc53a0e61cdd3e67efa842bc73ab"),
        .binaryTarget(
            name: "askar_uniffiFFI_local",
            path: "askar/out/askar_uniffiFFI.xcframework"),
        .target(
            name: "IndyVdr",
            path: "swift/Sources/IndyVdr"),
        .testTarget(
            name: "IndyVdrTests",
            dependencies: ["IndyVdr"],
            path: "swift/Tests/IndyVdrTests",
            resources: [
                .copy("resources/genesis_sov_buildernet.txn")
            ]),
        .binaryTarget(
            name: "indy_vdr_uniffiFFI",
            url: "https://github.com/hyperledger/aries-uniffi-wrappers/releases/download/0.1.1-binary/indy_vdr_uniffiFFI.xcframework.zip",
            checksum: "076821ffbeb291e2541dbe8eb6186e2583cbadd68cde144c999cf0362fe9a19c"),
        .binaryTarget(
            name: "indy_vdr_uniffiFFI_local",
            path: "indy-vdr/out/indy_vdr_uniffiFFI.xcframework"),
    ]
)

let anoncredsTarget = package.targets.first(where: { $0.name == "Anoncreds" })
let askarTarget = package.targets.first(where: { $0.name == "Askar" })
let indyVdrTarget = package.targets.first(where: { $0.name == "IndyVdr" })

if ProcessInfo.processInfo.environment["USE_LOCAL_XCFRAMEWORK"] == nil {
    anoncredsTarget?.dependencies.append("anoncreds_uniffiFFI")
    askarTarget?.dependencies.append("askar_uniffiFFI")
    indyVdrTarget?.dependencies.append("indy_vdr_uniffiFFI")
} else {
    anoncredsTarget?.dependencies.append("anoncreds_uniffiFFI_local")
    askarTarget?.dependencies.append("askar_uniffiFFI_local")
    indyVdrTarget?.dependencies.append("indy_vdr_uniffiFFI_local")
}
