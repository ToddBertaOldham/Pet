//**************************************************************************************************
// arch.rs                                                                                         *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub trait LocalApicCommon {
    fn read_id_register(&self) -> u32;

    fn read_version_register(&self) -> u32;

    fn read_tpr(&self) -> TaskPriority;
    fn write_tpr(&mut self, value: TaskPriority);

    fn read_ppr(&self) -> u32;

    fn write_eoi_register(&mut self);

    fn read_ldr(&self) -> u32;

    fn read_svr(&self) -> u32;
    fn write_svr(&self) -> u32;

    fn read_isr(&self) -> u32;

    fn read_tmr(&self) -> u32;

    fn read_irr(&self) -> u32;

    fn read_esr(&self) -> u32;
    fn write_esr(&self);

    fn read_lvt_cmci_register(&self) -> u32;
    fn write_lvt_cmci_register(&self);

    fn read_icr(&self) -> u32;
    fn write_icr(&self);

    fn read_lvt_time_register(&self) -> u32;
    fn write_lvt_time_register(&self);

    fn read_lvt_thermal_sensor_register(&self) -> u32;
    fn write_lvt_thermal_sensor_register(&self);

    fn read_lvt_perf_monitor_register(&self) -> u32;
    fn write_lvt_perf_monitor_register(&self);

    fn read_lvt_lint0_register(&self) -> u32;
    fn write_lvt_lint0_register(&self);

    fn read_lvt_lint1_register(&self) -> u32;
    fn write_lvt_lint1_register(&self);

    fn read_initial_count_register(&self) -> u32;
    fn write_initial_count_register(&self);

    fn read_current_count_register(&self) -> u32;
    fn write_current_count_register(&self);

    fn read_dcr(&self) -> u32;
    fn write_dcr(&self);
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct TaskPriority(u32);

impl From<u32> for TaskPriority {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<TaskPriority> for u32 {
    fn from(value: TaskPriority) -> Self {
        value.0
    }
}
