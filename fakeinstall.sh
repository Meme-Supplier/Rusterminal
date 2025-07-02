#!/bin/bash

# 2025 Meme Supplier
# memesupplierbusiness@gmail.com
# Maintained by Meme Supplier

# For developers only
# Replace path names as needed

if [ -d ~/rusterminal ]; then
    sudo rm -rf ~/rusterminal/
fi

cp -r ~/Documents/Code/Rust/Rusterminal/ ~/
cd ~/Rusterminal/installer || exit
bash install.sh
