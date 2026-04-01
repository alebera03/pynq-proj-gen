#!/bin/bash

REMOTE_IP="192.168.137.75"


sudo apt update -y && sudo apt upgrade -y && sudo apt install build-essential
sudo apt autoremove

# Check ssh configuration
if [[ -z $(ls -l ~/.ssh/id_*) ]]; then
    ssh-keygen -t rsa
fi
if [[ -z $(ssh xilinx@$REMOTE_IP "grep '$(cat ~/.ssh/id_*.pub)' ~/.ssh/authorized_keys") ]]; then
    ssh-copy-id xilinx@$REMOTE_IP
fi

# Capture the starting directory
ORIGINAL_DIR=$(pwd)
INSTALL_DIR="/opt/pz2"

# Download/Update repository
if [ -d "pynq-proj-gen" ]; then
    echo "Directory exists, pulling latest changes..."
    cd pynq-proj-gen && git pull origin master
else
    git clone https://github.com/AleBera03/pynq-proj-gen
    cd pynq-proj-gen
fi

# Build the Rust project
echo "Building project with Cargo..."
cargo build -r

# Setup /opt/pz2 structure
echo "Setting up $INSTALL_DIR..."
sudo rm -rf "$INSTALL_DIR"
sudo mkdir -p "$INSTALL_DIR/bin"
sudo mkdir -p "$INSTALL_DIR/.scripts"

# Copy bin file and scripts
sudo cp ./target/release/pz2 "$INSTALL_DIR/bin/" -v
sudo cp -r ./.scripts/* "$INSTALL_DIR/.scripts/" -v

# Set permissions
sudo chmod -R 755 "$INSTALL_DIR"

# Add to PATH safely (only if not already there)
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