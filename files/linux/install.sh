#!/usr/bin/env bash

INSTALLER_SCRIPT_URL="https://raw.githubusercontent.com/azomDev/pen/main/files/linux/install"
TMP_DIR="/tmp"
INSTALLER_PATH="$TMP_DIR/install"

curl -o "$TMP_DIR/install" "$INSTALLER_SCRIPT_URL"
chmod +x "$TMP_DIR/install"
"$INSTALLER_PATH"