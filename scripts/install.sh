#!/bin/bash

DIR=$(mktemp -d)

git clone https://github.com/nanoservicesforge/NanoForge.git "$DIR"


# Compile the project
# This step varies greatly depending on the project. Adjust as necessary.
cd "$DIR" || exit

# Remove the .git folder. This stops it from being "seen" as a git repo.
rm -rf .git

cargo build --release

# Check for OS and move the binary to the system's bin directory
if [ "$(uname)" = "Linux" ]; then
    # Linux
    if [ -e /usr/local/bin/nanoforge ]; then
        sudo rm /usr/local/bin/nanoforge
    fi
    sudo mv target/release/nanoforge /usr/local/bin/
    rm -rf "$DIR"

elif [ "$(uname)" = "Darwin" ]; then
    # macOS
    if [ -e /usr/local/bin/nanoforge ]; then
        sudo rm /usr/local/bin/nanoforge
    fi
    sudo mv target/release/nanoforge /usr/local/bin/
    rm -rf "$DIR"

else
    echo "Unsupported OS"
    rm -rf "$DIR"
    exit 1
fi

echo "Installation complete."
