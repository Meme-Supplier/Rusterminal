#!/bin/bash

# 2025 Meme Supplier
# memesupplierbusiness@gmail.com
# Maintained by Meme Supplier

echo -e "Do you want to uninstall Rusterminal?\n(Y or N)"
read -r answer

case "$answer" in
    [Nn]) exit ;;
esac

sudo rm -rf ~/rusterminal
sudo rm -f /usr/local/bin/rusterminal

echo -e "\nUninstalled!"
read
