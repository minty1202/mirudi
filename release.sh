#!/bin/bash
set -e

version=$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].version')
notes=$1

if [[ -z "$notes" ]]; then
  echo "使い方: bash release.sh \"リリースノート内容\""
  exit 1
fi

echo "Building binary..."
cargo build --release

echo "Releasing $version..."
gh release create "$version" \
  --title "$version" \
  --notes "$notes" \
  target/release/mirudi

echo "Release $version created!"
