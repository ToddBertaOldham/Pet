// *************************************************************************
// mod.rs
// Copayright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

pub mod storage;
pub mod console;

pub use ::io::*;

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