#!/bin/bash

# This script prepares the release for the LingCode Rime project.
# It performs the following tasks:
# 1. Clean previous build artifacts.
# 2. Build the project for all target platforms.
# 3. Package the binaries and resources for distribution.

set -e

# Define the output directory for the release
OUTPUT_DIR="release"

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf $OUTPUT_DIR
mkdir $OUTPUT_DIR

# Build for all platforms
echo "Building for all platforms..."
cargo build --release

# Copy binaries to the output directory
echo "Copying binaries to $OUTPUT_DIR..."
cp target/release/* $OUTPUT_DIR/

# Additional packaging steps can be added here

echo "Release preparation completed."