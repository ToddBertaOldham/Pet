// *************************************************************************
// mod.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

// Re-export from Shared/Arch/x86_64.
pub use x86::sixty_four::*;
pub use x86::control_registers;
pub use x86::port_io;

use uart_8250_family::*;
use core::fmt::Write;

#[no_mangle]
pub unsafe extern fn main() {
    let mut debug = SerialPort::new(PortNumber::COM1);
    debug.configure(Default::default()).unwrap();
    debug.write_str("Pet kernel\n").unwrap();
    debug.write_str("Copyright 2018-2019 Todd Berta-Oldham\n").unwrap();

    if cfg!(debug_assertions) {
        debug.write_str("This is a debug build.\n").unwrap();
    }
}