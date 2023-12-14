#!/bin/sh

set -eo pipefail

echo "Building anoncreds library..."
anoncreds/build-kotlin-library.sh
echo "Building askar library..."
askar/build-kotlin-library.sh
echo "Building indy-vdr library..."
indy-vdr/build-kotlin-library.sh
