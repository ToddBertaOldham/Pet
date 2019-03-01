// *************************************************************************
// text_io.rs
// Copayright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::ffi::{ SimpleTextOutputProtocol };
use super::error::UEFIError;
use super::system as uefi_system;
use core::fmt::{ Write, Error };

pub struct TextOuputWriter(*mut SimpleTextOutputProtocol);

impl TextOuputWriter {
    pub unsafe fn new(protocol : *mut SimpleTextOutputProtocol) -> Self {
        TextOuputWriter(protocol)
    }
}

impl Write for TextOuputWriter {
    fn write_str(&mut self, s : &str) -> Result<(), Error> {
        let length = s.encode_utf16().count();

        if length == 0 {
            return Ok(());
        }

        unsafe {

            unsafe fn flush(protocol : *mut SimpleTextOutputProtocol, characters : &mut [u16; 128], next_character : &mut usize) {
                characters[*next_character] = 0;
                ((*protocol).output_string)(protocol, &mut characters[0]);
            }
            
            let mut characters : [u16; 128] = [0; 128];
            let mut next_character = 0;

            for char16 in s.encode_utf16() {
                //TODO UEFI uses UCS-2 not UTF-16.
                characters[next_character] = char16;
                next_character += 1;

                if next_character == 127 {
                    flush(self.0, &mut characters, &mut next_character);
                    next_character = 0;
                }
            }

            if next_character > 0 {
                flush(self.0, &mut characters, &mut next_character);
            }

            Ok(())
        }
    }
}

pub fn console_writer() -> Result<TextOuputWriter, UEFIError> {
    unsafe {
        let system_table = &*uefi_system::system_table()?;

        if system_table.con_out.is_null() {
            return Err(UEFIError::BootServicesUnavailable);
        }

        Ok(TextOuputWriter::new(system_table.con_out))
    }
}

pub fn std_error_writer()-> Result<TextOuputWriter, UEFIError> {
    unsafe {
        let system_table = &*uefi_system::system_table()?;

        if system_table.std_error.is_null() {
            return Err(UEFIError::BootServicesUnavailable);
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
    ($($arg:tt)*) => ($crate::console_writer().expect("Failed to get console writer!").write_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printrln {
    () => (print!("\r\n"));
    ($($arg:tt)*) => ($crate::console_writer().expect("Failed to get console writer!").write_fmt(format_args!("{}\r\n", format_args!($($arg)*))))
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::std_error_writer().expect("Failed to get std error writer!").write_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintrln {
    () => (eprint!("\r\n"));
    ($($arg:tt)*) => ($crate::std_error_writer().expect("Failed to get std error writer!").write_fmt(format_args!("{}\r\n", format_args!($($arg)*))))
}