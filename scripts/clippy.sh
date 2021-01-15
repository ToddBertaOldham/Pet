#!/bin/bash

#***************************************************************************************************
# clippy.sh                                                                                        *
# Copyright (c) 2021 Aurora Berta-Oldham                                                           *
# This code is made available under the MIT License.                                               *
#***************************************************************************************************

echo Inspecting boot_apps/loader
cd boot_apps/loader
cargo xclippy --target x86_64-unknown-uefi -Z build-std=core,alloc
cd ../../

echo Inspecting kernel
cd kernel
cargo xclippy --target targets/x86_64-unknown-none.json -Z build-std=core,alloc
cd ../