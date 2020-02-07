// *************************************************************************
// lib.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]

#[macro_use]
extern crate enums;

pub mod registers;
mod error;
mod settings;

use core::fmt;
use x86::port_io;
use io::{Read, Write, EndianWrite};

use registers::*;
pub use error::*;
pub use settings::*;
pub use registers::InterruptEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PortNumber(u16);

impl PortNumber {
    pub const COM1 : PortNumber = PortNumber(0x3F8);
    pub const COM2 : PortNumber = PortNumber(0x2F8);
    pub const COM3 : PortNumber = PortNumber(0x3E8);
    pub const COM4 : PortNumber = PortNumber(0x2E8);

    pub const fn data_register(self) -> u16 {
        self.0
    }

    pub const fn interrupt_enable_register(self) -> u16 {
        self.0 + 1
    }

    pub const fn lower_divisor_latch_register(self) -> u16 {
        self.0
    }

    pub const fn higher_divisor_latch_register(self) -> u16 {
        self.0 + 1
    }

    pub const fn interrupt_identification_register(self) -> u16 {
        self.0 + 2
    }

    pub const fn fifo_control_register(self) -> u16 {
        self.0 + 2
    }

    pub const fn line_control_register(self) -> u16 {
        self.0 + 3
    }

    pub const fn modem_control_register(self) -> u16 {
        self.0 + 4
    }

    pub const fn line_status_register(self) -> u16 {
        self.0 + 5
    }

    pub const fn modem_status_register(self) -> u16 {
        self.0 + 6
    }

    pub const fn scratch_register(self) -> u16 {
        self.0 + 7
    }
}

impl From<u16> for PortNumber {
    fn from(value : u16) -> Self {
        PortNumber(value)
    }
}

impl From<PortNumber> for u16 {
    fn from(value : PortNumber) -> Self {
        value.0
    }
}


#[derive(Clone, Debug)]
pub struct SerialPort(PortNumber);

impl SerialPort {
    pub const unsafe fn new(port_number : PortNumber) -> Self {
        SerialPort(port_number)
    }

    pub fn configure(&mut self, settings : Settings) -> Result<(), Error> {
        unsafe {
            let divisor : u16 = settings.baud_divisor().into();

            if divisor == 0 {
                return Err(Error::UnsupportedBaudDivisor);
            }

            if self.line_busy() {
                return Err(Error::LineBusy)
            }

            let mut interrupt_enable_register = InterruptEnableRegister::new();
            self.set_interrupt_enable_register(interrupt_enable_register);

            let mut line_control_register = LineControlRegister::new();
            line_control_register.set_divisor_latch_access_enabled(true);
            self.set_line_control_register(line_control_register);

            port_io::out_u8(self.0.lower_divisor_latch_register(), divisor as u8);
            port_io::out_u8(self.0.higher_divisor_latch_register(), (divisor >> 8) as u8);

            line_control_register.set_word_length(settings.word_length());
            line_control_register.set_stop_bits(settings.stop_bits());
            line_control_register.set_parity(settings.parity());
            line_control_register.set_divisor_latch_access_enabled(false);
            self.set_line_control_register(line_control_register);

            let mut fifo_control_register = FifoControlRegister::new();
            fifo_control_register.set_fifo_mode(settings.fifo_mode());
            fifo_control_register.set_clear_receive(true);
            fifo_control_register.set_clear_transmit(true);
            self.set_fifo_control_register(fifo_control_register);

            let mut modem_control_register = ModemControlRegister::new();
            modem_control_register.set_data_terminal_ready(true);
            modem_control_register.set_request_to_send(true);
            modem_control_register.set_auxillary_output_2(true);
            self.set_modem_control_register(modem_control_register);

            interrupt_enable_register.set_recieved_data_available_interrupt(settings.recieved_data_available_interrupt());
            interrupt_enable_register.set_transmitter_holding_register_empty_interrupt(settings.transmitter_holding_register_empty_interrupt());
            interrupt_enable_register.set_line_status_interrupt(settings.line_status_interrupt());
            interrupt_enable_register.set_modem_status_interrupt(settings.modem_status_interrupt());
            self.set_interrupt_enable_register(interrupt_enable_register);

            Ok(())
        }
    }

    pub fn line_busy(&self) -> bool {
        unsafe {
            let status = self.line_status_register();
            !status.empty_data_holding_register() || status.data_ready()
        }
    }

    pub fn check_for_error(&self) -> Result<(), Error> {
        unsafe {
            Result::<(), Error>::from(self.line_status_register())
        }
    }

    pub fn interrupt_event(&self) -> InterruptEvent {
        unsafe {
            self.interrupt_identification_register().interrupt_event()
        }
    }

    unsafe fn line_status_register(&self) -> LineStatusRegister {
        let value = port_io::in_u8(self.0.line_status_register());
        LineStatusRegister::from(value)
    }
    unsafe fn interrupt_identification_register(&self) -> InterruptIdentificationRegister {
        let value = port_io::in_u8(self.0.interrupt_identification_register());
        InterruptIdentificationRegister::from(value)
    }
    unsafe fn set_line_control_register(&mut self, value : LineControlRegister) {
        port_io::out_u8(self.0.line_control_register(), value.into());
    }
    unsafe fn set_fifo_control_register(&mut self, value : FifoControlRegister) {
        port_io::out_u8(self.0.fifo_control_register(), value.into());
    }
    unsafe fn set_interrupt_enable_register(&mut self, value : InterruptEnableRegister) {
        port_io::out_u8(self.0.interrupt_enable_register(), value.into());
    }
    unsafe fn set_modem_control_register(&mut self, value : ModemControlRegister) {
        port_io::out_u8(self.0.modem_control_register(), value.into());
    }
}

impl Read for SerialPort {
    type Error = Error;

    fn read_exact(&mut self, buffer : &mut [u8]) -> Result<(), Self::Error> {
        unsafe {
            let mut status = self.line_status_register();
            for byte in buffer {
                while !status.data_ready() {
                    status = self.line_status_register();
                }
                *byte = port_io::in_u8(self.0.data_register());
                status = self.line_status_register();
                Result::<(), Self::Error>::from(status)?;
            }
        }

        Ok(())
    }
}

impl Write for SerialPort {
    type Error = Error;

    fn write(&mut self, buffer : &mut [u8]) -> Result<(), Self::Error> {
        unsafe {
            let mut status = self.line_status_register();
            for byte in buffer {
                while !status.empty_transmitter_holding_register() {
                    status = self.line_status_register();
                }
                port_io::out_u8(self.0.data_register(), *byte);
                status = self.line_status_register();
                Result::<(), Self::Error>::from(status)?;
            }
        }

        Ok(())
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s : &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_u8(byte)?;
        }
        Ok(())
    }
}