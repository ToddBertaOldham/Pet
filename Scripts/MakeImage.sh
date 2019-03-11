rm -r Image
mkdir Image
dd if=/dev/zero of=Image/boot.img bs=1M count=100
mkfs.fat -F 32 Image/boot.img
mmd -i Image/boot.img ::/EFI
mmd -i Image/boot.img ::/EFI/BOOT
mmd -i Image/boot.img ::/Boot
mcopy -i Image/boot.img target/x86_64-UEFI/debug/uefi_loader.efi ::EFI/BOOT/BOOTX64.EFI
mcopy -i Image/boot.img target/x86_64-unknown-none/debug/pet_kernel ::Boot/Kernel
xorriso -as mkisofs -R -f -e boot.img -no-emul-boot -o Image/Pet.iso Image/

