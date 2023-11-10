#!/bin/sh

set -eo pipefail

echo "Building anoncreds framework..."
anoncreds/build-swift-framework.sh
echo "Building askar framework..."
askar/build-swift-framework.sh
echo "Building indy-vdr framework..."
indy-vdr/build-swift-framework.sh
