// *************************************************************************
// console.rs
// Copayright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

pub use crate::ffi::simple_text_output::{BackColor, FrontColor};

use crate::ffi::Status;
use crate::ffi::simple_text_output;
use crate::ffi::simple_text_output::ColorAttribute;
use crate::error::UefiError;
use crate::system as uefi_system;
use core::fmt;

#[repr(transparent)]
#[derive(Clone)]
pub struct OutputDevice(*mut simple_text_output::Protocol);

impl OutputDevice {
    pub unsafe fn new(protocol : *mut simple_text_output::Protocol) -> Self {
        Self(protocol)
    }

    pub fn con_out() -> Result<Self, UefiError> {
        unsafe {
            let system_table = &*uefi_system::system_table()?;

            if system_table.con_out.is_null() {
                return Err(UefiError::BootServicesUnavailable);
            }

            Ok(Self::new(system_table.con_out))
        }
    }

    pub fn std_error()-> Result<Self, UefiError> {
        unsafe {
            let system_table = &*uefi_system::system_table()?;

            if system_table.std_error.is_null() {
                return Err(UefiError::BootServicesUnavailable);
            }

            Ok(Self::new(system_table.std_error))
        }
    }

    //TODO Mode functions.

    pub fn set_colors(&mut self, back_color : BackColor, front_color : FrontColor) -> Result<(), UefiError> {
        unsafe {
            let output = &*self.0;

            let attribute = ColorAttribute::new(back_color, front_color);

            let status = (output.set_attribute)(self.0, attribute);

            match status {
                Status::SUCCESS => Ok(()),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }

    pub fn set_cursor_position(&mut self, column : usize, row : usize) -> Result<(), UefiError> {
        unsafe {
            let output = &*self.0;
            let status = (output.set_cursor_position)(self.0, column, row);

            match status {
                Status::SUCCESS => Ok(()),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                Status::UNSUPPORTED => Err(UefiError::NotSupported),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }

    pub fn set_cursor_visible(&mut self, visible : bool) -> Result<(), UefiError> {
        unsafe {
            let output = &*self.0;
            let status = (output.enable_cursor)(self.0, visible);

            match status {
                Status::SUCCESS => Ok(()),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                Status::UNSUPPORTED => Err(UefiError::NotSupported),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }

    pub fn clear(&mut self) -> Result<(), UefiError> {
        unsafe {
            let output = &*self.0;
            let status = (output.clear_screen)(self.0);

            match status {
                Status::SUCCESS => Ok(()),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                Status::UNSUPPORTED => Err(UefiError::NotSupported),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }
}

impl fmt::Write for OutputDevice {
    fn write_str(&mut self, s : &str) -> Result<(), fmt::Error> {
        if s.is_empty() {
            return Ok(());
        }

        unsafe {

            unsafe fn flush(protocol : *mut simple_text_output::Protocol, characters : &mut [u16; 128], next_character : &mut usize) -> Result<(), fmt::Error> {
                characters[*next_character] = 0;
                match ((*protocol).output_string)(protocol, &mut characters[0]) {
                    Status::SUCCESS => Ok(()),
                    _ => Err(fmt::Error)
                }
            }
            
            let mut characters : [u16; 128] = [0; 128];
            let mut next_character = 0;

            //TODO UEFI uses UCS-2 not UTF-16.

            for char16 in s.encode_utf16() {
                characters[next_character] = char16;
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
macro_rules! print {
    ($($arg:tt)*) => (core::fmt::Write::write_fmt(&mut $crate::io::console::OutputDevice::con_out().expect("Failed to get con out output device!"), format_args!($($arg)*)).expect("Failed to write to con out!"));
}

#[macro_export]
macro_rules! printrln {
    () => (print!("\r\n"));
    ($($arg:tt)*) => (core::fmt::Write::write_fmt(&mut $crate::io::console::OutputDevice::con_out().expect("Failed to get con out output device!"), format_args!("{}\r\n", format_args!($($arg)*))).expect("Failed to write to con out!"))
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => (core::fmt::Write::write_fmt(&mut $crate::io::console::OutputDevice::std_error().expect("Failed to get std error output device!"), format_args!($($arg)*)).expect("Failed to write to std error!"));
}

#[macro_export]
macro_rules! eprintrln {
    () => (eprint!("\r\n"));
    ($($arg:tt)*) => (core::fmt::Write::write_fmt(&mut $crate::io::console::OutputDevice::std_error().expect("Failed to get std error output device!"), format_args!("{}\r\n", format_args!($($arg)*))).expect("Failed to write to std error!"))
}