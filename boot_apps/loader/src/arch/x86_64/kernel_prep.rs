//**************************************************************************************************
// kernel_prep.rs                                                                                  *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch::stall;
use core::mem;
use elf;
use kernel_interface::init;

static mut ENTRY_ADDRESS: usize = 0;
static mut ARGS: init::Args = init::Args::new();

pub unsafe fn enter_kernel(entry_address: usize, args: init::Args) {
    // Inlining the kernel_jump method and not moving the entry address out of the stack causes
    // some issues in the kernel. Not moving args causes the kernel not to boot entirely. At some
    // point it would be nice to look more into the exact cause of this, but it is likely due
    // to Rust asm trying to handle the state of the old stack when the new stack has been loaded.

    ENTRY_ADDRESS = entry_address;
    ARGS = args;

    // Set stack pointer to new BP stack mapped in higher half of virtual memory.
    llvm_asm!("mov $0, %rsp" :: "r"(init::BP_STACK_VIRTUAL_TOP) :: "volatile");

    kernel_jump();
}

unsafe fn kernel_jump() -> ! {
    let entry: init::EntryFunction = mem::transmute(ENTRY_ADDRESS);
    (entry)(&ARGS);
    stall();
}

pub fn check_headers(_: &elf::IdentityHeader, header: &elf::Header) {
    assert_eq!(
        header.machine,
        elf::Machine::X86_64,
        "Kernel is not x86_64."
    );
}
