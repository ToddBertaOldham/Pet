//**************************************************************************************************
// serial_port.rs                                                                                  *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub use crate::registers::Port;
use crate::registers::{
    FifoControlValue, InterruptEnableValue, InterruptEvent, LineControlValue, ModemControlValue,
};
use crate::{Error, Settings};
use core::fmt;
use io::{EndianWrite, Read, Write};

#[derive(Clone, Debug)]
pub struct SerialPort(Port);

impl SerialPort {
    pub const unsafe fn new(port: Port) -> Self {
        SerialPort(port)
    }

    pub fn configure(&mut self, settings: Settings) -> Result<(), Error> {
        unsafe {
            if u16::from(settings.baud_divisor()) == 0 {
                return Err(Error::InvalidBaudDivisor);
            }

            if self.is_line_busy() {
                return Err(Error::LineBusy);
            }

            let mut interrupt_value = InterruptEnableValue::new();

            // Disable interrupts first.

            self.0.write_interrupt_enable_register(interrupt_value);

            let mut line_value = LineControlValue::new();
            line_value.set_divisor_latch_access_enabled(true);

            self.0.write_line_control_register(line_value);

            self.0.write_divisor_latch_register(settings.baud_divisor());

            line_value.set_word_length(settings.word_length());
            line_value.set_stop_bits(settings.stop_bits());
            line_value.set_parity(settings.parity());
            line_value.set_divisor_latch_access_enabled(false);
            self.0.write_line_control_register(line_value);

            let mut fifo_value = FifoControlValue::new();
            fifo_value.set_fifo_mode(settings.fifo_mode());
            fifo_value.set_clear_receive(true);
            fifo_value.set_clear_transmit(true);
            self.0.write_fifo_control_register(fifo_value);

            let mut modem_value = ModemControlValue::new();
            modem_value.set_data_terminal_ready(true);
            modem_value.set_request_to_send(true);
            modem_value.set_auxiliary_output_2(true);
            self.0.write_modem_control_register(modem_value);

            interrupt_value.set_data_received_interrupt(settings.data_received_interrupt());
            interrupt_value.set_transmitter_empty_interrupt(settings.transmitter_empty_interrupt());
            interrupt_value.set_line_status_interrupt(settings.line_status_interrupt());
            interrupt_value.set_modem_status_interrupt(settings.modem_status_interrupt());
            self.0.write_interrupt_enable_register(interrupt_value);

            Ok(())
        }
    }

    pub fn is_line_busy(&self) -> bool {
        unsafe {
            let status = self.0.read_line_status_register();
            !status.receiver_empty() || status.data_ready()
        }
    }

    pub fn check_for_error(&self) -> Result<(), Error> {
        unsafe { Result::<(), Error>::from(self.0.read_line_status_register()) }
    }

    pub fn interrupt_event(&self) -> InterruptEvent {
        unsafe { self.0.read_interrupt_id_register().interrupt_event() }
    }
}

impl Read for SerialPort {
    type Error = Error;

    fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        unsafe {
            let mut status = self.0.read_line_status_register();
            for byte in buffer {
                while !status.data_ready() {
                    status = self.0.read_line_status_register();
                }
                *byte = self.0.read_receiver_register();
                status = self.0.read_line_status_register();
                Result::<(), Self::Error>::from(status)?;
            }
        }

        Ok(())
    }
}

impl Write for SerialPort {
    type Error = Error;

    fn write(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        unsafe {
            let mut status = self.0.read_line_status_register();
            for byte in buffer {
                while !status.transmitter_empty() {
                    status = self.0.read_line_status_register();
                }
                self.0.write_transmitter_register(*byte);
                status = self.0.read_line_status_register();
                Result::<(), Self::Error>::from(status)?;
            }
        }

        Ok(())
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_u8(byte)?;
        }
        Ok(())
    }
}
