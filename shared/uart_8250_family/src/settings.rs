// *************************************************************************
// settings.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoMode {
    Disabled = 0x0,
    OneByte = 0x1,
    FourBytes = 0x41,
    EightBytes = 0x81,
    FourteenBytes = 0xC1,
    SixteenBytes = 0x61,
    ThirtyTwoBytes = 0xA1,
    FiftySixBytes = 0xE1
}

impl TryFrom<u8> for FifoMode {
    type Error = ();

    fn try_from(value : u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(FifoMode::Disabled),
            0x1 => Ok(FifoMode::OneByte),
            0x41 => Ok(FifoMode::FourBytes),
            0x81 => Ok(FifoMode::EightBytes),
            0xC1 => Ok(FifoMode::FourteenBytes),
            0x61 => Ok(FifoMode::SixteenBytes),
            0xA1 => Ok(FifoMode::ThirtyTwoBytes),
            0xE1 => Ok(FifoMode::FiftySixBytes),
            _ => Err(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct Settings {
    baud_divisor : BaudDivisor,
    word_length : WordLength,
    stop_bits : StopBits,
    parity : Parity,
    fifo_mode : FifoMode,
    recieved_data_available_interrupt : bool,
    transmitter_holding_register_empty_interrupt : bool,
    line_status_interrupt : bool,
    modem_status_interrupt : bool
}

impl Settings {
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

    pub fn fifo_mode(&self) -> FifoMode {
        self.fifo_mode
    }
    pub fn set_fifo_mode(&mut self, value : FifoMode) {
        self.fifo_mode = value;
    }

    pub fn recieved_data_available_interrupt(&self) -> bool {
        self.recieved_data_available_interrupt
    }
    pub fn set_recieved_data_available_interrupt(&mut self, value : bool) {
        self.recieved_data_available_interrupt = value;
    }

    pub fn transmitter_holding_register_empty_interrupt(&self) -> bool {
        self.transmitter_holding_register_empty_interrupt
    }
    pub fn set_transmitter_holding_register_empty_interrupt(&mut self, value : bool) {
        self.transmitter_holding_register_empty_interrupt = value;
    }

    pub fn line_status_interrupt(&self) -> bool {
        self.line_status_interrupt
    }
    pub fn set_line_status_interrupt(&mut self, value : bool) {
        self.line_status_interrupt = value;
    }

    pub fn modem_status_interrupt(&self) -> bool {
        self.modem_status_interrupt
    }
    pub fn set_modem_status_interrupt_enabled(&mut self, value : bool) {
        self.modem_status_interrupt = value;
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            baud_divisor : BaudDivisor::RATE_9600,
            word_length : WordLength::Eight,
            stop_bits : StopBits::One,
            parity : Parity::None,
            fifo_mode : FifoMode::FourteenBytes,
            recieved_data_available_interrupt : false,
            transmitter_holding_register_empty_interrupt : false,
            line_status_interrupt : false,
            modem_status_interrupt : false   
        }     
    }
}