#!/bin/bash

# echo "setting keyword"
# loadkeys br-abnt2

lsblk -l
echo "disk path"
read path
sudo cfdisk "$path"
lsblk -l
echo "what is your partition configuration \
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
read is
if ["$swap" == 1]; then
    echo "partition path
    read swap_path
    mkswap "$swap_path"
fi
echo "root path"
read root_path
mkfs."$type" "$root_path"
echo "home? 
if yes => press the key 1
if not => press any key"
read is
if ["$swap" == 1]; then
    echo "partition path
    read home_path
    mkfs."$type" "$home_path"
fi
lsblk -l