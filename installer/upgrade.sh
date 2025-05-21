#!/bin/bash

sudo echo ""
echo "Upgrading Rusterminal..."

cd ~/ || exit

if [ -d ~/rusterminal ]; then
    sudo rm -rf ~/rusterminal/
fi

git clone --branch main --single-branch https://github.com/Meme-Supplier/Rusterminal

cd Rusterminal/installer || exit

sudo chmod +x install.sh

./install.sh
