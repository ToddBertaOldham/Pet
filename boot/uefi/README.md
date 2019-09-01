# uefi
This folder contains all crates necessary for booting Pet through UEFI. Currently only x86_64 systems are supported.
## core
This crate provides both FFI bindings for a subset of the UEFI specification and a high level wrapper. The high level wrapper does not cover all of the FFI and instead focuses on safety and ease of use for functionality required by applications such as Loader. The FFI and wrapper can be used together if necessary. 
## loader
This crate is responsible for loading and mapping the kernel, setting-up framebuffers, retrieving the memory map, and other essential boot loader functionality. Most of the code is temporary and will be rewritten in the future for a prettier and more sophisticated boot process.