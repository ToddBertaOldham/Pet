// *************************************************************************
// text.rs
// Copayright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use crate::ffi::{ SimpleTextOutputProtocol, Status };
use crate::error::UefiError;
use crate::system as uefi_system;
use core::fmt::{ Write, Error };

pub struct TextOuputWriter(*mut SimpleTextOutputProtocol);

impl TextOuputWriter {
    pub unsafe fn new(protocol : *mut SimpleTextOutputProtocol) -> Self {
        TextOuputWriter(protocol)
    }
}

impl Write for TextOuputWriter {
    fn write_str(&mut self, s : &str) -> Result<(), Error> {
        if s.is_empty() {
            return Ok(());
        }

        unsafe {

            unsafe fn flush(protocol : *mut SimpleTextOutputProtocol, characters : &mut [u16; 128], next_character : &mut usize) -> Result<(), Error> {
                characters[*next_character] = 0;
                match ((*protocol).output_string)(protocol, &mut characters[0]) {
                    Status::SUCCESS => Ok(()),
                    _ => Err(Error)
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

pub fn console_writer() -> Result<TextOuputWriter, UefiError> {
    unsafe {
        let system_table = &*uefi_system::system_table()?;

        if system_table.con_out.is_null() {
            return Err(UefiError::BootServicesUnavailable);
        }

        Ok(TextOuputWriter::new(system_table.con_out))
    }
}

pub fn std_error_writer()-> Result<TextOuputWriter, UefiError> {
    unsafe {
        let system_table = &*uefi_system::system_table()?;

        if system_table.std_error.is_null() {
            return Err(UefiError::BootServicesUnavailable);
        }

        Ok(TextOuputWriter::new(system_table.std_error))
    }
}

#[macro_export]
macro_rules! writerln {
    ($dst:expr) => (
        write!($dst, "\r\n")
    );
    ($dst:expr,) => (
        writerln!($dst)
    );
    ($dst:expr, $($arg:tt)*) => (
        $dst.write_fmt(format_args!("{}\r\n", format_args!($($arg)*)))
    );
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::text::console_writer().expect("Failed to get console writer!").write_fmt(format_args!($($arg)*)).expect("Failed to write to console!"));
}

#[macro_export]
macro_rules! printrln {
    () => (print!("\r\n"));
    ($($arg:tt)*) => ($crate::io::text::console_writer().expect("Failed to get console writer!").write_fmt(format_args!("{}\r\n", format_args!($($arg)*))).expect("Failed to write to console!"))
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::io::text::std_error_writer().expect("Failed to get std error writer!").write_fmt(format_args!($($arg)*)).expect("Failed to write to std error!"));
}

#[macro_export]
macro_rules! eprintrln {
    () => (eprint!("\r\n"));
    ($($arg:tt)*) => ($crate::io::text::std_error_writer().expect("Failed to get std error writer!").write_fmt(format_args!("{}\r\n", format_args!($($arg)*))).expect("Failed to write to std error!"))
}