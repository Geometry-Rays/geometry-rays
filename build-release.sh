#!/bin/bash
set -euo pipefail

echo Deleting old builds
cargo clean

echo Building...
cargo build --release

echo Creating folders
rm -rf Geometry-Rays
mkdir Geometry-Rays

echo Copying executable
cp ./target/release/geometry-rays-fyre ./Geometry-Rays

echo Copying required folders
cp -r ./Resources ./Geometry-Rays
cp -r ./mods ./Geometry-Rays
cp -r ./save-data ./Geometry-Rays
