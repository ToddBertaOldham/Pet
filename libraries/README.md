# libraries
## arch
Architecture interfaces. Just x86 (64 bit) for now.
## elf
ELF binary reader and loader.
## newtypes
Macros for generating various [newtypes](https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html).
## bits
A utility crate for easily reading and modifying bits.
## io
Provides replacements/alternatives to some structs and traits found in the std::io module. Allows for endian aware reading and writing.
## kernel
Crates for interacting with the kernel. Just kernel_init at the moment. Will contain system calls later.
## uart_8250_family
Interface for the 8250 UART, 16650A UART, and other chips in the family.
## split
A library for splitting types (especially integers) into halves or quarters.
## ucs2
A utility crate for working with [UCS-2](https://en.wikipedia.org/wiki/Universal_Coded_Character_Set) encoding.
## uefi
Provides an ffi interface for the UEFI api as well as a high level wrapper.