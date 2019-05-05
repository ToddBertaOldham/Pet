@echo off 
echo Building boot/uefi/loader...
cd boot/uefi/loader
cargo xbuild --target ../targets/x86_64-uefi.json
echo Building kernel...
cd ../../../
cd kernel
cargo xbuild --target targets/x86_64-unknown-none.json