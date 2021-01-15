//**************************************************************************************************
// x2apic.rs                                                                                       *
// Copyright (c) 2020-2021 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::apic::{LocalApicCommon, TaskPriority};
use crate::Msr;
use split::Halves;

pub struct LocalX2apic;

impl LocalX2apic {
    pub fn write_self_ipi(&mut self) {}
}

impl LocalApicCommon for LocalX2apic {
    fn read_id_register(&self) -> u32 {
        unsafe { Msr::new(0x802).read().lower_half() }
    }

    fn read_version_register(&self) -> u32 {
        unsafe { Msr::new(0x803).read().lower_half() }
    }

    fn read_tpr(&self) -> TaskPriority {
        unsafe { Msr::new(0x808).read().lower_half().into() }
    }

    fn write_tpr(&mut self, task_priority: TaskPriority) {
        unsafe {
            let value = u64::from_halves(task_priority.into(), 0);
            Msr::new(0x808).write(value);
        }
    }

    fn read_ppr(&self) -> u32 {
        unsafe { Msr::new(0x80A).read().lower_half() }
    }

    fn write_eoi_register(&mut self) {
        unsafe {
            Msr::new(0x80B).write(0);
        }
    }

    fn read_ldr(&self) -> u32 {
        unimplemented!()
    }

    fn read_svr(&self) -> u32 {
        unimplemented!()
    }

    fn write_svr(&self) -> u32 {
        unimplemented!()
    }

    fn read_isr(&self) -> u32 {
        unimplemented!()
    }

    fn read_tmr(&self) -> u32 {
        unimplemented!()
    }

    fn read_irr(&self) -> u32 {
        unimplemented!()
    }

    fn read_esr(&self) -> u32 {
        unimplemented!()
    }

    fn write_esr(&self) {
        unimplemented!()
    }

    fn read_lvt_cmci_register(&self) -> u32 {
        unimplemented!()
    }

    fn write_lvt_cmci_register(&self) {
        unimplemented!()
    }

    fn read_icr(&self) -> u32 {
        unimplemented!()
    }

    fn write_icr(&self) {
        unimplemented!()
    }

    fn read_lvt_time_register(&self) -> u32 {
        unimplemented!()
    }

    fn write_lvt_time_register(&self) {
        unimplemented!()
    }

    fn read_lvt_thermal_sensor_register(&self) -> u32 {
        unimplemented!()
    }

    fn write_lvt_thermal_sensor_register(&self) {
        unimplemented!()
    }

    fn read_lvt_perf_monitor_register(&self) -> u32 {
        unimplemented!()
    }

    fn write_lvt_perf_monitor_register(&self) {
        unimplemented!()
    }

    fn read_lvt_lint0_register(&self) -> u32 {
        unimplemented!()
    }

    fn write_lvt_lint0_register(&self) {
        unimplemented!()
    }

    fn read_lvt_lint1_register(&self) -> u32 {
        unimplemented!()
    }

    fn write_lvt_lint1_register(&self) {
        unimplemented!()
    }

    fn read_initial_count_register(&self) -> u32 {
        unimplemented!()
    }

    fn write_initial_count_register(&self) {
        unimplemented!()
    }

    fn read_current_count_register(&self) -> u32 {
        unimplemented!()
    }

    fn write_current_count_register(&self) {
        unimplemented!()
    }

    fn read_dcr(&self) -> u32 {
        unimplemented!()
    }

    fn write_dcr(&self) {
        unimplemented!()
    }
}
