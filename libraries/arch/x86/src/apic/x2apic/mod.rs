//**************************************************************************************************
// x2apic.rs                                                                                       *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

mod id;
mod ipi;

pub use id::*;
pub use ipi::*;

use crate::apic::local::{CommonRegisters, TaskPriority};
use crate::Msr;
use memory::split::Halves;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Registers;

impl Registers {
    pub unsafe fn write_self_ipi(&mut self, ipi: Ipi) {
        Msr::new(0x83F).write(ipi.into())
    }
}

impl CommonRegisters for Registers {
    type Id = Id;
    type Ipi = Ipi;

    unsafe fn read_id_register(&self) -> Self::Id {
        Msr::new(0x802).read().lower_half().into()
    }

    unsafe fn read_version_register(&self) -> u32 {
        Msr::new(0x803).read().lower_half()
    }

    unsafe fn read_tpr(&self) -> TaskPriority {
        Msr::new(0x808).read().lower_half().into()
    }

    unsafe fn write_tpr(&mut self, task_priority: TaskPriority) {
        let value = u64::from_halves(task_priority.into(), 0);
        Msr::new(0x808).write(value);
    }

    unsafe fn read_ppr(&self) -> u32 {
        Msr::new(0x80A).read().lower_half()
    }

    unsafe fn write_eoi_register(&mut self) {
        Msr::new(0x80B).write(0);
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
        let ipi_value = Msr::new(0x830).read();
        Ipi::from(ipi_value)
    }

    unsafe fn write_icr(&mut self, ipi: Self::Ipi) {
        Msr::new(0x830).write(ipi.into());
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
