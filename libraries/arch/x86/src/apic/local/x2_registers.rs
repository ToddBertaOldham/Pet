//**************************************************************************************************
// x2_registers.rs                                                                                 *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::apic::local::{CommonRegisters, DivideValue, TimerLvt, X2Ipi};
use crate::msr::Msr;
use memory::split::Halves;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct X2Registers;

impl X2Registers {
    const ID_REGISTER: Msr = Msr::new(0x802);
    const VERSION_REGISTER: Msr = Msr::new(0x803);
    const ICR: Msr = Msr::new(0x830);
    const LVT_TIME_REGISTER: Msr = Msr::new(0x832);
    const INITIAL_COUNT_REGISTER: Msr = Msr::new(0x838);
    const CURRENT_COUNT_REGISTER: Msr = Msr::new(0x839);
    const DCR_REGISTER: Msr = Msr::new(0x83E);

    pub unsafe fn write_self_ipi(&mut self, ipi: X2Ipi) {
        Msr::new(0x83F).write(ipi.into())
    }
}

impl CommonRegisters for X2Registers {
    type Id = u32;
    type Ipi = X2Ipi;

    unsafe fn read_id_register(&self) -> Self::Id {
        Self::ID_REGISTER.read().lower_half()
    }

    unsafe fn read_version_register(&self) -> u32 {
        Self::VERSION_REGISTER.read().lower_half()
    }

    unsafe fn read_tpr(&self) -> u32 {
        unimplemented!()
    }

    unsafe fn write_tpr(&mut self, task_priority: u32) {
        unimplemented!()
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
        Self::ICR.read().into()
    }

    unsafe fn write_icr(&mut self, ipi: Self::Ipi) {
        Self::ICR.write(ipi.into());
    }

    unsafe fn read_lvt_time_register(&self) -> TimerLvt {
        Self::LVT_TIME_REGISTER.read().lower_half().into()
    }

    unsafe fn write_lvt_time_register(&mut self, value: TimerLvt) {
        let inner_value = u32::from(value) as u64;
        Self::LVT_TIME_REGISTER.write(inner_value);
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
        Self::INITIAL_COUNT_REGISTER.read().lower_half()
    }

    unsafe fn write_initial_count_register(&mut self, value: u32) {
        Self::INITIAL_COUNT_REGISTER.write(value as u64);
    }

    unsafe fn read_current_count_register(&self) -> u32 {
        Self::CURRENT_COUNT_REGISTER.read().lower_half()
    }

    unsafe fn read_dcr(&self) -> DivideValue {
        Self::DCR_REGISTER.read().lower_half().into()
    }

    unsafe fn write_dcr(&mut self, value: DivideValue) {
        let inner_value = u32::from(value) as u64;
        Self::DCR_REGISTER.write(inner_value);
    }
}
