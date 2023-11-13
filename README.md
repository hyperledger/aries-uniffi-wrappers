# Aries uniffi wrappers

This repository contains the wrappers for the following libraries:
- Aries Askar: https://github.com/hyperledger/aries-askar
- AnonCreds Rust: https://github.com/hyperledger/anoncreds-rs
- Indy VDR: https://github.com/hyperledger/indy-vdr

The wrappers are generated using [UniFFI](https://github.com/mozilla/uniffi-rs).
UniFFI can generate several language bindings including Swift, Kotlin, and Python.
Only the Swift bindings are currently supported.

## Usage

The Swift wrappers are distributed as a Swift Package.
To use the wrappers in your project, add the following dependency to your `Package.swift`:
```swift
    .package(url: "https://github.com/hyperledger/aries-uniffi-wrappers", from: "0.1.0"),
```

And add the following dependency to your target:
```swift
    .target(
        name: "MyTarget",
        dependencies: [
            .product(name: "AriesAskar", package: "aries-uniffi-wrappers"),
            .product(name: "AnonCreds", package: "aries-uniffi-wrappers"),
            .product(name: "IndyVdr", package: "aries-uniffi-wrappers"),
        ]
    ),
```

Take a look at the tests in `Tests` folder for examples on how to use the wrappers.

## Contributing

Pull requests are welcome! We enforce [developer certificate of origin](https://developercertificate.org/) (DCO) commit signing. See guidance [here](https://github.com/apps/dco).

## License

Aries uniffi wrappers are licensed under the [Apache License 2.0](LICENSE).
