//**************************************************************************************************
// common_registers.rs                                                                             *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::apic::local::{DivideValue, Ipi, TimerLvt};

pub trait CommonRegisters {
    type Id;
    type Ipi;

    unsafe fn read_id_register(&self) -> Self::Id;

    unsafe fn read_version_register(&self) -> u32;

    unsafe fn read_tpr(&self) -> u32;

    unsafe fn write_tpr(&mut self, value: u32);

    unsafe fn read_ppr(&self) -> u32;

    unsafe fn write_eoi_register(&mut self);

    unsafe fn read_ldr(&self) -> u32;

    unsafe fn read_svr(&self) -> u32;

    unsafe fn write_svr(&self) -> u32;

    unsafe fn read_isr(&self) -> u32;

    unsafe fn read_tmr(&self) -> u32;

    unsafe fn read_irr(&self) -> u32;

    unsafe fn read_esr(&self) -> u32;

    unsafe fn write_esr(&self);

    unsafe fn read_lvt_cmci_register(&self) -> u32;

    unsafe fn write_lvt_cmci_register(&self);

    unsafe fn read_icr(&self) -> Self::Ipi;

    unsafe fn write_icr(&mut self, value: Self::Ipi);

    unsafe fn read_lvt_time_register(&self) -> TimerLvt;

    unsafe fn write_lvt_time_register(&mut self, value: TimerLvt);

    unsafe fn read_lvt_thermal_sensor_register(&self) -> u32;

    unsafe fn write_lvt_thermal_sensor_register(&self);

    unsafe fn read_lvt_perf_monitor_register(&self) -> u32;

    unsafe fn write_lvt_perf_monitor_register(&self);

    unsafe fn read_lvt_lint0_register(&self) -> u32;

    unsafe fn write_lvt_lint0_register(&self);

    unsafe fn read_lvt_lint1_register(&self) -> u32;

    unsafe fn write_lvt_lint1_register(&self);

    unsafe fn read_initial_count_register(&self) -> u32;

    unsafe fn write_initial_count_register(&mut self, value: u32);

    unsafe fn read_current_count_register(&self) -> u32;

    unsafe fn read_dcr(&self) -> DivideValue;

    unsafe fn write_dcr(&mut self, value: DivideValue);
}
