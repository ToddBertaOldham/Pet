//**************************************************************************************************
// debug.rs                                                                                        *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub use uart_8250_family::{BaudDivisor, Port};

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct DebugConfig {
    pub enabled: bool,
    pub port_number: Port,
    pub baud_divisor: BaudDivisor,
}

impl DebugConfig {
    pub const fn new() -> Self {
        DebugConfig {
            enabled: cfg!(debug_assertions),
            port_number: Port::COM_1,
            baud_divisor: BaudDivisor::RATE_9600,
        }
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self::new()
    }
}
