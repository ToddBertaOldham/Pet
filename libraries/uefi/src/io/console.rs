//**************************************************************************************************
// console.rs                                                                                      *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub use crate::ffi::simple_text_output::{BackColor, FrontColor};

use crate::error::Error;
use crate::ffi::simple_text_output;
use crate::ffi::simple_text_output::ColorAttribute;
use crate::ffi::Status;
use crate::system;
use core::fmt;
use ucs2::ToUcs2Buffer;

#[repr(transparent)]
#[derive(Clone)]
pub struct OutputDevice(*mut simple_text_output::Protocol);

impl OutputDevice {
    pub unsafe fn new(protocol: *mut simple_text_output::Protocol) -> Self {
        Self(protocol)
    }

    pub fn con_out() -> Result<Self, Error> {
        unsafe {
            let system_table = &*system::table()?;

            if system_table.con_out.is_null() {
                return Err(Error::BootServicesUnavailable);
            }

            Ok(Self::new(system_table.con_out))
        }
    }

    pub fn std_error() -> Result<Self, Error> {
        unsafe {
            let system_table = &*system::table()?;

            if system_table.std_error.is_null() {
                return Err(Error::BootServicesUnavailable);
            }

            Ok(Self::new(system_table.std_error))
        }
    }

    //TODO Mode functions.

    pub fn set_colors(
        &mut self,
        back_color: BackColor,
        front_color: FrontColor,
    ) -> Result<(), Error> {
        unsafe {
            let output = &*self.0;

            let attribute = ColorAttribute::new(back_color, front_color);

            let status = (output.set_attribute)(self.0, attribute);

            match status {
                Status::SUCCESS => Ok(()),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                _ => Err(Error::UnexpectedStatus(status)),
            }
        }
    }

    pub fn set_cursor_position(&mut self, column: usize, row: usize) -> Result<(), Error> {
        unsafe {
            let output = &*self.0;
            let status = (output.set_cursor_position)(self.0, column, row);

            match status {
                Status::SUCCESS => Ok(()),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                Status::UNSUPPORTED => Err(Error::NotSupported),
                _ => Err(Error::UnexpectedStatus(status)),
            }
        }
    }

    pub fn set_cursor_visible(&mut self, visible: bool) -> Result<(), Error> {
        unsafe {
            let output = &*self.0;
            let status = (output.enable_cursor)(self.0, visible);

            match status {
                Status::SUCCESS => Ok(()),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                Status::UNSUPPORTED => Err(Error::NotSupported),
                _ => Err(Error::UnexpectedStatus(status)),
            }
        }
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        unsafe {
            let output = &*self.0;
            let status = (output.clear_screen)(self.0);

            match status {
                Status::SUCCESS => Ok(()),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                Status::UNSUPPORTED => Err(Error::NotSupported),
                _ => Err(Error::UnexpectedStatus(status)),
            }
        }
    }
}

impl fmt::Write for OutputDevice {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        if s.is_empty() {
            return Ok(());
        }

        unsafe {
            unsafe fn flush(
                protocol: *mut simple_text_output::Protocol,
                characters: &mut [u16; 128],
                next_character: &mut usize,
            ) -> Result<(), fmt::Error> {
                characters[*next_character] = 0;
                match ((*protocol).output_string)(protocol, &mut characters[0]) {
                    Status::SUCCESS => Ok(()),
                    _ => Err(fmt::Error),
                }
            }

            let mut characters: [u16; 128] = [0; 128];
            let mut next_character = 0;

            for char16 in s.encode_usc2() {
                characters[next_character] = char16.map_err(|_| fmt::Error)?;
                next_character += 1;

                if next_character == 127 {
                    flush(self.0, &mut characters, &mut next_character)?;
                    next_character = 0;
                }
            }

            if next_character > 0 {
                flush(self.0, &mut characters, &mut next_character)?;
            }

            Ok(())
        }
    }
}

#[macro_export]
macro_rules! con_out_print {
    ($($arg:tt)*) => (core::fmt::Write::write_fmt(&mut $crate::io::console::OutputDevice::con_out().expect("Failed to get con out output device!"), format_args!($($arg)*)).expect("Failed to write to con out!"));
}

#[macro_export]
macro_rules! con_out_println {
    () => (print!("\r\n"));
    ($($arg:tt)*) => (core::fmt::Write::write_fmt(&mut $crate::io::console::OutputDevice::con_out().expect("Failed to get con out output device!"), format_args!("{}\r\n", format_args!($($arg)*))).expect("Failed to write to con out!"))
}

#[macro_export]
macro_rules! std_error_print {
    ($($arg:tt)*) => (core::fmt::Write::write_fmt(&mut $crate::io::console::OutputDevice::std_error().expect("Failed to get std error output device!"), format_args!($($arg)*)).expect("Failed to write to std error!"));
}

#[macro_export]
macro_rules! std_error_println {
    () => (eprint!("\r\n"));
    ($($arg:tt)*) => (core::fmt::Write::write_fmt(&mut $crate::io::console::OutputDevice::std_error().expect("Failed to get std error output device!"), format_args!("{}\r\n", format_args!($($arg)*))).expect("Failed to write to std error!"))
}
