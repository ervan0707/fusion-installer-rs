#!/bin/bash

VERSION="$1"

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

# Remove 'v' prefix if present
VERSION="${VERSION#v}"

# Update version in Cargo.toml
sed -i "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"/version = \"$VERSION\"/" Cargo.toml

# Update Cargo.lock
cargo update --package fusion-installer-rs

echo "Updated version to $VERSION"
