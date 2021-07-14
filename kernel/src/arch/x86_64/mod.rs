//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use kernel_interface::init::Args;
pub use x86::interrupts;
pub use x86::stall;

use crate::{heap, pmm, tm};

#[macro_use]
pub mod debug;
pub mod drivers;
pub mod gdt;
pub mod idt;
pub mod local_apic;
pub mod sync;
pub mod tss;
pub mod vmm;

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

    // Initialize virtual memory manager.
    vmm::init(args);

    // vmm::init maps all physical memory into the higher half of virtual memory so all
    // physical addresses/identity mapped virtual addresses must be offset from this point.

    let virtual_args_ptr = vmm::convert_physical_ptr(args_ptr);
    args = &*virtual_args_ptr;

    pmm::init_stage_two();

    // Try to enable local APIC for the timers and starting APs.
    local_apic::init(args);

    // Initialize timer manager.
    tm::init_bp(args);

    interrupts::enable();

    pmm::init_stage_three();

    crate::main(args)
}

pub unsafe extern "sysv64" fn entry_ap() {
    crate::main_ap()
}
