#!/bin/bash
set -e

if [ $# -ne 1 ]; then
  echo "Usage: $0 <config_name>"
  exit 1
fi

CONFIG_NAME="$1"
ROOT_DIR="$(pwd)"
WOKWI_DIR="$ROOT_DIR/wokwi"
TARGET_FILE="$ROOT_DIR/wokwi.toml"

if [ ! -d "$WOKWI_DIR" ]; then
  echo "Error: 'wokwi' directory not found in current directory ($ROOT_DIR)."
  exit 2
fi

if [ ! -f "$TARGET_FILE" ]; then
  echo "Error: 'wokwi.toml' file not found in current directory ($ROOT_DIR)."
  exit 3
fi

# Find the config file in wokwi dir ignoring case
CONFIG_FILE=$(find "$WOKWI_DIR" -maxdepth 1 -type f -iname "${CONFIG_NAME}.toml" | head -n 1)

if [ -z "$CONFIG_FILE" ]; then
  echo "Error: Config '$CONFIG_NAME' not found in $WOKWI_DIR"
  exit 4
fi

echo "Using config: $CONFIG_FILE"
echo "Copying to: $TARGET_FILE"

cp "$CONFIG_FILE" "$TARGET_FILE"

echo "Done."
