//**************************************************************************************************
// sync.rs                                                                                         *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use x86::interrupts;

pub use core::sync::atomic::spin_loop_hint;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LockState {
    enable_interrupts: bool,
}

pub fn start_lock() -> LockState {
    let state = LockState {
        //TODO Save interrupt state.
        enable_interrupts: true,
    };
    unsafe {
        interrupts::disable();
    }
    state
}

pub fn end_lock(state: LockState) {
    unsafe {
        if state.enable_interrupts {
            interrupts::enable();
        }
    }
}
