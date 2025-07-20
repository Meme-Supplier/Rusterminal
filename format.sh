#!/bin/bash

# For developers only
# Replace path names as needed

DEV_PATH=""                  # Leave empty if you need to
CLEAR_ON_EVERY_ACTION="true" # Change to false or something else otherwise

# Install dependencies (assuming rustfmt is already installed)
echo -e "Installing dependencies...\n"
if command -v pacman &>/dev/null; then
    sudo pacman -S --needed python-pip shfmt

elif command -v apt &>/dev/null; then
    sudo apt update
    sudo apt install -y python3-pip shfmt

elif command -v dnf &>/dev/null; then
    sudo dnf install -y python3-pip shfmt

elif command -v zypper &>/dev/null; then
    sudo zypper install -y python3-pip shfmt

else
    echo "Unsupported distro or package manager not found"
    exit 1
fi

if [ "$CLEAR_ON_EVERY_ACTION" = "true" ]; then
    clear
fi

# Rust files
echo "Formatting Rust files..."

rustfmt src/main.rs
rustfmt src/cmds.rs
rustfmt src/funcs.rs
rustfmt src/logger.rs
rustfmt src/xray.rs
rustfmt src/sys/s_info.rs
rustfmt src/sys/s_vars.rs

if [ "$CLEAR_ON_EVERY_ACTION" = "true" ]; then
    clear
fi

# Python files
echo "Formatting Python files..."

if [ ! -d ".venv" ]; then
    echo -e "\nCreating virtual env in .venv"
    python3 -m venv ".venv"
fi

".venv/bin/pip" install --upgrade pip black
".venv/bin/black" src/diskfmt.py

if [ -n "$DEV_PATH" ]; then
    cd "$DEV_PATH" || {
        echo "Failed to cd into $DEV_PATH"
        exit 1
    }
fi

if [ "$CLEAR_ON_EVERY_ACTION" = "true" ]; then
    clear
fi

# Shell files
echo "Formatting Shell files..."

shfmt -w -i 4 -ci -ln bash installer/install.sh
shfmt -w -i 4 -ci -ln bash installer/uninstall.sh
shfmt -w -i 4 -ci -ln bash installer/upgrade.sh
shfmt -w -i 4 -ci -ln bash fakeinstall.sh
shfmt -w -i 4 -ci -ln bash format.sh
shfmt -w -i 4 -ci -ln bash launch.sh

if [ "$CLEAR_ON_EVERY_ACTION" = "true" ]; then
    clear
fi

echo "Files have been formatted."
