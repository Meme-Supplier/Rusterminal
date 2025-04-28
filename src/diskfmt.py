#!/usr/bin/env python3

"""
2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier
"""

import subprocess
import sys
import os
import time
import readline

diskfmtver = "0.3.4"

MAX_RETRIES = 5
WAIT_TIME = 5  # seconds

def runShellCommand(command):
    try:
        subprocess.run(command, shell=True, check=True)
    except subprocess.CalledProcessError:
        print(f"Command failed: {command}")
        sys.exit(1)

def checkIfDeviceExists(drive):
    result = subprocess.run(
        ["lsblk", "-p", "-n", "-o", "NAME"],
        capture_output=True,
        text=True
    )
    return drive in result.stdout.splitlines()

def getFirstPartition(drive):
    retries = 0
    while retries < MAX_RETRIES:
        result = subprocess.run(
            ["lsblk", "-ln", "-o", "NAME,TYPE", drive],
            capture_output=True,
            text=True
        )
        for line in result.stdout.strip().splitlines():
            parts = line.split()
            if len(parts) == 2 and parts[1] == "part":
                part_name = parts[0]
                return f"/dev/{part_name}"

        print(f"Retrying to detect partition (Attempt {retries+1}/{MAX_RETRIES})...")
        retries += 1
        time.sleep(WAIT_TIME)
    print("Failed to detect partition after retries.")
    sys.exit(1)

def formatDisk(drive, name, fsys, ptable):
    print(f"\nUnmounting all partitions on {drive}...")
    subprocess.run(
        f"lsblk -ln -o NAME {drive} | grep -E '[0-9]+$' | while read part; do sudo umount /dev/$part || true; done",
        shell=True
    )

    print("Wiping filesystem signatures...")
    runShellCommand(f"sudo wipefs -a --force {drive}")

    print("Writing zeros to the beginning of the disk...")
    runShellCommand(f"sudo dd if=/dev/zero of={drive} bs=1M count=100 status=progress")

    print(f"Creating {ptable.upper()} partition table...")
    runShellCommand(f"sudo parted -s {drive} mklabel {ptable}")

    print("Creating primary partition...")
    runShellCommand(f"sudo parted -s {drive} mkpart primary 0% 100%")

    print("Waiting for the partition table to be recognized...")
    subprocess.run(f"sudo partprobe {drive}", shell=True)
    subprocess.run("sudo udevadm settle", shell=True)

    partition = getFirstPartition(drive)

    print(f"Creating new {fsys.upper()} filesystem with label '{name}'...")

    if fsys == "vfat":
        runShellCommand(f"sudo mkfs.vfat -F 32 -n {name} {partition}")
    elif fsys == "ntfs":
        runShellCommand(f"sudo mkfs.ntfs -f -L {name} {partition}")
    else:
        runShellCommand(f"sudo mkfs.{fsys} -L {name} {partition}")

    print("Mounting to /mnt...")
    runShellCommand(f"sudo mount {partition} /mnt")

    user = os.getenv("USER") or os.getlogin()

    if fsys == "ext4":
        runShellCommand(f"sudo chown -R {user}:{user} /mnt")
    else:
        print(f"Skipping chown for filesystem {fsys.upper()} (not supported).")

def main():
    print(f"Rusterminal Disk Formatter\nVersion: {diskfmtver}")
    print("\033[91m!!! WARNING: This tool will ERASE your selected disk completely. Proceed with caution !!!\033[0m\n")
    input("Press Enter to continue...")

    runShellCommand("lsblk -p")

    disk = input("\nEnter full disk path (e.g., /dev/sdX or /dev/nvmeXn1): ").strip()
    if not disk.startswith("/dev/") or " " in disk:
        print("Invalid disk path.")
        sys.exit(1)

    if not checkIfDeviceExists(disk):
        print(f"Error: Device {disk} not found or is not a valid block device.")
        sys.exit(1)

    print("\nSelect partition table type:")
    print("1] GPT")
    print("2] MSDOS")
    table = input("Choice: ").strip()

    match table:
        case '1':
            ptable = "gpt"
        case '2':
            ptable = "msdos"
        case _:
            print("Invalid option.")
            sys.exit(1)

    name = input("\nEnter disk label (no spaces): ").strip()
    if " " in name or not name:
        print("Disk label cannot contain spaces and must not be empty.")
        sys.exit(1)

    print("\nSelect filesystem type:")
    print("1] ext4")
    print("2] fat32")
    print("3] ntfs")
    fs_choice = input("Choice: ").strip()

    match fs_choice:
        case '1':
            fsys = "ext4"
        case '2':
            fsys = "vfat"
        case '3':
            fsys = "ntfs"
        case _:
            print("Invalid filesystem choice.")
            sys.exit(1)

    print(f"\nFinal confirmation:")
    print(f"Disk: {disk}")
    print(f"Partition Table: {ptable.upper()}")
    print(f"Filesystem: {fsys.upper()}")
    print(f"Label: {name}")

    confirm = input("\nType 'YES' to continue: ").strip()
    if confirm != "YES":
        print("Operation canceled.")
        sys.exit(0)

    formatDisk(disk, name, fsys, ptable)

    print("\nDisk formatting complete!\nYou may need to remove and reinsert your drive to mount it!")

if __name__ == "__main__":
    main()
