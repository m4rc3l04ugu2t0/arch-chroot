# Installation Guide For Arch Linux

This guide provides detailed instructions for installing Arch Linux, including disk partitioning, mounting file systems, and installing essential packages.

## 1. Configure Keyboard

```bash
loadkeys br-abnt2
# Load keyboard layout.
```

## 2. Connecting to Wi-Fi

```bash
rfkill unblock all
# Unblock all radio devices (Wi-Fi, Bluetooth, etc.).

iwctl
# Start the tool from the command line for iwd.

# Inside iwctl:
station list
# List all network interfaces.

station wlan0 get-networks
# List all networks on the wlan interface.
# Your interface name may be different.

station wlan0 connect network-name
# Connect to the Wi-Fi.
```

## 3. Synchronize the System Clock

```bash
timedatectl set-ntp true
# Enables automatic time synchronization via NTP.
```

## 4. Partition the Disk

```bash
fdisk /dev/nvme0n1
# Disk partition utility for NVMe disk.

# Create partition table:
# - /dev/nvme0n1p1: EFI Partition, 512 MB
# - /dev/nvme0n1p2: Btrfs Partition, remaining disk 
```

## 5. Format Partitions

```bash
mkfs.fat -F32 /dev/nvme0n1p1
# Create a FAT32 file system in the EFI partition.

mkfs.btrfs /dev/nvme0n1p2
# Create a Btrfs file system in the second partition. 
```

## 6. Create Btrfs Subvolumes

```bash
mount /dev/nvme0n1p2 /mnt
# Mount the Btrfs partition temporarily.

btrfs su cr /mnt/@
# Create a subvolume at the root.

btrfs su cr /mnt/@home
# Create a subvolume at home.

btrfs su cr /mnt/@snapshots
# Create a subvolume at snapshots.

btrfs su cr /mnt/@var_log
# Create a subvolume at /var/log.

btrfs su cr /mnt/@var_cache_pacman
# Create a subvolume at /var/cache/pacman/pkg.

umount /mnt
# Unmount the partition temporarily. 
```

## 7. Mount Subvolumes

```bash
mount -o subvol=@ /dev/nvme0n1p2 /mnt
# Mount the subvolume at the root.

mkdir /mnt/{boot,home,.snapshots,var/log,var/cache/pacman/pkg}
# Create the necessary directories.

mount /dev/nvme0n1p1 /mnt/boot
# Mount the EFI partition.

mount -o subvol=@home /dev/nvme0n1p2 /mnt/home
# Mount the subvolume at home.

mount -o subvol=@snapshots /dev/nvme0n1p2 /mnt/.snapshots
# Mount the subvolume at snapshots.

mount -o subvol=@var_log /dev/nvme0n1p2 /mnt/var/log
# Mount the subvolume at var_log.

mount -o subvol=@var_cache_pacman /dev/nvme0n1p2 /mnt/var/cache/pacman/pkg
# Mount the subvolume at var_cache_pacman.
```

## 8. Install the Base System

```bash
pacstrap /mnt base linux linux-firmware base-devel vim dhcpcd
# Install base packages on the mounted system.
```

## 9. Generate the fstab

```bash
genfstab -U /mnt >> /mnt/etc/fstab
# Generate the fstab file using UUIDs.
cat /mnt/etc/fstab
# Verify the content of the fstab file.
```

## 10. Enter the New System

```bash
arch-chroot /mnt
# Switch to the new installed system.
```

## 11. Configure Timezone and Clock

```bash
ln -sf /usr/share/zoneinfo/Region/City /etc/localtime
# Set the timezone.

hwclock --systohc
# Set the hardware clock from the system clock.
```

## 12. Configure Localization

```bash
vim /etc/locale.gen
# Open the locale configuration file in Vim.

locale-gen
# Generate the specified locales.

echo KEYMAP=br-abnt2 > /etc/vconsole.conf
# Set the keyboard layout for the virtual console.
```

## 13. Configure Hostname

```bash
echo arch > /etc/hostname
# Set the hostname to "arch".

echo "127.0.0.1    localhost" >> /etc/hosts
echo "::1          localhost" >> /etc/hosts
echo "127.0.1.1    my-hostname.localdomain my-hostname" >> /etc/hosts
# Add entries to the /etc/hosts file.
```

## 14. Set Root Password

```bash
passwd
# Set the root user's password.
```

## 15. Create User

```bash
useradd -m -g users -G wheel,video,audio,kvm -s /bin/bash arch
# Add a new user.

passwd arch
# Set the password for the user "arch".
```

## 16. Install Essential Packages

```bash
pacman -Sy dosfstools os-prober mtools network-manager-applet networkmanager wpa_supplicant wireless_tools dialog sudo
# Install essential packages.
```

## 17. Install and Configure the GRUB Bootloader

```bash
pacman -S grub efibootmgr
# Install GRUB and efibootmgr.

grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=arch --recheck
# Install GRUB in EFI mode.

grub-mkconfig -o /boot/grub/grub.cfg
# Generate the GRUB configuration file.
```

## 18. Configure zRAM for Swap

```bash
pacman -S zramswap
# Install zramswap.

systemctl enable zramswap.service
# Enable the zramswap service.
```

## 19. Exit Chroot, Unmount Partitions, and Reboot

```bash
exit
# Exit chroot.

umount -R /mnt
# Recursively unmount all mounted partitions.

reboot
# Reboot the system.
```

## 20. Post-Installation Configuration

```bash
systemctl enable NetworkManager
# Enable the NetworkManager service.

systemctl start NetworkManager
# Start the NetworkManager service.

# (Optional) Install graphical environment, drivers, and login manager.
# Example: KDE Plasma, NVIDIA/AMD drivers, and SDDM.
```