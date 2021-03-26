//**************************************************************************************************
// xapic.rs                                                                                        *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::apic::{LocalApicCommon, TaskPriority};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct LocalXapic {
    address: *mut u8,
}

impl LocalXapic {
    pub unsafe fn new(address: *mut u8) -> Self {
        Self { address }
    }

    unsafe fn get_register(&self, offset: usize) -> *mut u32 {
        self.address.add(offset) as *mut u32
    }

    pub fn write_local_destination(&mut self, value: u32) {
        unimplemented!()
    }
}

impl LocalApicCommon for LocalXapic {
    fn read_id_register(&self) -> u32 {
        unsafe { *self.get_register(0x20) }
    }

    fn read_version_register(&self) -> u32 {
        unimplemented!()
    }

    fn read_tpr(&self) -> TaskPriority {
        unimplemented!()
    }

    fn write_tpr(&mut self, value: TaskPriority) {
        unimplemented!()
    }

    fn read_ppr(&self) -> u32 {
        unimplemented!()
    }

    fn write_eoi_register(&mut self) {
        unimplemented!()
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
