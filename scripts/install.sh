#!/bin/bash

DIR=`mktemp -d`

git clone git@github.com:nanoservicesforge/NanoForge.git $DIR


# Compile the project
# This step varies greatly depending on the project. Adjust as necessary.
cd $DIR

cargo build --release

# Check for OS and move the binary to the system's bin directory
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    if [ -e /usr/local/bin/nanoforge ]; then
        sudo rm /usr/local/bin/nanoforge
    fi
    sudo mv target/release/nanoforge /usr/local/bin/
    rm -rf $DIR


# macOS
elif [[ "$OSTYPE" == "darwin"* ]]; then
    if [ -e /usr/local/bin/nanoforge ]; then
        sudo rm /usr/local/bin/nanoforge
    fi
    sudo mv target/release/nanoforge /usr/local/bin/
    rm -rf $DIR

# Unsupported
else
    echo "Unsupported OS"
    rm -rf $DIR
    exit 1
fi

echo "Installation complete."
