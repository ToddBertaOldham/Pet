# UEFI Boot
This folder contains all crates necessary for booting Pet through UEFI. Currently only x86_64 systems are supported. This boot process is fairly simple but will likely under go many changes as the need for a more complex system emerges.
## Core
This crate provides both FFI bindings for a subset of the UEFI specification and a high level wrapper. The high level wrapper does not cover all of the FFI and instead focuses on safety and ease of use for functionality required by applications such as Loader. The FFI and wrapper can be used together if necessary. 
## Loader
This crate is responsible for loading and mapping the kernel, setting-up framebuffers, retrieving the memory map, and other essential boot loader functionality. Run the command below to build it. Make sure [xbuild](https://github.com/rust-osdev/cargo-xbuild) is installed.
```
cargo xbuild --target ../Targets/x86_64-UEFI.json
```
The custom target will likely be removed if [rust-lld is used for the x86_64-unknown-uefi target](https://github.com/rust-lang/rust/pull/58976). The need for xbuild may also change in the future if Cargo gains its functionality.