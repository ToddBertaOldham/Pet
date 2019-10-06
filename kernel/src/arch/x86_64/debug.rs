//**************************************************************************************************
// debug.rs                                                                                        *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::spinlock::Spinlock;
use core::fmt::{Error, Write};
use core::ops::DerefMut;
use kernel_init::DebugConfig;
use uart_8250_family::{SerialPort, Settings};

static WRITER: Spinlock<Writer> = Spinlock::new(Writer::new());

#[derive(Clone, Debug)]
pub struct Writer(Option<SerialPort>);

impl Writer {
    pub const fn new() -> Self {
        Self(None)
    }

    pub unsafe fn config(&mut self, config: DebugConfig) {
        if config.enabled() {
            let mut serial_port = SerialPort::new(config.port_number());

            let mut settings = Settings::default();
            settings.set_baud_divisor(config.baud_divisor());

            if serial_port.configure(settings).is_ok() {
                self.0 = Some(serial_port);
                return;
            }
        }

        self.disable();
    }

    pub fn disable(&mut self) {
        self.0 = None;
    }

    pub fn is_available(&self) -> bool {
        self.0.is_some()
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        if let Some(ref mut serial_port) = self.0 {
            let result = serial_port.write_str(s);
            if result.is_err() {
                self.disable();
            }
            result
        } else {
            Err(Error)
        }
    }
}

pub fn writer() -> &'static mut Writer {
    WRITER.lock().deref_mut()
}

macro_rules! print {
    ($($arg:tt)*) => (core::fmt::Write::write_fmt($crate::arch::debug::writer(), format_args!($
    ($arg)*)));
}

macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (core::fmt::Write::write_fmt($crate::arch::debug::writer(), format_args!
    ("{}\n", format_args!($($arg)*))))
}
