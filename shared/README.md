# shared
This folder contains all of the crates that are used by several pieces of the OS.
## arch
Architecture interfaces. Just x86 (64 bit) for now.
## elf
ELF binary reader and loader.
## generation
Generic code generation macros.
## bits
A utility crate for easily reading and modifying bits.
## io
Provides replacements/alternatives to some structs and traits found in the std IO module. Allows for endian aware reading and writing.
## uart_8250_family
Interface for the 8250 UART, 16650A UART, and other chips in the family.