// *************************************************************************
// lib.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]

#[macro_use]
extern crate generation;


use x86::port_io;
use io::{ BinaryReader, BinaryWriter };
use core::fmt;
use bits::BitField;
use core::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PortNumber(u16);

impl PortNumber {
    pub const COM1 : PortNumber = PortNumber::new(0x3F8);
    pub const COM2 : PortNumber = PortNumber::new(0x2F8);
    pub const COM3 : PortNumber = PortNumber::new(0x3E8);
    pub const COM4 : PortNumber = PortNumber::new(0x2E8);

    pub const fn new(base_number : u16) -> Self {
        PortNumber(base_number)
    }

    pub const fn data_register(self) -> u16 {
        self.0
    }

    pub const fn interrupt_enabled_register(self) -> u16 {
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
        PortNumber::new(value)
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StopBits {
    One = 0,
    Two = 0x4
}

impl TryFrom<u8> for StopBits {
    type Error = ();

    fn try_from(value : u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(StopBits::One),
            0x4 => Ok(StopBits::Two),
            _ => Err(())
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WordLength {
    Five = 0,
    Six = 0x1,
    Seven = 0x2,
    Eight = 0x3
}

impl TryFrom<u8> for WordLength {
    type Error = ();

    fn try_from(value : u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WordLength::Five),
            0x1 => Ok(WordLength::Six),
            0x2 => Ok(WordLength::Seven),
            0x3 => Ok(WordLength::Eight),
            _ => Err(())
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Parity {
    None = 0,
    Odd = 0x8,
    Even = 0x18,
    Mark = 0x28,
    Space = 0x38
}

impl TryFrom<u8> for Parity {
    type Error = ();

    fn try_from(value : u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Parity::None),
            0x8 => Ok(Parity::Odd),
            0x18 => Ok(Parity::Even),
            0x28 => Ok(Parity::Mark),
            0x38 => Ok(Parity::Space),
            _ => Err(())
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
struct LineControlRegister(u8);

impl LineControlRegister {
    pub fn new() -> Self {
        LineControlRegister(0)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn word_length(&self) -> WordLength {
        WordLength::try_from(self.0 & 0x3).unwrap()
    }
    pub fn set_word_length(&mut self, word_length : WordLength) {
        self.0 &= 0xFC;
        self.0 |= word_length as u8;
    }

    pub fn stop_bits(&self) -> StopBits {
        StopBits::try_from(self.0 & 0x4).unwrap()
    }
    pub fn set_stop_bits(&mut self, stop_bits : StopBits) {
        self.0 &= 0xFB;
        self.0 |= stop_bits as u8;
    }

    pub fn parity(&self) -> Parity {
        Parity::try_from(self.0 & 0x38).unwrap()
    }
    pub fn set_parity(&mut self, parity : Parity) {
        self.0 &= 0xC7;
        self.0 |= parity as u8;
    }

    pub fn divisor_latch_access_enabled(&self) -> bool {
        self.0.is_bit_set(7)
    }
    pub fn set_divisor_latch_access_enabled(&mut self, value : bool) {
        self.0.set_bit(7, value);
    }
}

impl From<u8> for LineControlRegister {
    fn from(value : u8) -> Self {
        LineControlRegister(value)
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
struct LineStatusRegister(u8);

impl LineStatusRegister {
    pub fn data_ready(&self) -> bool {
        self.0.is_bit_set(0)
    }

    pub fn overrun_error(&self) -> bool {
        self.0.is_bit_set(1)
    }

    pub fn parity_error(&self) -> bool {
        self.0.is_bit_set(2)
    }

    pub fn framing_error(&self) -> bool {
        self.0.is_bit_set(3)
    }

    pub fn break_interrupt(&self) -> bool {
        self.0.is_bit_set(4)
    }

    pub fn empty_transmitter_holding_register(&self) -> bool {
        self.0.is_bit_set(5)
    }

    pub fn empty_data_holding_register(&self) -> bool {
        self.0.is_bit_set(6)
    }

    pub fn fifo_error(&self) -> bool {
        self.0.is_bit_set(7)
    }
}

impl From<u8> for LineStatusRegister {
    fn from(value : u8) -> Self {
        LineStatusRegister(value)
    }
}


c_enum!(
    pub enum BaudDivisor : u16 {
        RATE_50 = 2304;
        RATE_75 = 1536;
        RATE_110 = 1047;
        RATE_134 = 857;
        RATE_150 = 768;
        RATE_220 = 524;
        RATE_300 = 384;
        RATE_600 = 192;
        RATE_1200 = 96;
        RATE_1800 = 64;
        RATE_2000 = 58;
        RATE_2400 = 48;
        RATE_3600 = 32;
        RATE_4800 = 24;
        RATE_7200 = 16;
        RATE_9600 = 12;
        RATE_14400 = 8;
        RATE_19200 = 6;
        RATE_38400 = 3;
        RATE_57600 = 2;
        RATE_115200 = 1;
    }
);


#[derive(Clone, Debug)]
pub struct SerialPortSettings {
    baud_divisor : BaudDivisor,
    word_length : WordLength,
    stop_bits : StopBits,
    parity : Parity,
    fifo : bool
}

impl SerialPortSettings {
    pub fn baud_divisor(&self) -> BaudDivisor {
        self.baud_divisor
    }
    pub fn set_baud_divisor(&mut self, value : BaudDivisor) {
        self.baud_divisor = value;
    }

    pub fn word_length(&self) -> WordLength {
        self.word_length
    }
    pub fn set_word_length(&mut self, value : WordLength) {
        self.word_length = value;
    }

    pub fn stop_bits(&self) -> StopBits {
        self.stop_bits
    }
    pub fn set_stop_bits(&mut self, value : StopBits) {
        self.stop_bits = value;
    }

    pub fn parity(&self) -> Parity {
        self.parity
    }
    pub fn set_parity(&mut self, value : Parity) {
        self.parity = value;
    }
}

impl Default for SerialPortSettings {
    fn default() -> Self {
        SerialPortSettings {
            baud_divisor : BaudDivisor::RATE_9600,
            word_length : WordLength::Eight,
            stop_bits : StopBits::One,
            parity : Parity::None,
            fifo : true
        }     
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SerialPortError {
    InvalidBaudDivisor
    //TODO Other errors.
}

impl From<SerialPortError> for fmt::Error {
    fn from(_ : SerialPortError) -> Self {
        fmt::Error
    }
}


#[derive(Clone, Debug)]
pub struct SerialPort(PortNumber);

impl SerialPort {
    pub const unsafe fn new(port_number : PortNumber) -> Self {
        SerialPort(port_number)
    }

    //TODO FIFO and interrupts.

    pub fn configure(&mut self, settings : SerialPortSettings) -> Result<(), SerialPortError> {
        unsafe {
            let divisor = settings.baud_divisor().value();

            if divisor == 0 {
                return Err(SerialPortError::InvalidBaudDivisor);
            }

            port_io::out_u8(self.0.interrupt_enabled_register(), 0);

            let mut register = LineControlRegister::new();

            register.set_divisor_latch_access_enabled(true);
            self.set_line_control_register(register);

            port_io::out_u8(self.0.lower_divisor_latch_register(), divisor as u8);
            port_io::out_u8(self.0.higher_divisor_latch_register(), (divisor >> 8) as u8);

            register.set_word_length(settings.word_length());
            register.set_stop_bits(settings.stop_bits());
            register.set_parity(settings.parity());
            register.set_divisor_latch_access_enabled(false);

            self.set_line_control_register(register);

            port_io::out_u8(self.0.fifo_control_register(), 0xC7);
            port_io::out_u8(self.0.modem_control_register(), 0x0B);

            Ok(())
        }
    }

    pub fn current_settings(&self) -> SerialPortSettings {
        unsafe {
            let mut settings = SerialPortSettings::default();
            let mut register = self.line_control_register();
            
            settings.set_word_length(register.word_length());
            settings.set_stop_bits(register.stop_bits());
            settings.set_parity(register.parity());

            register.set_divisor_latch_access_enabled(true);
            self.set_line_control_register(register);

            let lower = port_io::in_u8(self.0.lower_divisor_latch_register()) as u16;
            let higher = (port_io::in_u8(self.0.higher_divisor_latch_register()) as u16) << 8;

            settings.set_baud_divisor(BaudDivisor::new(lower | higher));

            register.set_divisor_latch_access_enabled(false);
            self.set_line_control_register(register);

            settings
        }
    }

    pub fn sending_data(&self) -> bool {
        unsafe {
            !self.line_status_register().empty_transmitter_holding_register()
        }
    }
    pub fn data_recieved(&self) -> bool {
        unsafe {
            self.line_status_register().data_ready()
        }
    }

    unsafe fn line_status_register(&self) -> LineStatusRegister {
        let value = port_io::in_u8(self.0.line_status_register());
        LineStatusRegister::from(value)     
    }
    unsafe fn line_control_register(&self) -> LineControlRegister {
        let value = port_io::in_u8(self.0.line_control_register());
        LineControlRegister::from(value)    
    }
    unsafe fn set_line_control_register(&self, line_control_register : LineControlRegister) {
        port_io::out_u8(self.0.line_control_register(), line_control_register.as_u8());
    }
}

impl BinaryReader for SerialPort {
    type Error = SerialPortError;

    fn read_exact(&mut self, buffer : &mut [u8]) -> Result<(), Self::Error> {
        unsafe {
            for byte in buffer {
                while !self.data_recieved() { }
                *byte = port_io::in_u8(self.0.data_register());
            }
        }

        Ok(())
    }
}

impl BinaryWriter for SerialPort {
    type Error = SerialPortError;

    fn write(&mut self, buffer : &mut [u8]) -> Result<(), Self::Error> {
        unsafe {
            for byte in buffer {
                while self.sending_data() { }
                port_io::out_u8(self.0.data_register(), *byte);
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