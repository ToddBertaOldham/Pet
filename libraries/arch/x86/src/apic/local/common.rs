//**************************************************************************************************
// common.rs                                                                                       *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::apic::local::TaskPriority;

pub trait CommonRegisters {
    type Id;
    type Ipi;

    unsafe fn read_id_register(&self) -> Self::Id;

    unsafe fn read_version_register(&self) -> u32;

    unsafe fn read_tpr(&self) -> TaskPriority;

    unsafe fn write_tpr(&mut self, value: TaskPriority);

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

    unsafe fn write_icr(&mut self, ipi: Self::Ipi);

    unsafe fn read_lvt_time_register(&self) -> u32;

    unsafe fn write_lvt_time_register(&self);

    unsafe fn read_lvt_thermal_sensor_register(&self) -> u32;

    unsafe fn write_lvt_thermal_sensor_register(&self);

    unsafe fn read_lvt_perf_monitor_register(&self) -> u32;

    unsafe fn write_lvt_perf_monitor_register(&self);

    unsafe fn read_lvt_lint0_register(&self) -> u32;

    unsafe fn write_lvt_lint0_register(&self);

    unsafe fn read_lvt_lint1_register(&self) -> u32;

    unsafe fn write_lvt_lint1_register(&self);

    unsafe fn read_initial_count_register(&self) -> u32;

    unsafe fn write_initial_count_register(&self);

    unsafe fn read_current_count_register(&self) -> u32;

    unsafe fn write_current_count_register(&self);

    unsafe fn read_dcr(&self) -> u32;

    unsafe fn write_dcr(&self);
}
