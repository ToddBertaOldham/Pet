//**************************************************************************************************
// arch.rs                                                                                         *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::stack_frame::StackFrame;

pub(super) extern "x86-interrupt" fn divide_error_exception(stack_frame: &StackFrame) {}

pub(super) extern "x86-interrupt" fn debug_exception(stack_frame: &StackFrame) {}

pub(super) extern "x86-interrupt" fn nmi(stack_frame: &StackFrame) {}

pub(super) extern "x86-interrupt" fn breakpoint(stack_frame: &StackFrame) {}

pub(super) extern "x86-interrupt" fn overflow_exception(stack_frame: &StackFrame) {}

pub(super) extern "x86-interrupt" fn bound_range_exceeded_exception(stack_frame: &StackFrame) {}

pub(super) extern "x86-interrupt" fn invalid_opcode_exception(stack_frame: &StackFrame) {}

pub(super) extern "x86-interrupt" fn device_not_available_exception(stack_frame: &StackFrame) {}

pub(super) extern "x86-interrupt" fn double_fault_exception(
    stack_frame: &StackFrame,
    error_code: u64,
) {
    println!("A double fault exception was thrown.");
}

pub(super) extern "x86-interrupt" fn coprocessor_segment_exception(stack_frame: &StackFrame) {}

pub(super) extern "x86-interrupt" fn invalid_tss_exception(
    stack_frame: &StackFrame,
    error_code: u64,
) {
}

pub(super) extern "x86-interrupt" fn segment_not_present_exception(
    stack_frame: &StackFrame,
    error_code: u64,
) {
}

pub(super) extern "x86-interrupt" fn stack_fault_exception(
    stack_frame: &StackFrame,
    error_code: u64,
) {
}

pub(super) extern "x86-interrupt" fn general_protection_exception(
    stack_frame: &StackFrame,
    error_code: u64,
) {
}

pub(super) extern "x86-interrupt" fn page_fault_exception(
    stack_frame: &StackFrame,
    error_code: u64,
) {
    println!("A page fault exception was thrown.");
}

pub(super) extern "x86-interrupt" fn x87_fpu_floating_point_error(stack_frame: &StackFrame) {}

pub(super) extern "x86-interrupt" fn alignment_check_exception(
    stack_frame: &StackFrame,
    error_code: u64,
) {
}

pub(super) extern "x86-interrupt" fn machine_check_exception(stack_frame: &StackFrame) {}

pub(super) extern "x86-interrupt" fn simd_floating_point_exception(stack_frame: &StackFrame) {}

pub(super) extern "x86-interrupt" fn virtualization_exception(stack_frame: &StackFrame) {}
