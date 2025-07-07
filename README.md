# Rusterminal - An "alternative" to Zsh and Bash

## Current builds:

- Latest release: **(v0.3.4)**
- Latest beta: **(v0.3.5-rc1)**

# Tested on:

- Arch Linux
- Arch Linux (WSL)
- Linux Mint
- Fedora
- Raspberry PI OS
- openSUSE Tumbleweed

# For **Linux** machines only.
## Supported distro bases:
- Debian
- Ubuntu
- Fedora
- Arch
- SUSE

## And if you have one of the package managers listed:
- Apt
- Dnf
- Pacman
- Zypper

**Optional:**
- Flatpak
- Yay
- Paru

# Install:

## 1. Clone the repo:

**Release:**

`$ git clone https://github.com/Meme-Supplier/Rusterminal.git`

**Beta: (WIP Releases, can be extremely broken)**

`$ git clone --branch beta --single-branch https://github.com/Meme-Supplier/Rusterminal.git`

## 2. Navigate to the source:

`$ cd ~/Rusterminal/installer`

## 3. Install Rusterminal:

`$ bash install.sh`

# Uninstall:

## Method 1:
`$ rusterminal`

`rusterminal$~: rusterminal uninstall`

## Method 2: (If you can't launch Rusterminal)

`$ cd ~/rusterminal/installer`

`$ chmod +x uninstall.sh`

`$ ./uninstall.sh`
