#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
APP_DIR="$(dirname "$SCRIPT_DIR")"

# Path to the private key
PRIVATE_KEY_PATH="$APP_DIR/tauri-private.key"

if [ ! -f "$PRIVATE_KEY_PATH" ]; then
    echo "Error: Private key not found at $PRIVATE_KEY_PATH"
    echo "Please ensure you have generated the keys in the app directory."
    exit 1
fi

echo "Loading signing key from $PRIVATE_KEY_PATH"

# Load the private key into the environment variable
# We read the content and export it
export TAURI_SIGNING_PRIVATE_KEY=$(cat "$PRIVATE_KEY_PATH")

# Run the build
cd "$APP_DIR"
echo "Starting Tauri build..."
yarn tauri build "$@"
