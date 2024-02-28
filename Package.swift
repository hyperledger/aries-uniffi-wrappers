// swift-tools-version: 5.7
import PackageDescription
import class Foundation.ProcessInfo

var package = Package(
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
            url: "https://github.com/hyperledger/aries-uniffi-wrappers/releases/download/0.2.0-binary/anoncreds_uniffiFFI.xcframework.zip",
            checksum: "ae96ac8bbef2b9d116b641f38bc3120e1e9666dae7333fe3a982d2a81ac34f2a"),
        .target(
            name: "Askar",
            path: "swift/Sources/Askar"),
        .testTarget(
            name: "AskarTests",
            dependencies: ["Askar"],
            path: "swift/Tests/AskarTests"),
        .binaryTarget(
            name: "askar_uniffiFFI",
            url: "https://github.com/hyperledger/aries-uniffi-wrappers/releases/download/0.2.0-binary/askar_uniffiFFI.xcframework.zip",
            checksum: "c9e7582cecc2658633db1a26b13f372f69d468e079bd8deef12d8db9b5bb91d9"),
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
            url: "https://github.com/hyperledger/aries-uniffi-wrappers/releases/download/0.2.0-binary/indy_vdr_uniffiFFI.xcframework.zip",
            checksum: "c158a04e5300bbb4fd20124a11e7823e512d220e869dd13a2ab2e6435bbadc29")
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
    package.targets.append(.binaryTarget(
        name: "anoncreds_uniffiFFI_local",
        path: "anoncreds/out/anoncreds_uniffiFFI.xcframework"))
    package.targets.append(.binaryTarget(
        name: "askar_uniffiFFI_local",
        path: "askar/out/askar_uniffiFFI.xcframework"))
    package.targets.append(.binaryTarget(
        name: "indy_vdr_uniffiFFI_local",
        path: "indy-vdr/out/indy_vdr_uniffiFFI.xcframework"))

    anoncredsTarget?.dependencies.append("anoncreds_uniffiFFI_local")
    askarTarget?.dependencies.append("askar_uniffiFFI_local")
    indyVdrTarget?.dependencies.append("indy_vdr_uniffiFFI_local")
}
