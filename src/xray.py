#!/usr/bin/env python3

"""
2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier
"""

import subprocess
import readline
import sys

version = "v1.4"

def choose():
    global file
    choice = input("Choice: ")

    match choice:
        case 'exit':
            sys.exit(0)
        case '0':
            sys.exit(0)
        case '1':
            file = "main.rs"
        case '2':
            file = "cmds.rs"
        case '3':
            file = "funcs.rs"
        case '4':
            file = "diskfmt.py"
        case '5':
            file = "ver.py"
        case '6':
            file = "xray.py"
        case '7':
            file = "install.sh"
        case '8':
            file = "launch.sh"
        case '9':
            file = "upgrade.sh"
        case '10':
            file = "uninstall.sh"
        case _:
            print("Invalid option! Choose again!")
            choose()

print(f"\nRusterminal Viewer {version}")
print("\nChoose file to view (select number):\nType \"0\" or \"exit\" to exit.\n")

files = ["1] main.rs",
         "2] cmds.rs",
         "3] funcs.rs",
         "4] diskfmt.py",
         "5] ver.py",
         "6] xray.py",
         "7] install.sh",
         "8] launch.sh",
         "9] upgrade.sh",
         "10] uninstall.sh\n"]

for line in files:
    print(line)

choose()
subprocess.run([f"nano ~/rusterminal/src/{file}"],shell=True)
