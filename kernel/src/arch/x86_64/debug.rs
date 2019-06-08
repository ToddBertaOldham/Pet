// *************************************************************************
// debug.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use uart_8250_family::{ SerialPort, PortNumber };
use core::fmt::{ Write, Arguments };

static mut SERIAL_PORT : Option<SerialPort> = None;

pub unsafe fn init() {
    let mut serial_port = SerialPort::new(PortNumber::COM1);
    if serial_port.configure(Default::default()).is_ok() {
        SERIAL_PORT = Some(serial_port);
    }
    else {
        disable();
    }
}

pub fn disable() {
    unsafe {
        SERIAL_PORT = None;
    }
}

pub fn is_available() -> bool {
    unsafe {
        SERIAL_PORT.is_some()
    }
}

pub fn write_str(s: &str) {
    unsafe {
        match &mut SERIAL_PORT {
            Some(ref mut serial_port) => serial_port.write_str(s).unwrap_or_else(|_| disable()),
            None => { }
        };
    }
}

pub fn write_fmt(args: Arguments)  {
    unsafe {
        match &mut SERIAL_PORT {
            Some(ref mut serial_port) => serial_port.write_fmt(args).unwrap_or_else(|_| disable()),
            None => { }
        };
    }
}

macro_rules! print {
    ($($arg:tt)*) => ($crate::arch::debug::write_fmt(format_args!($($arg)*)));
}

macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ($crate::arch::debug::write_fmt(format_args!("{}\r\n", format_args!($($arg)*))))
}