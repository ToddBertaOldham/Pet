@echo off 
echo Running clippy for Boot/UEFI/Loader...
cd Boot/UEFI/Loader
cargo xclippy --target ../Targets/x86_64-UEFI.json
echo Running clippy for Kernel...
cd ../../../
cd Kernel
cargo xclippy --target Targets/x86_64-unknown-none.json