@echo off 
echo Running clippy for boot/uefi/loader...
cd boot/uefi/loader
cargo xclippy --target ../targets/x86_64-uefi.json
echo Running clippy for kernel...
cd ../../../
cd kernel
cargo xclippy --target targets/x86_64-unknown-none.json