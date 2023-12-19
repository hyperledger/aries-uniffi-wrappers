#!/bin/sh

set -eo pipefail

pushd `dirname $0`
trap popd EXIT

NAME="askar_uniffi"
BUNDLE_IDENTIFIER="org.hyperledger.$NAME"
LIBRARY_NAME="lib$NAME.a"
OUT_PATH="out/kmpp-uniffi"
WRAPPER_PATH="../Sources/Askar"
AARCH64_APPLE_DARWIN_PATH="./target/aarch64-apple-darwin/release"
X86_64_APPLE_DARWIN_PATH="./target/x86_64-apple-darwin/release"

apple_targets=("aarch64-apple-ios" "aarch64-apple-ios-sim" "x86_64-apple-ios" "aarch64-apple-darwin" "x86_64-apple-darwin")

android_targets=("aarch64-linux-android" "armv7-linux-androideabi" "i686-linux-android" "x86_64-linux-android")
android_jni=("arm64-v8a" "armeabi-v7a" "x86" "x86_64")

# Build for apple targets
for target in "${apple_targets[@]}"; do
  echo "Building for $target..."
  rustup target add $target
  cargo build --release --target $target
done

# Merge mac libraries with lipo
mkdir -p $OUT_PATH/macos-native/static
mkdir -p $OUT_PATH/macos-native/dynamic

# dylib for JVM
lipo -create $AARCH64_APPLE_DARWIN_PATH/lib$NAME.dylib \
             $X86_64_APPLE_DARWIN_PATH/lib$NAME.dylib \
     -output $OUT_PATH/macos-native/dynamic/lib$NAME.dylib

# .a for Native
lipo -create $AARCH64_APPLE_DARWIN_PATH/lib$NAME.a \
             $X86_64_APPLE_DARWIN_PATH/lib$NAME.a \
     -output $OUT_PATH/macos-native/static/lib$NAME.a

cargo install cross --git https://github.com/cross-rs/cross

# Build for android targets
for target in "${android_targets[@]}"; do
  echo "Building for $target..."
  rustup toolchain install 1.65.0 --target $target
  cross build --release --target $target
done

# Create JNI Libs folder
for key in "${!android_targets[@]}"; do
  mkdir -p $OUT_PATH/jniLibs/${android_jni[$key]}
  cp ./target/${android_targets[$key]}/release/lib$NAME.so $OUT_PATH/jniLibs/${android_jni[$key]}/lib$NAME.so || echo ""
  echo "${android_targets[$key]}: ${android_jni[$key]}"
done

# Generate wrapper
echo "Generating wrapper..."
mkdir -p $OUT_PATH
cargo install --bin uniffi-bindgen-kotlin-multiplatform uniffi_bindgen_kotlin_multiplatform@0.1.0
CURRENT_ARCH=$(rustc --version --verbose | grep host | cut -f2 -d' ')
uniffi-bindgen-kotlin-multiplatform --lib-file ./target/$CURRENT_ARCH/release/$LIBRARY_NAME --out-dir $OUT_PATH uniffi/askar.udl
