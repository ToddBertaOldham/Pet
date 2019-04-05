echo Building Boot/UEFI/Loader...
cd Boot/UEFI/Loader
cargo xbuild --target ../Targets/x86_64-UEFI.json
echo Building Kernel...
cd ../../../
cd Kernel
cargo xbuild --target Targets/x86_64-unknown-none.json