#!/bin/bash

#***************************************************************************************************
# build_all.sh                                                                                     *
# Copyright (c) 2021 Aurora Berta-Oldham                                                           *
# This code is made available under the MIT License.                                               *
#***************************************************************************************************

# Boot loader
echo Building boot_apps/loader
cd boot_apps/loader
cargo build --target x86_64-unknown-uefi -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem
cd ../../

# Kernel
echo Building kernel
cd kernel
cargo build --target targets/x86_64-unknown-none.json -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem
cd ../