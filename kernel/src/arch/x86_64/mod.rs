//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
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
pub mod vmm;

pub use x86::interrupts;
pub use x86::stall;

use kernel_init;

use crate::{heap, pmm};

pub const PAGE_SIZE: usize = 4096;

#[no_mangle]
pub unsafe extern "sysv64" fn entry(args_ptr: *const kernel_init::Args) {
    if args_ptr.is_null() {
        return;
    }

    let mut args = &*args_ptr;

    if args.is_outdated() {
        return;
    }

    debug::writer().config(args.debug_config);

    crate::print_header();

    interrupts::disable();

    gdt::install();

    idt::install();

    pmm::init(&args.memory_info);

    vmm::init();

    let virtual_args_ptr = vmm::convert_physical_address(args_ptr);

    args = &*virtual_args_ptr;

    heap::init();

    interrupt_controller::init(args);

    timing::init();

    interrupts::enable();

    crate::main(args)
}

pub unsafe extern "sysv64" fn entry_ap() {
    crate::main_ap()
}
