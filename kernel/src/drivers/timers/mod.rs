//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use alloc::vec::Vec;
use kernel_interface::init::Args;
use units::{Nanoseconds, Time};

#[derive(Copy, Clone)]
pub struct Device {
    id: &'static str,
    init_ptr: Option<fn(&Args)>,
    start_ptr: Option<fn(Nanoseconds<u64>)>,
    read_count_ptr: fn() -> Nanoseconds<u64>,
    calibration: Option<DeviceCalibration>,
}

impl Device {
    pub fn new(
        id: &'static str,
        init_ptr: Option<fn(&Args)>,
        start_ptr: Option<fn(Nanoseconds<u64>)>,
        read_count_ptr: fn() -> Nanoseconds<u64>,
        calibration: Option<DeviceCalibration>,
    ) -> Self {
        Self {
            id,
            init_ptr,
            start_ptr,
            read_count_ptr,
            calibration,
        }
    }

    pub fn id(&self) -> &'static str {
        self.id
    }

    pub fn init(&self, args: &Args) {
        // Some devices might not need to initialize.
        if let Some(value) = self.init_ptr {
            (value)(args);
        }
    }

    pub fn start<T: Time<u64>>(&self, time: T) {
        // Devices like the TSC in x86 simply increment in the background once the CPU starts
        // and do not need to be started.

        match self.start_ptr {
            None => panic!("Timer device cannot be started."),
            Some(ptr) => (ptr)(time.convert()),
        }
    }

    pub fn read_count(&self) -> Nanoseconds<u64> {
        (self.read_count_ptr)()
    }

    pub fn calibration(&self) -> &DeviceCalibration {
        match &self.calibration {
            None => panic!("Timer device does not require calibration."),
            Some(calibration) => calibration,
        }
    }

    pub fn calibration_required(&self) -> bool {
        self.calibration.is_some()
    }
}

#[derive(Copy, Clone)]
pub struct DeviceCalibration {
    start_ptr: fn(),
    finish_ptr: fn(Nanoseconds<u64>),
}

impl DeviceCalibration {
    pub fn new(start_ptr: fn(), finish_ptr: fn(Nanoseconds<u64>)) -> Self {
        Self {
            start_ptr,
            finish_ptr,
        }
    }

    pub fn start(&self) {
        (self.start_ptr)();
    }

    pub fn finish<T: Time<u64>>(&self, time_passed: T) {
        (self.finish_ptr)(time_passed.convert());
    }
}

pub unsafe fn create_devices(args: &Args, vec: &mut Vec<Device>) {
    // No generic devices yet.
}
