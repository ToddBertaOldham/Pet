@echo off 
echo Building boot_apps/loader...
cd boot_apps/loader
cargo build --target x86_64-unknown-uefi -Z build-std=core,alloc
echo Building kernel...
cd ../../
cd kernel
cargo build --target targets/x86_64-unknown-none.json -Z build-std=core,alloc