//**************************************************************************************************
// port.rs                                                                                         *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::registers::{
    BaudDivisor, FifoControlValue, InterruptEnableValue, InterruptIdValue, LineControlValue,
    LineStatusValue, ModemControlValue, ModemStatusValue,
};
use x86::IoPort;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Port(u16);

impl Port {
    pub const COM_1: Port = Port(0x3F8);

    pub const COM_2: Port = Port(0x2F8);

    pub const COM_3: Port = Port(0x3E8);

    pub const COM_4: Port = Port(0x2E8);

    pub const fn new(base_address: u16) -> Self {
        Port(base_address)
    }

    pub const fn base_address(self) -> u16 {
        self.0
    }

    // Receiver Register

    pub unsafe fn read_receiver_register(self) -> u8 {
        IoPort::new(self.0).in_u8()
    }

    // Transmitter Register

    pub unsafe fn write_transmitter_register(self, value: u8) {
        IoPort::new(self.0).out_u8(value)
    }

    // Interrupt Enable Register

    pub unsafe fn read_interrupt_enable_register(self) -> InterruptEnableValue {
        IoPort::new(self.0 + 1).in_u8().into()
    }

    pub unsafe fn write_interrupt_enable_register(self, value: InterruptEnableValue) {
        IoPort::new(self.0 + 1).out_u8(value.into())
    }

    // Divisor Latch Register

    pub unsafe fn read_divisor_latch_register(self) -> BaudDivisor {
        let mut value = IoPort::new(self.0).in_u8() as u16;
        value |= (IoPort::new(self.0 + 1).in_u8() as u16) >> 8;
        BaudDivisor::from(value)
    }

    pub unsafe fn write_divisor_latch_register(self, value: BaudDivisor) {
        let divisor = u16::from(value);
        IoPort::new(self.0).out_u8(divisor as u8);
        IoPort::new(self.0 + 1).out_u8((divisor >> 8) as u8);
    }

    // Interrupt Identification Register

    pub unsafe fn read_interrupt_id_register(self) -> InterruptIdValue {
        IoPort::new(self.0 + 2).in_u8().into()
    }

    // FIFO Control Register

    pub unsafe fn write_fifo_control_register(self, value: FifoControlValue) {
        IoPort::new(self.0 + 2).out_u8(value.into());
    }

    // Line Control Register

    pub unsafe fn read_line_control_register(self) -> LineControlValue {
        IoPort::new(self.0 + 3).in_u8().into()
    }

    pub unsafe fn write_line_control_register(self, value: LineControlValue) {
        IoPort::new(self.0 + 3).out_u8(value.into());
    }

    // Modem Control Register

    pub unsafe fn read_modem_control_register(self) -> ModemControlValue {
        IoPort::new(self.0 + 4).in_u8().into()
    }

    pub unsafe fn write_modem_control_register(self, value: ModemControlValue) {
        IoPort::new(self.0 + 4).out_u8(value.into());
    }

    // Line Status Register

    pub unsafe fn read_line_status_register(self) -> LineStatusValue {
        IoPort::new(self.0 + 5).in_u8().into()
    }

    pub unsafe fn read_modem_status_register(self) -> ModemStatusValue {
        IoPort::new(self.0 + 6).in_u8().into()
    }

    // Scratch Register

    pub unsafe fn read_scratch_register(self) -> u8 {
        IoPort::new(self.0 + 7).in_u8()
    }

    pub unsafe fn write_scratch_register(self, value: u8) {
        IoPort::new(self.0 + 7).out_u8(value);
    }
}

impl From<u16> for Port {
    fn from(value: u16) -> Self {
        Port(value)
    }
}

impl From<Port> for u16 {
    fn from(value: Port) -> Self {
        value.0
    }
}
