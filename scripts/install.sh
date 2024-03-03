#!/bin/bash

git clone git@github.com:nanoservicesforge/NanoForge.git


# Compile the project
# This step varies greatly depending on the project. Adjust as necessary.
cd NanoForge

cargo build --release
cd ..


# Check for OS and move the binary to the system's bin directory
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    if [[ /usr/local/bin/nanoforge ]]; then
        sudo rm /usr/local/bin/nanoforge
    fi
    sudo mv NanoForge/target/release/nanoforge /usr/local/bin/
    rm -rf NanoForge


elif [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    if [[ /usr/local/bin/nanoforge ]]; then
        sudo rm /usr/local/bin/nanoforge
    fi
    sudo mv NanoForge/target/release/nanoforge /usr/local/bin/
    rm -rf NanoForge
else
    echo "Unsupported OS"
    rm -rf NanoForge
    exit 1
fi

echo "Installation complete."
