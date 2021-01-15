//**************************************************************************************************
// debug.rs                                                                                        *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub use uart_8250_family::{BaudDivisor, Port};

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct DebugConfig {
    enabled: bool,
    port_number: Port,
    baud_divisor: BaudDivisor,
}

impl DebugConfig {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn port_number(&self) -> Port {
        self.port_number
    }

    pub fn set_port_number(&mut self, value: Port) {
        self.port_number = value;
    }

    pub fn baud_divisor(&self) -> BaudDivisor {
        self.baud_divisor
    }

    pub fn set_baud_divisor(&mut self, value: BaudDivisor) {
        self.baud_divisor = value;
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        DebugConfig {
            enabled: cfg!(debug_assertions),
            port_number: Port::COM_1,
            baud_divisor: BaudDivisor::RATE_9600,
        }
    }
}
