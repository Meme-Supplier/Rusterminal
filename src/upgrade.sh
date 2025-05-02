#!/bin/bash

sudo echo ""
echo "Upgrading Rusterminal..."

cd ~/

if [ -d ~/rusterminal ]; then
    sudo rm -rf ~/rusterminal/
fi

git clone https://github.com/Meme-Supplier/Rusterminal.git

cd Rusterminal/src

sudo chmod +x install.sh

./install.sh

echo "Rusterminal successfully updated!"
