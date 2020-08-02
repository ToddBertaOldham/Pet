//**************************************************************************************************
// settings.rs                                                                                     *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub use crate::registers::{BaudDivisor, FifoMode, Parity, StopBits, WordLength};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Settings {
    baud_divisor: BaudDivisor,
    word_length: WordLength,
    stop_bits: StopBits,
    parity: Parity,
    fifo_mode: FifoMode,
    data_received_interrupt: bool,
    transmitter_empty_interrupt: bool,
    line_status_interrupt: bool,
    modem_status_interrupt: bool,
}

impl Settings {
    pub fn baud_divisor(&self) -> BaudDivisor {
        self.baud_divisor
    }

    pub fn set_baud_divisor(&mut self, value: BaudDivisor) {
        self.baud_divisor = value;
    }

    pub fn word_length(&self) -> WordLength {
        self.word_length
    }

    pub fn set_word_length(&mut self, value: WordLength) {
        self.word_length = value;
    }

    pub fn stop_bits(&self) -> StopBits {
        self.stop_bits
    }

    pub fn set_stop_bits(&mut self, value: StopBits) {
        self.stop_bits = value;
    }

    pub fn parity(&self) -> Parity {
        self.parity
    }

    pub fn set_parity(&mut self, value: Parity) {
        self.parity = value;
    }

    pub fn fifo_mode(&self) -> FifoMode {
        self.fifo_mode
    }

    pub fn set_fifo_mode(&mut self, value: FifoMode) {
        self.fifo_mode = value;
    }

    pub fn data_received_interrupt(&self) -> bool {
        self.data_received_interrupt
    }

    pub fn set_data_received_interrupt(&mut self, value: bool) {
        self.data_received_interrupt = value;
    }

    pub fn transmitter_empty_interrupt(&self) -> bool {
        self.transmitter_empty_interrupt
    }

    pub fn set_transmitter_empty_interrupt(&mut self, value: bool) {
        self.transmitter_empty_interrupt = value;
    }

    pub fn line_status_interrupt(&self) -> bool {
        self.line_status_interrupt
    }

    pub fn set_line_status_interrupt(&mut self, value: bool) {
        self.line_status_interrupt = value;
    }

    pub fn modem_status_interrupt(&self) -> bool {
        self.modem_status_interrupt
    }

    pub fn set_modem_status_interrupt(&mut self, value: bool) {
        self.modem_status_interrupt = value;
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            baud_divisor: BaudDivisor::RATE_9600,
            word_length: WordLength::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
            fifo_mode: FifoMode::FourteenBytes,
            data_received_interrupt: false,
            transmitter_empty_interrupt: false,
            line_status_interrupt: false,
            modem_status_interrupt: false,
        }
    }
}
