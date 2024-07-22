#!/bin/bash

REPO="https://github.com/isaac-parks/imt_cli.git"
TEMP_DIR="$(mktemp -d)"

if ! command -v cargo &> /dev/null
then
    echo "Cargo is not installed. Please install Rust and Cargo with the command: brew install rust"
    exit 1
fi

git clone $REPO $TEMP_DIR
cd $TEMP_DIR
cargo install --path $TEMP_DIR
rm -rf $TEMP_DIR
echo "imt_cli was installed at ~/.cargo/bin/imt_cli. In order to use it from the command line, make sure of the following:"
echo "1.) add ~/.cargo/bin to your PATH variable."
echo "2.) create an OS environment variable called IMT_SERVICES_DIR set to the directory containing IMT nubs"
echo "3.) create an OS environment variable called VAULT_TOKEN_DB set to value of the IMT vault token db"