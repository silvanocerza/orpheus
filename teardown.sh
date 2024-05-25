#!/usr/bin/env bash
set -e

SERVICE_DIR="$HOME/.local/share/systemd/user/"
INSTALL_DIR="$HOME/.local/bin/"

# Stop and disable the service
systemctl --user stop orpheus.service
systemctl --user disable orpheus.service

# Remove the service definition
rm "$SERVICE_DIR/orpheus.service"

# Remove the executable
rm "$INSTALL_DIR/orpheus"


