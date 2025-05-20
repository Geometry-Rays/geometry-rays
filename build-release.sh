#!/bin/bash
set -euo pipefail

echo Deleting old builds
cargo clean

echo Building for Linux
cargo build --target x86_64-unknown-linux-gnu --release

echo Building for Windows
cargo build --target x86_64-pc-windows-gnu --release

echo Creating folders
rm -rf Geometry-Rays
mkdir Geometry-Rays

echo Moving executables
mv ./target/x86_64-unknown-linux-gnu/release/geometry-rays-fyre ./Geometry-Rays
mv ./target/x86_64-pc-windows-gnu/release/geometry-rays-fyre.exe ./Geometry-Rays

echo Copying required folders
cp -r ./Resources ./Geometry-Rays
cp -r ./mods ./Geometry-Rays
cp -r ./save-data ./Geometry-Rays

echo Zipping the package
7z a -tzip Geometry-Rays.zip Geometry-Rays

echo Removing temporary directory
rm -rf ./Geometry-Rays
