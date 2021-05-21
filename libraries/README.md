# libraries
## acpi
ACPI tables.
## arch
Architecture interfaces. Just x86 (64 bit) for now.
## elf
ELF binary reader and loader.
## enums
Helpful macros for creating C-like enums and Rust enums that can easily be converted from integers.
## io
Provides replacements/alternatives to some structs and traits found in the std::io module. Allows for endian aware reading and writing.
## kernel_interface
Crate for booting and interacting with the kernel.
## memory
Allocators, primitives for memory sections and addresses, traits for modifying bits, and more.
## uart_8250_family
Interface for the 8250 UART, 16650A UART, and other chips in the family.
## ucs2
A utility crate for working with [UCS-2](https://en.wikipedia.org/wiki/Universal_Coded_Character_Set) encoding.
## uefi
Provides an FFI interface and high level wrappers for the UEFI api.
## units
Structures for converting between units.