#!/bin/bash

# 2025 Meme Supplier
# memesupplierbusiness@gmail.com
# Maintained by Meme Supplier

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

PM=$(detect_package_manager)

if [[ "$PM" == "Unsupported package manager" ]]; then
    echo "Error: No supported package manager found."
    exit 1
fi

echo -e "Do you want to install Rusterminal?\n(Y or N)"
read -r answer

case "$answer" in
    [Nn]) exit ;;
esac

if [ -d ~/rusterminal ]; then
    echo -e "${CYAN}\nRemoving \"$HOME/rusterminal/\"...${RESET}"
    sudo rm -rf ~/rusterminal/
fi

mkdir -p ~/rusterminal

echo -e "\nInstalling dependencies...\n"

if [[ "$PM" == "pacman" ]]; then
    sudo pacman -Syu --noconfirm
    sudo pacman -S --noconfirm rustup dosfstools ntfs-3g nano python-colorama
    source "$HOME/.cargo/env"
elif [[ "$PM" == "apt" ]]; then
    sudo apt update -y
    sudo apt upgrade -y
    sudo apt install -y curl build-essential dosfstools ntfs-3g nano python3-colorama
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
elif [[ "$PM" == "dnf" ]]; then
    sudo dnf update -y
    sudo dnf install -y curl rustup gcc glibc-devel clang llvm make cmake dosfstools ntfs-3g nano python3-colorama
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Error: Unsupported package manager."
    read
    exit 1
fi

# Ensure cargo is available
export PATH="$HOME/.cargo/bin:$PATH"

# Set Rust to stable
rustup default stable

if [[ ! -d "$HOME/Rusterminal" ]]; then
    echo "Error: Rusterminal source directory not found!"
    read
    exit 1
fi

cd ~/Rusterminal || exit

cp -r src ~/rusterminal/
cp Cargo.toml ~/rusterminal/

chmod +x ~/rusterminal/src/launch.sh
sudo ln -sf ~/rusterminal/src/launch.sh /usr/local/bin/rusterminal

if ! grep -qxF "/usr/local/bin/rusterminal" /etc/shells; then
    echo "Registering /usr/local/bin/rusterminal as a login shell..."
    echo "/usr/local/bin/rusterminal" | sudo tee -a /etc/shells > /dev/null
fi

sudo rm -rf ~/Rusterminal

echo -e "Installed Rusterminal!\n"

echo -e "Do you want to launch Rusterminal?\n(Y or N)"
read -r answer

case "$answer" in
    [Yy]) rusterminal ;;
esac
