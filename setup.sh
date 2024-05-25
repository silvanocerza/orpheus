#!/usr/bin/env bash
set -e

# Get the project dir
PROJECT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

SERVICE_DIR="$HOME/.local/share/systemd/user/"
INSTALL_DIR="$HOME/.local/bin/"

# Build the project in release mode
cargo build -r

# Create the necessary folders
mkdir -p "$INSTALL_DIR"
mkdir -p "$SERVICE_DIR"

# Copy the systemd service and replace the binary path
sed "s|BIN_PATH|$INSTALL_DIR|g" "$PROJECT_DIR/orpheus.service" > "$SERVICE_DIR/orpheus.service"

# Copy the executable
cp "$PROJECT_DIR/target/release/orpheus" "$INSTALL_DIR"

# Enable and start the service
systemctl --user enable orpheus.service
systemctl --user start orpheus.service

