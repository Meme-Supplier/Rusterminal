import subprocess
import readline

version = 1.0

def runcmd(cmd):
    subprocess.run([cmd],shell=True)

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
print("\nChoose file to view (select number):\n")

files = ["1] main.rs",
         "2] cmds.rs",
         "3] funcs.rs",
         "4] diskfmt.py",
         "5] ver.py",
         "6] xray.py",
         "7] install.sh",
         "8] launch.sh",
         "9] upgrade.sh",
         "10] uninstall.sh\n",]

for line in files:
    print(line)

choose()

runcmd(f"nano ~/rusterminal/src/{file}")
