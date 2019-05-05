rm -r image
mkdir image
dd if=/dev/zero of=image/boot.img bs=1M count=100
mkfs.fat -F 32 image/boot.img
mmd -i image/boot.img ::/EFI
mmd -i image/boot.img ::/EFI/BOOT
mmd -i image/boot.img ::/Boot
mcopy -i image/boot.img target/x86_64-uefi/debug/uefi_loader.efi ::EFI/BOOT/BOOTX64.EFI
mcopy -i image/boot.img target/x86_64-unknown-none/debug/kernel ::boot/kernel
xorriso -as mkisofs -R -f -e boot.img -no-emul-boot -o image/pet.iso image/