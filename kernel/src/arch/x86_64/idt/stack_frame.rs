//**************************************************************************************************
// stack_frame.rs                                                                                  *
// Copyright (c) 2021 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

// Representation of the handler's stack (figure 6-8 in Intel manual) for use
// with the x86-interrupt calling convention. Field order is reversed compared
// to the figure since the stack goes from a higher to lowers address unlike
// structs which are lower to higher.

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub(super) struct StackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}
