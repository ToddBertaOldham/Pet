#***************************************************************************************************
# clippy.ps1                                                                                       *
# Copyright (c) 2021 Aurora Berta-Oldham                                                           *
# This code is made available under the MIT License.                                               *
#***************************************************************************************************

Write-Host "Inspecting " -ForegroundColor Green -NoNewline
Write-Host "boot_apps/loader"
Set-Location boot_apps/loader
cargo clippy --target x86_64-unknown-uefi -Z build-std=core,alloc
Set-Location ../../

Write-Host "Inspecting " -ForegroundColor Green -NoNewline
Write-Host "kernel"
Set-Location kernel
cargo clippy --target targets/x86_64-unknown-none.json -Z build-std=core,alloc
Set-Location ../