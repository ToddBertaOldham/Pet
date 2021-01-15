@echo off 
echo Running clippy for boot_apps/loader...
cd boot_apps/loader
cargo xclippy --target x86_64-unknown-uefi
echo Running clippy for kernel...
cd ../../../
cd kernel
cargo xclippy --target targets/x86_64-unknown-none.json