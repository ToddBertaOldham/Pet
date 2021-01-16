#!/bin/bash

#***************************************************************************************************
# make_image.sh                                                                                    *
# Copyright (c) 2021 Aurora Berta-Oldham                                                           *
# This code is made available under the MIT License.                                               *
#***************************************************************************************************

rm -r image
mkdir image
dd if=/dev/zero of=image/boot.img bs=1M count=100
mkfs.fat -F 32 image/boot.img
mmd -i image/boot.img ::/EFI
mmd -i image/boot.img ::/EFI/BOOT
mmd -i image/boot.img ::/boot
mmd -i image/boot.img ::/boot/system
mcopy -i image/boot.img target/x86_64-unknown-uefi/debug/loader.efi ::EFI/BOOT/BOOTX64.EFI
mcopy -i image/boot.img target/x86_64-unknown-none/debug/kernel ::boot/system/kernel
xorriso -as mkisofs -R -f -e boot.img -no-emul-boot -o image/verdure.iso image/