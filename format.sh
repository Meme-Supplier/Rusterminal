#!/bin/bash

# 2025 Meme Supplier
# memesupplierbusiness@gmail.com
# Maintained by Meme Supplier

# For developers only
# Replace path names as needed

cd src || exit 1

rustfmt main.rs
rustfmt cmds.rs
rustfmt funcs.rs
rustfmt logger.rs
rustfmt xray.rs
rustfmt sysinfo.rs
