#!/bin/bash

# 2025 Meme Supplier
# memesupplierbusiness@gmail.com
# Maintained by Meme Supplier

detect_package_manager() {
	if command -v pacman &>/dev/null; then
		echo "pacman"
	elif command -v apt &>/dev/null; then
		echo "apt"
	elif command -v dnf &>/dev/null; then
		echo "dnf"
	elif command -v zypper &>/dev/null; then
		echo "zypper"
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

echo -e "\nDo you want to install Rusterminal?\n(Y or N)"
read -r answer

case "$answer" in
[Nn]) exit ;;
esac

echo ""

if [ -d ~/rusterminal ]; then
	sudo rm -rf ~/rusterminal/
fi

if [ -d ~/.config/rusterminal ]; then
	sudo rm -rf ~/.config/rusterminal
fi

if [ -d ~/.rusterminal_history ]; then
	sudo rm -rf ~/.rusterminal_history
fi

mkdir -p ~/rusterminal

echo -e "\nInstalling dependencies...\n"

if [[ "$PM" == "pacman" ]]; then
	sudo pacman -Syu --noconfirm
	sudo pacman -S --noconfirm rustup dosfstools ntfs-3g nano bash parted gcc
elif [[ "$PM" == "apt" ]]; then
	sudo apt update -y
	sudo apt upgrade -y
	sudo apt install -y curl build-essential dosfstools ntfs-3g nano bash parted gcc
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
elif [[ "$PM" == "dnf" ]]; then
	sudo dnf update -y
	sudo dnf install -y curl rustup gcc glibc-devel clang llvm make cmake dosfstools ntfs-3g nano bash parted gcc
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
elif [[ "$PM" == "zypper" ]]; then
	sudo zypper refresh
	sudo zypper update -y
	sudo zypper install -y curl rustup gcc glibc-devel clang llvm make cmake dosfstools ntfs-3g nano bash parted
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
else
	echo "Error: Unsupported package manager."
	read
	exit 1
fi

# Ensure cargo is available
export PATH="$HOME/.cargo/bin:$PATH"

rustup default nightly

if [[ ! -d "$HOME/Rusterminal" ]]; then
	echo "Error: Rusterminal source directory not found!"
	read
	exit 1
fi

cd ~/Rusterminal || exit

cp -r src installer Cargo.toml changes.md launch.sh ~/rusterminal/
mkdir -p ~/.config/rusterminal && cp ~/Rusterminal/config/settings.conf ~/.config/rusterminal/
cd ~/.config/rusterminal && cp settings.conf defaults.conf


chmod +x ~/rusterminal/launch.sh
sudo ln -sf ~/rusterminal/launch.sh /usr/local/bin/rusterminal

sudo rm -rf ~/Rusterminal
sudo rm -f ~/rusterminal/installer/install.sh

echo -e "Installed Rusterminal!\n"

echo -e "Do you want to edit Rusterminal's configurations before launching?\n(Y or N)"
read -r answer

case "$answer" in
[Yy]) nano ~/.config/rusterminal/settings.conf ;;
esac

echo -e "\nDo you want to launch Rusterminal?\n(Y or N)"
read -r answer

case "$answer" in
[Yy]) rusterminal ;;
esac
