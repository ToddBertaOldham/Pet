//**************************************************************************************************
// idt.rs                                                                                          *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub mod arch;
mod stack_frame;

use super::gdt;
use core::convert::TryInto;
use x86::interrupts::size_64::{interrupt_trap_gate, load_idt};
use x86::ProtectionRing;

static mut ENTRIES: [interrupt_trap_gate::Descriptor; 21] =
    [interrupt_trap_gate::Descriptor::new(); 21];

pub unsafe fn install() {
    create_arch_entry(0, arch::divide_error_exception as u64);
    create_arch_entry(1, arch::debug_exception as u64);
    create_arch_entry(2, arch::nmi as u64);
    create_arch_entry(3, arch::breakpoint as u64);
    create_arch_entry(4, arch::overflow_exception as u64);
    create_arch_entry(5, arch::bound_range_exceeded_exception as u64);
    create_arch_entry(6, arch::invalid_opcode_exception as u64);
    create_arch_entry(7, arch::device_not_available_exception as u64);
    create_arch_entry(8, arch::double_fault_exception as u64);
    create_arch_entry(9, arch::coprocessor_segment_exception as u64);
    create_arch_entry(10, arch::invalid_tss_exception as u64);
    create_arch_entry(11, arch::segment_not_present_exception as u64);
    create_arch_entry(12, arch::stack_fault_exception as u64);
    create_arch_entry(13, arch::general_protection_exception as u64);
    create_arch_entry(14, arch::page_fault_exception as u64);
    create_arch_entry(16, arch::x87_fpu_floating_point_error as u64);
    create_arch_entry(17, arch::alignment_check_exception as u64);
    create_arch_entry(18, arch::machine_check_exception as u64);
    create_arch_entry(19, arch::simd_floating_point_exception as u64);
    create_arch_entry(20, arch::virtualization_exception as u64);

    // 15 and 21-31 are reserved by Intel. 32 - 255 are user defined.

    load_idt(&ENTRIES[..].try_into().expect("IDT too large."));

    println!("IDT installed.");
}

unsafe fn create_arch_entry(number: usize, offset: u64) {
    ENTRIES[number].set_is_present(true);
    ENTRIES[number].set_privilege_level(ProtectionRing::Level0);
    ENTRIES[number].set_descriptor_type(interrupt_trap_gate::DescriptorType::Interrupt);
    ENTRIES[number].set_segment_selector(gdt::kernel_code_selector());
    ENTRIES[number].set_offset(offset);
}
