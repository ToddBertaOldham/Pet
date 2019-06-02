// *************************************************************************
// registers.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::Error;
use super::settings::{ Parity, StopBits, WordLength, FifoMode };
use core::convert::TryFrom;
use encapsulation::BitGetterSetters;

#[derive(Clone, Copy, Debug, PartialEq, Eq, BitGetterSetters)]
#[repr(transparent)]
pub struct LineControlRegister(
    #[bit_access(name = "divisor_latch_access_enabled", set = true, index = 7, borrow_self = false)]
    u8);

impl LineControlRegister {
    pub fn new() -> Self {
        LineControlRegister(0)
    }

    pub fn word_length(self) -> WordLength {
        WordLength::try_from(self.0 & 0x3).unwrap()
    }
    pub fn set_word_length(&mut self, word_length : WordLength) {
        self.0 &= 0xFC;
        self.0 |= word_length as u8;
    }

    pub fn stop_bits(self) -> StopBits {
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
}

impl From<u8> for LineControlRegister {
    fn from(value : u8) -> Self {
        LineControlRegister(value)
    }
}

impl From<LineControlRegister> for u8 {
    fn from(value : LineControlRegister) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, BitGetterSetters)]
#[repr(transparent)]
pub struct LineStatusRegister(
    #[bit_access(name = "data_ready", index = 0, borrow_self = false)]
    #[bit_access(name = "overrun_error", index = 1, borrow_self = false)]
    #[bit_access(name = "parity_error", index = 2, borrow_self = false)]
    #[bit_access(name = "framing_error", index = 3, borrow_self = false)]
    #[bit_access(name = "break_interrupt", index = 4, borrow_self = false)]
    #[bit_access(name = "empty_transmitter_holding_register", index = 5, borrow_self = false)]
    #[bit_access(name = "empty_data_holding_register", index = 6, borrow_self = false)]
    #[bit_access(name = "fifo_error", index = 7, borrow_self = false)]
    u8);

impl From<u8> for LineStatusRegister {
    fn from(value : u8) -> Self {
        LineStatusRegister(value)
    }
}

impl From<LineStatusRegister> for u8 {
    fn from(value : LineStatusRegister) -> Self {
        value.0
    }
}

impl From<LineStatusRegister> for Result<(), Error> {
    fn from(value : LineStatusRegister) -> Self {
        if value.framing_error() {
            return Err(Error::InvalidFraming);
        }

        if value.parity_error() {
            return Err(Error::InvalidParity);
        }

        if value.overrun_error() {
            return Err(Error::Overrun);     
        }

        if value.fifo_error() {
            return Err(Error::FifoInvalid);
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, BitGetterSetters)]
#[repr(transparent)]
pub struct FifoControlRegister(
    #[bit_access(name = "clear_receive", set = true, index = 1, borrow_self = false)]
    #[bit_access(name = "clear_transmit", set = true, index = 2, borrow_self = false)]
    #[bit_access(name = "dma_enabled", set = true, index = 3, borrow_self = false)]
    u8);

impl FifoControlRegister {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn fifo_mode(&self) -> FifoMode {
        FifoMode::try_from(self.0 & 0xE1).unwrap()
    }
    pub fn set_fifo_mode(&mut self, fifo_mode : FifoMode) {
        self.0 &= 0x1E;
        self.0 |= fifo_mode as u8;
    }
}

impl From<u8> for FifoControlRegister {
    fn from(value : u8) -> Self {
        FifoControlRegister(value)
    }
}

impl From<FifoControlRegister> for u8 {
    fn from(value : FifoControlRegister) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterruptEvent {
    None = 0x0,
    ModemStatus = 0x1,
    TransmitterHoldingRegisterEmpty = 0x3,
    RecievedDataAvailable = 0x5,
    LineStatus = 0x7,
    TimeOut = 0xD
}

impl TryFrom<u8> for InterruptEvent {
    type Error = ();

