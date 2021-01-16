#!/bin/bash

#***************************************************************************************************
# make_image.sh                                                                                    *
# Copyright (c) 2021 Aurora Berta-Oldham                                                           *
# This code is made available under the MIT License.                                               *
#***************************************************************************************************

+# Remove old directory and create a new one.
rm -f -r image
mkdir -p image/src/

# Create FAT16 boot image for UEFI.
dd if=/dev/zero of=image/src/boot.img bs=512 count=32000
mkfs.fat -F 16 image/src/boot.img

# Copy files to boot image.
mmd -i image/src/boot.img ::/EFI
mmd -i image/src/boot.img ::/EFI/BOOT
mmd -i image/src/boot.img ::/boot
mmd -i image/src/boot.img ::/boot/system
mcopy -i image/src/boot.img target/x86_64-unknown-uefi/debug/loader.efi ::EFI/BOOT/BOOTX64.EFI
mcopy -i image/src/boot.img target/x86_64-unknown-none/debug/kernel ::boot/system/kernel

# Create ISO.
xorriso -as mkisofs -R -f -e boot.img -no-emul-boot -c boot.cat -V "VERDURE_OS" -A "Verdure OS" -iso-level 3 -o image/verdure_os.iso image/src