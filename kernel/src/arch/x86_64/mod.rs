//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[macro_use]
#[allow(dead_code)]
#[allow(unused_macros)]
pub mod debug;
pub mod gdt;
pub mod idt;
pub mod tss;

pub use x86::stall;

use kernel_init::KernelArgs;
use x86::interrupts;

#[no_mangle]
pub unsafe extern "sysv64" fn main(args: &KernelArgs) -> ! {
    if args.is_outdated() {
        stall();
    }

    debug::config(args.debug_config());

    crate::print_header();

    interrupts::disable();

    gdt::install();

    idt::install();

    interrupts::enable();

    crate::main_stage_2();
}
