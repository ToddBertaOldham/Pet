# shared
This folder contains all of the crates that are used by several pieces of the OS.
## arch
Architecture interfaces. Just x86 (64 bit) for now.
## elf
ELF binary reader and loader.
## newtypes
Macros for generating various [newtypes](https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html).
## encapsulation
Custom derives for generating getters and setters for fields and bit values. Uses [syn](https://github.com/dtolnay/syn), [quote](https://github.com/dtolnay/quote), and [proc-macro2](https://github.com/alexcrichton/proc-macro2) to handle Rust tokens.
## bits
A utility crate for easily reading and modifying bits. May be removed in the future.
## io
Provides replacements/alternatives to some structs and traits found in the std::io module. Allows for endian aware reading and writing.
## kernel
Crates for interacting with the kernel. Just kernel_init at the moment. Will contain system calls later.
## uart_8250_family
Interface for the 8250 UART, 16650A UART, and other chips in the family.
## ucs2
A utility crate for working with [UCS-2](https://en.wikipedia.org/wiki/Universal_Coded_Character_Set) encoding.