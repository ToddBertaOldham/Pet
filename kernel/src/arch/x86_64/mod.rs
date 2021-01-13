//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019-2021 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[macro_use]
pub mod debug;
pub mod gdt;
pub mod idt;
pub mod interrupt_controller;
pub mod sync;
pub mod timing;
pub mod tss;

pub use x86::interrupts;
pub use x86::stall;

use kernel_init;

pub const PAGE_SIZE: usize = 4096;

pub const KERNEL_VIRTUAL_START: usize = 0xffffffff80000000;

#[no_mangle]
pub unsafe extern "sysv64" fn entry(args: *const kernel_init::Args) {
    if let Some(args_value) = args.as_ref() {
        if args_value.is_outdated() {
            return;
        }

        debug::writer().config(args_value.debug_config);

        crate::print_header();

        interrupts::disable();

        gdt::install();

        idt::install();

        interrupt_controller::init();

        timing::init();

        interrupts::enable();

        crate::main(args_value)
    }
}
