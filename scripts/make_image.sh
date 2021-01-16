#!/bin/bash

#***************************************************************************************************
# make_image.sh                                                                                    *
# Copyright (c) 2021 Aurora Berta-Oldham                                                           *
# This code is made available under the MIT License.                                               *
#***************************************************************************************************

# Remove old directory and create a new one.
rm -f -r image
mkdir -p image/src/boot

# Create FAT16 boot image for UEFI.
dd if=/dev/zero of=image/src/boot.img bs=512 count=32000
mkfs.fat -F 16 image/src/boot.img

# Mount boot image.
mount -t msdos image/src/boot.img image/src/boot -o loop

# Copy files to boot image.
mkdir -p image/src/boot/EFI/BOOT/
mkdir -p image/src/boot/boot/system/
cp target/x86_64-unknown-uefi/debug/loader.efi image/src/boot/EFI/BOOT/BOOTX64.EFI
cp  target/x86_64-unknown-none/debug/kernel image/src/boot/boot/system/kernel

# Unmount boot image.
umount image/src/boot
rm -r image/src/boot

# Create ISO.
xorriso -as mkisofs -R -f -e boot.img -no-emul-boot -c boot.cat -V "VERDURE_OS" -A "Verdure OS" -iso-level 3 -o image/verdure_os.iso image/src