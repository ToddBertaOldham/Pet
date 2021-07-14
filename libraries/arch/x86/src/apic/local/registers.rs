//**************************************************************************************************
// registers.rs                                                                                    *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::apic::local::{CommonRegisters, DivideValue, Ipi, TimerLvt};
use memory::split::Halves;
use memory::GetBit;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Registers {
    address: *mut u8,
}

impl Registers {
    const ID_REGISTER: usize = 0x020;
    const ICR_LOWER: usize = 0x300;
    const ICR_UPPER: usize = 0x310;
    const INITIAL_COUNT_REGISTER: usize = 0x380;
    const CURRENT_COUNT_REGISTER: usize = 0x390;
    const DCR_REGISTER: usize = 0x3E0;

    pub fn new(address: *mut u8) -> Self {
        Self { address }
    }

    unsafe fn get_register(&self, offset: usize) -> *mut u32 {
        self.address.add(offset) as *mut u32
    }

    pub fn write_local_destination(&mut self, value: u32) {
        unimplemented!()
    }
}

impl CommonRegisters for Registers {
    type Id = u8;
    type Ipi = Ipi;

    unsafe fn read_id_register(&self) -> Self::Id {
        let value = *self.get_register(Self::ID_REGISTER);
        value.get_bits(24, 0, 8) as u8
    }

    unsafe fn read_version_register(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn read_tpr(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_tpr(&mut self, value: u32) {
        unimplemented!()
    }

    unsafe fn read_ppr(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_eoi_register(&mut self) {
        unimplemented!()
    }

    unsafe fn read_ldr(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn read_svr(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_svr(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn read_isr(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn read_tmr(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn read_irr(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn read_esr(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_esr(&self) {
        unimplemented!()
    }

    unsafe fn read_lvt_cmci_register(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_lvt_cmci_register(&self) {
        unimplemented!()
    }

    unsafe fn read_icr(&self) -> Self::Ipi {
        let lower = *self.get_register(Self::ICR_LOWER);
        let upper = *self.get_register(Self::ICR_UPPER);

        let ipi_value = u64::from_halves(lower, upper);
        Ipi::from(ipi_value)
    }

    unsafe fn write_icr(&mut self, value: Self::Ipi) {
        let inner_value: u64 = value.into();

        *self.get_register(Self::ICR_LOWER) = inner_value.lower_half();
        *self.get_register(Self::ICR_UPPER) = inner_value.upper_half();
    }

    unsafe fn read_lvt_time_register(&self) -> TimerLvt {
        unimplemented!()
    }

    unsafe fn write_lvt_time_register(&mut self, lvt: TimerLvt) {
        unimplemented!()
    }

    unsafe fn read_lvt_thermal_sensor_register(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_lvt_thermal_sensor_register(&self) {
        unimplemented!()
    }

    unsafe fn read_lvt_perf_monitor_register(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_lvt_perf_monitor_register(&self) {
        unimplemented!()
    }

    unsafe fn read_lvt_lint0_register(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_lvt_lint0_register(&self) {
        unimplemented!()
    }

    unsafe fn read_lvt_lint1_register(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_lvt_lint1_register(&self) {
        unimplemented!()
    }

    unsafe fn read_initial_count_register(&self) -> u32 {
        *self.get_register(Self::INITIAL_COUNT_REGISTER)
    }

    unsafe fn write_initial_count_register(&mut self, value: u32) {
        *self.get_register(Self::INITIAL_COUNT_REGISTER) = value;
    }

    unsafe fn read_current_count_register(&self) -> u32 {
        *self.get_register(Self::CURRENT_COUNT_REGISTER)
    }

    unsafe fn read_dcr(&self) -> DivideValue {
        (*self.get_register(Self::DCR_REGISTER)).into()
    }

    unsafe fn write_dcr(&mut self, value: DivideValue) {
        *self.get_register(Self::DCR_REGISTER) = value.into();
    }
}
