#!/bin/bash

echo

detect_package_manager() {
    if command -v pacman &>/dev/null; then
        echo "pacman"
    elif command -v apt &>/dev/null; then
        echo "apt"
    elif command -v dnf &>/dev/null; then
        echo "dnf"
    else
        echo "Unsupported package manager"
        return 1
    fi
}

# Sets the package manager
PM=$(detect_package_manager)

if [[ "$PM" == "Unsupported package manager" ]]; then
    echo "Error: No supported package manager found."
    exit 1
fi

# Asks if you want to install
echo -e "Do you want to install Rusterminal?\n(Y or N)"
read -p "" answer

case "$answer" in
    [Nn]) # If N answered, exits the script
        exit
        ;;
esac

# Create new dir
mkdir -p ~/.rusterminal

# Installs dependencies
echo "Installing dependencies:"

if [[ "$PM" == "pacman" ]]; then
    sudo pacman -Syu
    sudo pacman -S rustup
    rustup default stable
elif [[ "$PM" == "apt" ]]; then
    sudo apt update
    sudo apt install curl build-essential
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
else
    sudo dnf update
    sudo dnf install rustup
fi

rustup default stable

# Installs the files
cd ~/Rusterminal

sudo cp -r src ~/.rusterminal

sudo cp Cargo.lock ~/.rusterminal
sudo cp Cargo.toml ~/.rusterminal

sudo chmod +x ~/.rusterminal/src/launch.sh
sudo ln -sf ~/.rusterminal/launch.sh /usr/local/bin/rusterminal
