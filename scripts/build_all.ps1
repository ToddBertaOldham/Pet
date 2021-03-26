#***************************************************************************************************
# build_all.ps1                                                                                    *
# Copyright (c) 2021 Aurora Berta-Oldham                                                           *
# This code is made available under the MIT License.                                               *
#***************************************************************************************************

Write-Host "Building " -ForegroundColor Green -NoNewline
Write-Host "boot_apps/loader"
Set-Location boot_apps/loader
cargo build --target x86_64-unknown-uefi -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem
Set-Location ../../

Write-Host "Building " -ForegroundColor Green -NoNewline
Write-Host "kernel"
Set-Location kernel
cargo build --target targets/x86_64-unknown-none.json -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem
Set-Location ../