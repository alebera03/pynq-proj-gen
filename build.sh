#!/bin/bash

# Build essentials
sudo apt update -y && sudo apt upgrade -y && sudo apt install build-essential -y

# Capture the starting directory
ORIGINAL_DIR=$(pwd)
INSTALL_DIR="/opt/pz2"

# 1. Download/Update repository
if [ -d "pynq-proj-gen" ]; then
    echo "Directory exists, pulling latest changes..."
    cd pynq-proj-gen && git pull origin master
else
    git clone https://github.com/AleBera03/pynq-proj-gen
    cd pynq-proj-gen
fi

# 2. Build the Rust project
echo "Building project with Cargo..."
cargo build -r

# 3. Setup /opt/pz2 structure
echo "Setting up $INSTALL_DIR..."
sudo rm -rf "$INSTALL_DIR"
sudo mkdir -p "$INSTALL_DIR/bin"
sudo mkdir -p "$INSTALL_DIR/.scripts"

# 4. Copy bin file and scripts
sudo cp ./target/release/pz2 "$INSTALL_DIR/bin/" -v
sudo cp -r ./.scripts/* "$INSTALL_DIR/.scripts/" -v

# 5. Set permissions
sudo chmod -R 755 "$INSTALL_DIR"

# 6. Add to PATH safely (only if not already there)
PATH_LINE="export PATH=\$PATH:$INSTALL_DIR/bin"
if ! grep -qF "$PATH_LINE" ~/.bashrc; then
    echo "Adding $INSTALL_DIR/bin to PATH in .bashrc"
    echo "$PATH_LINE" >> ~/.bashrc
else
    echo "Path already exists in .bashrc"
fi

# Return to original directory
cd "$ORIGINAL_DIR"
rm -rf pynq-proj-gen

echo "Installation complete. Please run 'source ~/.bashrc' or restart your terminal."