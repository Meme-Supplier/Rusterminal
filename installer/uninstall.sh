#!/bin/bash

# 2025 Meme Supplier
# memesupplierbusiness@gmail.com
# Maintained by Meme Supplier

echo -e "\nDo you want to uninstall Rusterminal?\n(Y or N)"
read -r answer

case "$answer" in
    [Nn]) exit ;;
esac

# Remove Rusterminal files and symlink
sudo rm -rf ~/rusterminal
sudo rm -f /usr/local/bin/rusterminal

rm -r ~/.config/rusterminal

# Remove from /etc/shells if present
if grep -qxF "/usr/local/bin/rusterminal" /etc/shells; then
    sudo sed -i '\|/usr/local/bin/rusterminal|d' /etc/shells
fi

echo -e "\nUninstalled!"
read
