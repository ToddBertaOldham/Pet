# Shared
This folder contains all of the crates that are used by several pieces of the OS.
## Arch
Architecture interfaces. Just x86 (64 bit) for now.
## ELF
ELF binary reader and loader.
## Generation
Generic code generation macros.
## Bits
A utility crate for easily reading and modifying bits.
## IO
Provides replacements/alternatives to some structs and traits found in the std IO module. Allows for endian aware reading and writing.