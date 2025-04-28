#!/usr/bin/env python3

"""
2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier
"""

import subprocess
import readline

version = "v1.2"

def choose():
    global file
    choice = input("Choice: ")

    match choice:
        case '1':
            file = "main.rs"
        case '2':
            file = "cmds.rs"
        case '3':
            file = "funcs.rs"
        case '4':
            file = "loadconfigs.rs"
        case '5':
            file = "diskfmt.py"
        case '6':
            file = "ver.py"
        case '7':
            file = "xray.py"
        case '8':
            file = "install.sh"
        case '9':
            file = "launch.sh"
        case '10':
            file = "upgrade.sh"
        case '11':
            file = "uninstall.sh"
        case _:
            print("Invalid option! Choose again!")
            choose()

print(f"\nRusterminal Viewer {version}")
print("\nChoose file to view (select number):\n")

files = ["1] main.rs",
         "2] cmds.rs",
         "3] funcs.rs",
         "4] loadconfigs.rs",
         "5] diskfmt.py",
         "6] ver.py",
         "7] xray.py",
         "8] install.sh",
         "9] launch.sh",
         "10] upgrade.sh",
         "11] uninstall.sh\n"]

for line in files:
    print(line)

choose()
subprocess.run([f"nano ~/rusterminal/src/{file}"],shell=True)
