#!/bin/bash

# echo "setting keyword"
# loadkeys br-abnt2
set -e

lsblk -l
echo "disk path"
read path
sudo cfdisk "$path"
lsblk -l
echo "what is your partition configuration:
    1 => ext4
    2 => btrfs
"
read type
echo "format boot partion, enter the path"
read boot
mkfs.fat -F 32 "$boot"
echo "swap? 
if yes => press the key 1
if not => press any key"
read swap_path
if [ "$swap_path" == 1 ]; then
    echo "partition path"
    read swap
    swap_path="$swap"
    mkswap "$swap_path"
fi

echo "root path"
read root_path
mkfs."$type" "$root_path"
echo "home? 
if yes => press the key 1
if not => press any key"
read home_path

if [ "$home_path" == 1 ]; then
    echo "partition path"
    read path
    home_path="$path"
    mkfs."$type" "$home_path"
fi
clear
lsblk
echo "mount root"
mount "$root_path" /mnt
echo "creating home directory"
mkdir /mnt/home
echo "creating boot directory"
mkdir -p /mnt/boot/efi
echo "mounting partitions"
mount "$home_path" /mnt/home
mount "$boot_path" /mnt/boot

if [! -d "/mnt/boot/efi"]; then
    mkdir /mnt/boot/rfi
fi
mount "$boot_path" /mnt/boot/efi

if [ "$swap_path" == 1 ]; then
    swapon "$swap_path"
fi
lsblk
echo "Okay"

