#!/bin/sh

set -eo pipefail

pushd `dirname $0`
trap popd EXIT

NAME="anoncreds_uniffi"
VERSION=${1:-"1.0"} # first arg or "1.0"
BUNDLE_IDENTIFIER="org.hyperledger.$NAME"
LIBRARY_NAME="lib$NAME.a"
FRAMEWORK_LIBRARY_NAME=${NAME}FFI
FRAMEWORK_NAME="$FRAMEWORK_LIBRARY_NAME.framework"
XC_FRAMEWORK_NAME="$FRAMEWORK_LIBRARY_NAME.xcframework"
HEADER_NAME="${NAME}FFI.h"
OUT_PATH="out/kmpp-uniffi"
MIN_IOS_VERSION="15.0"
WRAPPER_PATH="../Sources/Anoncreds"

AARCH64_APPLE_IOS_PATH="./target/aarch64-apple-ios/release"
AARCH64_APPLE_IOS_SIM_PATH="./target/aarch64-apple-ios-sim/release"
X86_64_APPLE_IOS_PATH="./target/x86_64-apple-ios/release"
AARCH64_APPLE_DARWIN_PATH="./target/aarch64-apple-darwin/release"
X86_64_APPLE_DARWIN_PATH="./target/x86_64-apple-darwin/release"

apple_targets=("aarch64-apple-ios" "aarch64-apple-ios-sim" "x86_64-apple-ios" "aarch64-apple-darwin" "x86_64-apple-darwin")
apple_jni=("ios-aarch64" "ios-aarch64" "ios-x86-64" "darwin-aarch64" "darwin-x86-64")


android_targets=("aarch64-linux-android" "armv7-linux-androideabi" "i686-linux-android" "x86_64-linux-android")
android_jni=("arm64-v8a" "armeabi-v7a" "x86" "x86_64")

# Build for apple targets
for target in "${apple_targets[@]}"; do
  echo "Building for $target..."
  rustup target add $target
  cargo build --release --target $target
done

# # Merge libraries with lipo
mkdir -p $OUT_PATH/macos-native/
lipo -create $AARCH64_APPLE_DARWIN_PATH/lib$NAME.dylib \
             $X86_64_APPLE_DARWIN_PATH/lib$NAME.dylib \
     -output $OUT_PATH/macos-native/lib$NAME.dylib

cargo install cross --git https://github.com/cross-rs/cross

# Build for android targets
for target in "${android_targets[@]}"; do
  echo "Building for $target..."
  rustup toolchain install 1.65.0 --target $target
  cross build --release --target $target
done

# for key in "${!apple_targets[@]}"; do
#   mkdir -p $OUT_PATH/jniLibs/${apple_jni[$key]}
#   cp ./target/${apple_targets[$key]}/release/lib$NAME.a $OUT_PATH/jniLibs/${apple_jni[$key]}/lib$NAME.a || echo ""
#   cp ./target/${apple_targets[$key]}/release/lib$NAME.dylib $OUT_PATH/jniLibs/${apple_jni[$key]}/lib$NAME.dylib || echo ""
#   echo "${apple_targets[$key]}: ${apple_jni[$key]}"
# done

# Create JNI Libs folder
for key in "${!android_targets[@]}"; do
  mkdir -p $OUT_PATH/jniLibs/${android_jni[$key]}
  cp ./target/${android_targets[$key]}/release/lib$NAME.so $OUT_PATH/jniLibs/${android_jni[$key]}/lib$NAME.so || echo ""
  echo "${android_targets[$key]}: ${android_jni[$key]}"
done

# Generate wrapper
echo "Generating wrapper..."
mkdir -p $OUT_PATH
mkdir -p $WRAPPER_PATH
cargo install --bin uniffi-bindgen-kotlin-multiplatform uniffi_bindgen_kotlin_multiplatform@0.1.0
CURRENT_ARCH=$(rustc --version --verbose | grep host | cut -f2 -d' ')
uniffi-bindgen-kotlin-multiplatform --lib-file ./target/$CURRENT_ARCH/release/$LIBRARY_NAME --out-dir $OUT_PATH uniffi/anoncreds_uniffi.udl


