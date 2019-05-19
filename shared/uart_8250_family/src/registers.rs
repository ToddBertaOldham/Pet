// *************************************************************************
// registers.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::Error;
use super::settings::{ Parity, StopBits, WordLength, FifoMode };
use core::convert::TryFrom;
use bits::BitField;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct LineControlRegister(u8);

impl LineControlRegister {
    pub fn new() -> Self {
        LineControlRegister(0)
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

impl From<LineControlRegister> for u8 {
    fn from(value : LineControlRegister) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct LineStatusRegister(u8);

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct FifoControlRegister(u8);

impl FifoControlRegister {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn mode(&self) -> FifoMode {
        FifoMode::try_from(self.0 & 0xE1).unwrap()
    }
    pub fn set_fifo_mode(&mut self, fifo_mode : FifoMode) {
        self.0 &= 0x1E;
        self.0 |= fifo_mode as u8;
    }

    pub fn clear_receive(&self) -> bool {
        self.0.is_bit_set(1)
    }
    pub fn set_clear_receive(&mut self, value : bool) {
        self.0.set_bit(1, value);
    }

    pub fn clear_transmit(&self) -> bool {
        self.0.is_bit_set(2)
    }
    pub fn set_clear_transmit(&mut self, value : bool) {
        self.0.set_bit(2, value);
    }

    pub fn dma_enabled(&self) -> bool {
        self.0.is_bit_set(2)
    }
    pub fn set_dma_enabled(&mut self, value : bool) {
        self.0.set_bit(2, value);
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct InterruptIdentificationRegister(u8);

impl InterruptIdentificationRegister {
    pub fn interrupt_event(&self) -> InterruptEvent {
        InterruptEvent::try_from(self.0 & 0xF).unwrap()
    }

    pub fn expanded_fifo_enabled(&self) -> bool {
        self.0.is_bit_set(5) 
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct InterruptEnableRegister(u8);

impl InterruptEnableRegister {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn recieved_data_available_interrupt(&self) -> bool {
        self.0.is_bit_set(0)
    }
    pub fn set_recieved_data_available_interrupt(&mut self, value : bool) {
        self.0.set_bit(0, value);
    }

    pub fn transmitter_holding_register_empty_interrupt(&self) -> bool {
        self.0.is_bit_set(1)
    }
    pub fn set_transmitter_holding_register_empty_interrupt(&mut self, value : bool) {
        self.0.set_bit(1, value);
    }

    pub fn line_status_interrupt(&self) -> bool {
        self.0.is_bit_set(2)
    }
    pub fn set_line_status_interrupt(&mut self, value : bool) {
        self.0.set_bit(2, value);
    }

    pub fn modem_status_interrupt(&self) -> bool {
        self.0.is_bit_set(3)
    }
    pub fn set_modem_status_interrupt(&mut self, value : bool) {
        self.0.set_bit(3, value);
    }

    pub fn sleep_mode_enabled(&self) -> bool {
        self.0.is_bit_set(4)
    }
    pub fn set_sleep_mode_enabled(&mut self, value : bool) {
        self.0.set_bit(4, value);
    }
    
    pub fn low_power_mode_enabled(&self) -> bool {
        self.0.is_bit_set(5)
    }
    pub fn set_low_power_mode_enabled(&mut self, value : bool) {
        self.0.set_bit(5, value);
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ModemControlRegister(u8);

impl ModemControlRegister {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn data_terminal_ready(&self) -> bool {
        self.0.is_bit_set(0)
    }
    pub fn set_data_terminal_ready(&mut self, value : bool) {
        self.0.set_bit(0, value);
    }

    pub fn request_to_send(&self) -> bool {
        self.0.is_bit_set(1)
    }
    pub fn set_request_to_send(&mut self, value : bool) {
        self.0.set_bit(1, value);
    }

    pub fn auxillary_output_1(&self) -> bool {
        self.0.is_bit_set(2)
    }
    pub fn set_auxillary_output_1(&mut self, value : bool) {
        self.0.set_bit(2, value);
    }

    pub fn auxillary_output_2(&self) -> bool {
        self.0.is_bit_set(3)
    }
    pub fn set_auxillary_output_2(&mut self, value : bool) {
        self.0.set_bit(3, value);
    }

    pub fn loopback_mode(&self) -> bool {
        self.0.is_bit_set(4)
    }
    pub fn set_loopback_mode(&mut self, value : bool) {
        self.0.set_bit(4, value);
    }

    pub fn autoflow_control_enabled(&self) -> bool {
        self.0.is_bit_set(5)
    }
    pub fn set_autoflow_control_enabled(&mut self, value : bool) {
        self.0.set_bit(5, value);
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ModemStatusRegister(u8);

impl ModemStatusRegister {
    pub fn delta_clear_to_send(&self) -> bool {
        self.0.is_bit_set(0)
    }

    pub fn delta_data_set_ready(&self) -> bool {
        self.0.is_bit_set(1)
    }

    pub fn trailing_edge_ring_indicator(&self) -> bool {
        self.0.is_bit_set(2)
    }

    pub fn delta_data_carrier_detect(&self) -> bool {
        self.0.is_bit_set(3)
    }

    pub fn clear_to_send(&self) -> bool {
        self.0.is_bit_set(4)
    }

    pub fn data_set_ready(&self) -> bool {
        self.0.is_bit_set(5)
    }

    pub fn ring_indicator(&self) -> bool {
        self.0.is_bit_set(6)
    }

    pub fn carrier_detect(&self) -> bool {
        self.0.is_bit_set(7)
    }
}

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
