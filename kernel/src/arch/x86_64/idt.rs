//**************************************************************************************************
// idt.rs                                                                                          *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::gdt;
use core::convert::TryInto;
use x86::interrupts::size_64::{interrupt_trap_gate, load_idt};
use x86::ProtectionRing;

static mut ENTRIES: [interrupt_trap_gate::Descriptor; 21] =
    [interrupt_trap_gate::Descriptor::new(); 21];

pub unsafe fn install() {
    println!("Installing IDT...");

    //TODO In future versions of Rust it will likely be possible to make
    // the descriptor initialization constant.

    create_machine_entry(0, divide_error_exception as u64);
    create_machine_entry(1, debug_exception as u64);
    create_machine_entry(2, nmi as u64);
    create_machine_entry(3, breakpoint as u64);
    create_machine_entry(4, overflow_exception as u64);
    create_machine_entry(5, bound_range_exceeded_exception as u64);
    create_machine_entry(6, invalid_opcode_exception as u64);
    create_machine_entry(7, device_not_available_exception as u64);
    create_machine_entry(8, double_fault_exception as u64);
    create_machine_entry(9, coprocessor_segment_exception as u64);
    create_machine_entry(10, invalid_tss_exception as u64);
    create_machine_entry(11, segment_not_present_exception as u64);
    create_machine_entry(12, stack_fault_exception as u64);
    create_machine_entry(13, general_protection_exception as u64);
    create_machine_entry(14, page_fault_exception as u64);
    create_machine_entry(16, x87_fpu_floating_point_error as u64);
    create_machine_entry(17, alignment_check_exception as u64);
    create_machine_entry(18, machine_check_exception as u64);
    create_machine_entry(19, simd_floating_point_exception as u64);
    create_machine_entry(20, virtualization_exception as u64);

    // 15 and 21-31 are reserved by Intel. 32 - 255 are user defined.

    load_idt(&ENTRIES[..].try_into().expect("IDT too large."));

    println!("IDT installed.");
}

unsafe fn create_machine_entry(number: usize, offset: u64) {
    ENTRIES[number].set_is_present(true);
    ENTRIES[number].set_privilege_level(ProtectionRing::Level0);
    ENTRIES[number].set_descriptor_type(interrupt_trap_gate::DescriptorType::Interrupt);
    ENTRIES[number].set_segment_selector(gdt::kernel_code_selector());
    ENTRIES[number].set_offset(offset);
}

// Representation of the handler's stack (figure 6-8 in Intel manual) for use
// with the x86-interrupt calling convention. Field order is reversed compared
// to the figure since the stack goes from a higher to lowers address unlike
// structs which are lower to higher.

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct StackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

extern "x86-interrupt" fn divide_error_exception(stack_frame: &StackFrame) {}

extern "x86-interrupt" fn debug_exception(stack_frame: &StackFrame) {}

extern "x86-interrupt" fn nmi(stack_frame: &StackFrame) {}

extern "x86-interrupt" fn breakpoint(stack_frame: &StackFrame) {
    println!("Breakpoint hit!");
}

extern "x86-interrupt" fn overflow_exception(stack_frame: &StackFrame) {}

extern "x86-interrupt" fn bound_range_exceeded_exception(stack_frame: &StackFrame) {}

extern "x86-interrupt" fn invalid_opcode_exception(stack_frame: &StackFrame) {}

extern "x86-interrupt" fn device_not_available_exception(stack_frame: &StackFrame) {}

extern "x86-interrupt" fn double_fault_exception(stack_frame: &StackFrame, error_code: u64) {}

extern "x86-interrupt" fn coprocessor_segment_exception(stack_frame: &StackFrame) {}

extern "x86-interrupt" fn invalid_tss_exception(stack_frame: &StackFrame, error_code: u64) {}

extern "x86-interrupt" fn segment_not_present_exception(stack_frame: &StackFrame, error_code: u64) {
}

extern "x86-interrupt" fn stack_fault_exception(stack_frame: &StackFrame, error_code: u64) {}

extern "x86-interrupt" fn general_protection_exception(stack_frame: &StackFrame, error_code: u64) {}

extern "x86-interrupt" fn page_fault_exception(stack_frame: &StackFrame, error_code: u64) {}

extern "x86-interrupt" fn x87_fpu_floating_point_error(stack_frame: &StackFrame) {}

extern "x86-interrupt" fn alignment_check_exception(stack_frame: &StackFrame, error_code: u64) {}

extern "x86-interrupt" fn machine_check_exception(stack_frame: &StackFrame) {}

extern "x86-interrupt" fn simd_floating_point_exception(stack_frame: &StackFrame) {}

extern "x86-interrupt" fn virtualization_exception(stack_frame: &StackFrame) {}
