#!/bin/bash

sudo echo ""
echo "Upgrading Rusterminal..."

cd ~/


if [ -d ~/rusterminal ]; then
    sudo rm -rf ~/rusterminal/
fi

git clone https://github.com/Meme-Supplier/Rusterminal.git

cd Rusterminal
cd src

sudo chmod +x install.sh

./install.sh

source ~/.bashrc

echo "Rusterminal successfully updated!"
read