# # Create framework template
# rm -rf $OUT_PATH/$FRAMEWORK_NAME
# mkdir -p $OUT_PATH/$FRAMEWORK_NAME/Headers
# mkdir -p $OUT_PATH/$FRAMEWORK_NAME/Modules
# cp $OUT_PATH/$HEADER_NAME $OUT_PATH/$FRAMEWORK_NAME/Headers
# cat <<EOT > $OUT_PATH/$FRAMEWORK_NAME/Modules/module.modulemap
# framework module $FRAMEWORK_LIBRARY_NAME {
#   umbrella header "$HEADER_NAME"

#   export *
#   module * { export * }
# }
# EOT

# cat <<EOT > $OUT_PATH/$FRAMEWORK_NAME/Info.plist
# <?xml version="1.0" encoding="UTF-8"?>
# <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
# <plist version="1.0">
# <dict>
# 	<key>CFBundleDevelopmentRegion</key>
# 	<string>en</string>
# 	<key>CFBundleExecutable</key>
# 	<string>$FRAMEWORK_LIBRARY_NAME</string>
# 	<key>CFBundleIdentifier</key>
# 	<string>$BUNDLE_IDENTIFIER</string>
# 	<key>CFBundleInfoDictionaryVersion</key>
# 	<string>6.0</string>
# 	<key>CFBundleName</key>
# 	<string>$FRAMEWORK_LIBRARY_NAME</string>
# 	<key>CFBundlePackageType</key>
# 	<string>FMWK</string>
# 	<key>CFBundleShortVersionString</key>
# 	<string>1.0</string>
# 	<key>CFBundleVersion</key>
# 	<string>$VERSION</string>
# 	<key>NSPrincipalClass</key>
# 	<string></string>
# 	<key>MinimumOSVersion</key>
# 	<string>$MIN_IOS_VERSION</string>
# </dict>
# </plist>
# EOT

# # Prepare frameworks for each platform
# rm -rf $OUT_PATH/frameworks
# mkdir -p $OUT_PATH/frameworks/sim
# mkdir -p $OUT_PATH/frameworks/ios
# mkdir -p $OUT_PATH/frameworks/macos
# cp -r $OUT_PATH/$FRAMEWORK_NAME $OUT_PATH/frameworks/sim/
# cp -r $OUT_PATH/$FRAMEWORK_NAME $OUT_PATH/frameworks/ios/
# cp -r $OUT_PATH/$FRAMEWORK_NAME $OUT_PATH/frameworks/macos/
# mv $OUT_PATH/sim-$LIBRARY_NAME $OUT_PATH/frameworks/sim/$FRAMEWORK_NAME/$FRAMEWORK_LIBRARY_NAME
# mv $OUT_PATH/macos-$LIBRARY_NAME $OUT_PATH/frameworks/macos/$FRAMEWORK_NAME/$FRAMEWORK_LIBRARY_NAME
# cp $AARCH64_APPLE_IOS_PATH/$LIBRARY_NAME $OUT_PATH/frameworks/ios/$FRAMEWORK_NAME/$FRAMEWORK_LIBRARY_NAME

# # Create xcframework
# echo "Creating xcframework..."
# rm -rf $OUT_PATH/$XC_FRAMEWORK_NAME
# xcodebuild -create-xcframework \
#     -framework $OUT_PATH/frameworks/sim/$FRAMEWORK_NAME \
#     -framework $OUT_PATH/frameworks/ios/$FRAMEWORK_NAME \
#     -framework $OUT_PATH/frameworks/macos/$FRAMEWORK_NAME \
#     -output $OUT_PATH/$XC_FRAMEWORK_NAME

# # Copy swift wrapper
# # Need some temporary workarounds to compile swift wrapper
# # https://github.com/rust-lang/cargo/issues/11953
# cat <<EOT > $OUT_PATH/import.txt
# #if os(macOS)
# import SystemConfiguration
# #endif
# EOT
# cat $OUT_PATH/import.txt $OUT_PATH/$NAME.swift > $WRAPPER_PATH/$NAME.swift
