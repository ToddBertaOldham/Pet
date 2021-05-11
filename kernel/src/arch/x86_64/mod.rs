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

use kernel_interface::init::Args;

use crate::{heap, pmm};

pub const PAGE_SIZE: usize = 4096;

#[no_mangle]
pub unsafe extern "sysv64" fn entry(args_ptr: *const Args) {
    if args_ptr.is_null() {
        return;
    }

    let mut args = &*args_ptr;

    if args.is_outdated() {
        return;
    }

    debug::writer().config(args);

    println!("Entered Verdure OS x86-64 kernel.");

    interrupts::disable();

    gdt::install();

    idt::install();

    pmm::init_stage_one(args);

    vmm::init(args);

    // vmm::init maps all physical memory into the higher half of virtual memory so all
    // physical addresses/identity mapped virtual addresses must be offset.

    let virtual_args_ptr = vmm::convert_physical_address(args_ptr);

    args = &*virtual_args_ptr;

    pmm::init_stage_two();

    heap::init();

    interrupt_controller::init(args);

    timing::init();

    interrupts::enable();

    pmm::init_stage_three();

    crate::main(args)
}

pub unsafe extern "sysv64" fn entry_ap() {
    crate::main_ap()
}
