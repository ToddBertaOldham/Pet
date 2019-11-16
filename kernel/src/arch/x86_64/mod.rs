//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[macro_use]
pub mod debug;
pub mod gdt;
pub mod idt;
pub mod sync;
pub mod tss;

pub use x86::stall;

use kernel_init::KernelArgs;
use x86::interrupts;

#[no_mangle]
pub unsafe extern "sysv64" fn entry(args: *const KernelArgs) {
    if let Some(args_value) = args.as_ref() {
        if args_value.is_outdated() {
            return;
        }

        debug::writer().config(args_value.debug_config);

        crate::print_header();

        interrupts::disable();

        gdt::install();

        idt::install();

        interrupts::enable();

        crate::main(args_value)
    }
}