    fn try_from(value : u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(InterruptEvent::None),
            0x1 => Ok(InterruptEvent::ModemStatus),
            0x3 => Ok(InterruptEvent::TransmitterHoldingRegisterEmpty),
            0x5 => Ok(InterruptEvent::RecievedDataAvailable),
            0x7 => Ok(InterruptEvent::LineStatus),
            0xD => Ok(InterruptEvent::TimeOut),
            _ => Err(())
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoState {
    NotEnabled = 0,
    NotFunctioning = 0x80,
    Functioning = 0xC0
}

impl TryFrom<u8> for FifoState {
    type Error = ();

    fn try_from(value : u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(FifoState::NotEnabled),
            0x80 => Ok(FifoState::NotFunctioning),
            0xC0 => Ok(FifoState::Functioning),
            _ => Err(())
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, BitGetterSetters)]
#[repr(transparent)]
pub struct InterruptIdentificationRegister(
    #[bit_access(name = "expanded_fifo_enabled", index = 5, borrow_self = false)]
    u8);

impl InterruptIdentificationRegister {
    pub fn interrupt_event(&self) -> InterruptEvent {
        InterruptEvent::try_from(self.0 & 0xF).unwrap()
    }

    pub fn fifo_state(&self) -> FifoState {
        FifoState::try_from(self.0 & 0xC0).unwrap()
    }
}

impl From<u8> for InterruptIdentificationRegister {
    fn from(value : u8) -> Self {
        InterruptIdentificationRegister(value)
    }
}

impl From<InterruptIdentificationRegister> for u8 {
    fn from(value : InterruptIdentificationRegister) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, BitGetterSetters)]
#[repr(transparent)]
pub struct InterruptEnableRegister(
    #[bit_access(name = "recieved_data_available_interrupt", set = true, index = 0, borrow_self = false)]
    #[bit_access(name = "transmitter_holding_register_empty_interrupt", set = true, index = 1, borrow_self = false)]
    #[bit_access(name = "line_status_interrupt", set = true, index = 2, borrow_self = false)]
    #[bit_access(name = "modem_status_interrupt", set = true, index = 3, borrow_self = false)]
    #[bit_access(name = "sleep_mode_enabled", set = true, index = 4, borrow_self = false)]
    #[bit_access(name = "low_power_mode_enabled", set = true, index = 5, borrow_self = false)]
    u8);

impl InterruptEnableRegister {
    pub fn new() -> Self {
        Self(0)
    }
}

impl From<u8> for InterruptEnableRegister {
    fn from(value : u8) -> Self {
        InterruptEnableRegister(value)
    }
}

impl From<InterruptEnableRegister> for u8 {
    fn from(value : InterruptEnableRegister) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, BitGetterSetters)]
#[repr(transparent)]
pub struct ModemControlRegister(
    #[bit_access(name = "data_terminal_ready", set = true, index = 0, borrow_self = false)]
    #[bit_access(name = "request_to_send", set = true, index = 1, borrow_self = false)]
    #[bit_access(name = "auxillary_output_1", set = true, index = 2, borrow_self = false)]
    #[bit_access(name = "auxillary_output_2", set = true, index = 3, borrow_self = false)]
    #[bit_access(name = "loopback_mode", set = true, index = 4, borrow_self = false)]
    #[bit_access(name = "autoflow_control_enabled", set = true, index = 5, borrow_self = false)]
    u8);

impl ModemControlRegister {
    pub fn new() -> Self {
        Self(0)
    }
}

impl From<u8> for ModemControlRegister {
    fn from(value : u8) -> Self {
        ModemControlRegister(value)
    }
}

impl From<ModemControlRegister> for u8 {
    fn from(value : ModemControlRegister) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, BitGetterSetters)]
#[repr(transparent)]
pub struct ModemStatusRegister(
    #[bit_access(name = "delta_clear_to_send", index = 0, borrow_self = false)]
    #[bit_access(name = "delta_data_set_ready", index = 1, borrow_self = false)]
    #[bit_access(name = "trailing_edge_ring_indicator", index = 2, borrow_self = false)]
    #[bit_access(name = "delta_data_carrier_detect", index = 3, borrow_self = false)]
    #[bit_access(name = "clear_to_send", index = 4, borrow_self = false)]
    #[bit_access(name = "data_set_ready", index = 5, borrow_self = false)]
    #[bit_access(name = "ring_indicator", index = 6, borrow_self = false)]
    #[bit_access(name = "carrier_detect", index = 7, borrow_self = false)]
    u8);

impl From<u8> for ModemStatusRegister {
    fn from(value : u8) -> Self {
        ModemStatusRegister(value)
    }
}

impl From<ModemStatusRegister> for u8 {
    fn from(value : ModemStatusRegister) -> Self {
        value.0
    }
}
