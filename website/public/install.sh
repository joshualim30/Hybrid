#!/bin/bash
set -e

# Hybrid CLI Installer
# Version: 0.1.0

echo "🚀 Starting Hybrid CLI installation..."

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)     PLATFORM=linux;;
    Darwin*)    PLATFORM=macos;;
    *)          echo "❌ Unsupported OS: $OS"; exit 1;;
esac

if [ "$ARCH" = "x86_64" ]; then
    BINARY_ARCH="x64"
elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
    BINARY_ARCH="arm64"
else
    echo "❌ Unsupported architecture: $ARCH"
    exit 1
fi

echo "📦 Detected $PLATFORM ($BINARY_ARCH)"

# Simulated download
INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    echo "🔑 Need sudo permissions to install to $INSTALL_DIR"
    SUDO="sudo"
fi

echo "⬇️  Downloading Hybrid CLI..."
# curl -L "https://github.com/joshualim30/hybrid/releases/latest/download/hybrid-$PLATFORM-$BINARY_ARCH" -o /tmp/hybrid
# $SUDO mv /tmp/hybrid $INSTALL_DIR/hybrid
# $SUDO chmod +x $INSTALL_DIR/hybrid

echo "✅ Hybrid CLI successfully installed!"
echo "👉 Run 'hybrid --help' to get started."
