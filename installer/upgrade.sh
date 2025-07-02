#!/bin/bash

sudo echo ""
echo "Upgrading Rusterminal..."

cd ~/ || exit 1

if [ -d ~/rusterminal ]; then
    sudo rm -rf ~/rusterminal/
fi

git clone --branch main --single-branch https://github.com/Meme-Supplier/Rusterminal

cd Rusterminal/installer || exit 1

sudo chmod +x install.sh

./install.sh
