//**************************************************************************************************
// xapic.rs                                                                                        *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

mod id;
mod ipi;

pub use id::*;
pub use ipi::*;

use crate::apic::local::{CommonRegisters, TaskPriority};
use memory::split::Halves;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Registers {
    address: *mut u8,
}

impl Registers {
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
    type Id = Id;
    type Ipi = Ipi;

    unsafe fn read_id_register(&self) -> Self::Id {
        (*self.get_register(0x20)).into()
    }

    unsafe fn read_version_register(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn read_tpr(&self) -> TaskPriority {
        unimplemented!()
    }

    unsafe fn write_tpr(&mut self, value: TaskPriority) {
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
        let lower = *self.get_register(0x300);
        let upper = *self.get_register(0x310);

        let ipi_value = u64::from_halves(lower, upper);
        Ipi::from(ipi_value)
    }

    unsafe fn write_icr(&mut self, ipi: Self::Ipi) {
        let ipi_value: u64 = ipi.into();

        *self.get_register(0x300) = ipi_value.lower_half();
        *self.get_register(0x310) = ipi_value.upper_half();
    }

    unsafe fn read_lvt_time_register(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_lvt_time_register(&self) {
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
        unimplemented!()
    }

    unsafe fn write_initial_count_register(&self) {
        unimplemented!()
    }

    unsafe fn read_current_count_register(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_current_count_register(&self) {
        unimplemented!()
    }

    unsafe fn read_dcr(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_dcr(&self) {
        unimplemented!()
    }
}
