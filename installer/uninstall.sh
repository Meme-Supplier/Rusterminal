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

echo -e "\nDo you want to keep Rusterminal's configurations?\n(Y or N)"
read -r answer

case "$answer" in
[Nn]) rm -r ~/.config/rusterminal ;;
esac

echo -e "\nDo you want to keep Rusterminal's command history?\n(Y or N)"
read -r answer

case "$answer" in
[Nn]) rm -f ~/.rusterminal_history ;;
esac

echo -e "\nUninstalled!"
read
