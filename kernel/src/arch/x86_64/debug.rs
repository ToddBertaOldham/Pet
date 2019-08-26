//**************************************************************************************************
// debug.rs                                                                                        *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::fmt::{Arguments, Write};
use kernel_init::DebugConfig;
use uart_8250_family::{PortNumber, SerialPort, Settings, WordLength};

static mut SERIAL_PORT: Option<SerialPort> = None;

pub unsafe fn config(config: DebugConfig) {
    if config.enabled() {
        let mut serial_port = SerialPort::new(config.port_number());

        let mut settings = Settings::default();
        settings.set_baud_divisor(config.baud_divisor());

        if serial_port.configure(settings).is_ok() {
            SERIAL_PORT = Some(serial_port);
            return;
        }
    }

    disable();
}

pub fn disable() {
    unsafe {
        SERIAL_PORT = None;
    }
}

pub fn is_available() -> bool {
    unsafe { SERIAL_PORT.is_some() }
}

pub fn write_str(s: &str) {
    unsafe {
        if let Some(ref mut serial_port) = &mut SERIAL_PORT {
            serial_port.write_str(s).unwrap_or_else(|_| disable());
        };
    }
}

pub fn write_fmt(args: Arguments) {
    unsafe {
        if let Some(ref mut serial_port) = &mut SERIAL_PORT {
            serial_port.write_fmt(args).unwrap_or_else(|_| disable());
        }
    }
}

macro_rules! print {
    ($($arg:tt)*) => ($crate::arch::debug::write_fmt(format_args!($($arg)*)));
}

macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ($crate::arch::debug::write_fmt(format_args!("{}\n", format_args!($($arg)*))))
}
