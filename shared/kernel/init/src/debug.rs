//**************************************************************************************************
// debug.rs                                                                                        *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub use uart_8250_family::{BaudDivisor,PortNumber};
use encapsulation::GetterSetters;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, GetterSetters)]
pub struct DebugConfig {
    #[field_access(set = true, borrow_self = false)]
    enabled: bool,
    #[field_access(set = true, borrow_self = false)]
    port_number: PortNumber,
    #[field_access(set = true, borrow_self = false)]
    baud_divisor: BaudDivisor
}

impl Default for DebugConfig {
    fn default() -> Self {
        DebugConfig {
            enabled: false,
            port_number: PortNumber::COM1,
            baud_divisor: BaudDivisor::RATE_9600
        }
    }
}