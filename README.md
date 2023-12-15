# Aries uniffi wrappers

This repository contains the wrappers for the following libraries:
- Aries Askar: https://github.com/hyperledger/aries-askar
- AnonCreds Rust: https://github.com/hyperledger/anoncreds-rs
- Indy VDR: https://github.com/hyperledger/indy-vdr

The wrappers are generated using [UniFFI](https://github.com/mozilla/uniffi-rs).
UniFFI can generate several language bindings including Swift, Kotlin, and Python.
Kotlin wrappers are generated with [uniffi-kotlin-multiplatform-bindings](https://gitlab.com/trixnity/uniffi-kotlin-multiplatform-bindings).

## Usage

### Swift

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

### Kotlin

The Kotlin wrappers are set up to be distributed as a Maven package. In the future we aim to have releases on github packages. For now you will either have to use Maven Local or fork this repository and publish it yourself.

#### Generate Kotlin bindings

First step before publishing will be to generate the bindings. To generate all bindings for Kotlin just run the `build-kotlin-libraries.sh` file from root. You can also generate bindings for a specific library by running the `build-kotlin-libraries.sh` found inside of target library's directory. (ie: `/askar/build-kotlin-library.sh`)

#### Publish Kotlin libraries with Maven Local

Once bindings have been generated we can now publish to Maven Local. To do so we'll cd into the target kotlin wrapper (ie: `/kotlin/askar/`) and then we will run `./gradlew publishToMavenLocal`.

To use Maven Local in a seperate project you'll want to make sure to add it inside of your `build.gradle.kts`
```kotlin
    repositories {
        mavenLocal()
        mavenCentral()
        google()
    }
```

#### Publish Kotlin libraries to github

Publishing to github will require you to do a couple more steps. First we will want to get a github token with `write:packages` permissions, more details can be [found here](https://docs.github.com/en/packages/learn-github-packages/about-permissions-for-github-packages#about-scopes-and-permissions-for-package-registries)

Once you have a token we will want to add both your token and your github username into `kotlin/${library}/local.properties` like so:
```
githubUsername=ExampleUsername
githubToken=ghp_ajsldk1FakeTokenjkash
```
Next you'll want to make sure you are targetting the correct URL for publishing. Make sure that the `setUrl` function in `kotlin/${library}/build.gradle.kts` targets your fork's URL instead of the main repository's.

```kotlin
    publishing{
        repositories{
            maven{
                name = "github"
                setUrl("https://maven.pkg.github.com/${FORK_DIRECTORY}/aries-uniffi-wrappers")
                credentials {
                    username = getExtraString("githubUsername")
                    password = getExtraString("githubToken")
                }
            }
        }
        ...
    }
```

Now we can publish to github packages. You'll want to call `./gradlew publishAllPublicationsToGithubRepository` inside of the target wrapper root directory.

To add a github packages repository in a seperate project you will have to have a github token with `read:packages` permissions. Then you will add the repository to your `build.gradle.kts` like so:
```kotlin
    repositories {
        mavenCentral()
        google()
        maven {
            setUrl("https://maven.pkg.github.com/${FORK_DIRECTORY}/aries-uniffi-wrappers")
            credentials {
                username = getExtraString("githubUsername")
                password = getExtraString("githubToken")
            }
        }
    }
```

#### Adding as a dependency

Now all we have to do is add the libraries as a dependency in your `build.gradle.kts` like so:
```kotlin
    dependencies {
        implementation("org.hyperledger:anoncreds_uniffi:0.1.0-wrapper.1")
        implementation("org.hyperledger:indy_vdr_uniffi:0.1.0-wrapper.1")
        implementation("org.hyperledger:askar_uniffi:0.1.0-wrapper.1")
    }
```

## Contributing

Pull requests are welcome! We enforce [developer certificate of origin](https://developercertificate.org/) (DCO) commit signing. See guidance [here](https://github.com/apps/dco).

## License

Aries uniffi wrappers are licensed under the [Apache License 2.0](LICENSE).
