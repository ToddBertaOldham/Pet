//**************************************************************************************************
// registers.rs                                                                                    *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use memory::{GetBit, SetBit};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Registers {
    address: *mut u8,
}

impl Registers {
    pub const fn new(address: *mut u8) -> Self {
        Self { address }
    }

    unsafe fn get_register(&self, offset: usize) -> *mut u64 {
        self.address.add(offset) as *mut u64
    }

    pub unsafe fn read_gci_register(&self) -> CapabilitiesAndId {
        (*self.get_register(0x0)).into()
    }

    pub unsafe fn read_gc_register(&self) -> GeneralConfig {
        (*self.get_register(0x010)).into()
    }

    pub unsafe fn read_gis_register(&self) -> GeneralInterruptStatus {
        (*self.get_register(0x020)).into()
    }

    pub unsafe fn read_mcv_register(&self) -> u64 {
        (*self.get_register(0x0F0))
    }

    pub unsafe fn read_tcc_register(&self, timer: usize) -> u64 {
        assert!(timer < 32);
        (*self.get_register(0x100 + (timer * 0x20)))
    }

    pub unsafe fn read_tcv_register(&self, timer: usize) -> u64 {
        assert!(timer < 32);
        (*self.get_register(0x108 + (timer * 0x20)))
    }

    pub unsafe fn read_fsb_ir_register(&self, timer: usize) -> u64 {
        assert!(timer < 32);
        (*self.get_register(0x110 + (timer * 0x20)))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CapabilitiesAndId(u64);

impl CapabilitiesAndId {
    pub fn revision_id(self) -> u8 {
        self.0 as u8
    }
}

impl From<u64> for CapabilitiesAndId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<CapabilitiesAndId> for u64 {
    fn from(value: CapabilitiesAndId) -> Self {
        value.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GeneralConfig(u64);

impl GeneralConfig {
    pub fn cnf_enabled(self) -> bool {
        self.0.get_bit(0)
    }

    pub fn set_cnf_enabled(&mut self, value: bool) {
        self.0.set_bit(0, value);
    }

    pub fn leg_rt_cnf(self) -> bool {
        self.0.get_bit(1)
    }

    pub fn set_leg_rt_cnf(&mut self, value: bool) {
        self.0.set_bit(1, value);
    }
}

impl From<u64> for GeneralConfig {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<GeneralConfig> for u64 {
    fn from(value: GeneralConfig) -> Self {
        value.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GeneralInterruptStatus(u64);

impl GeneralInterruptStatus {
    pub fn interrupt_active(self, timer: usize) -> bool {
        assert!(timer < 32);
        self.0.get_bit(timer as u32)
    }

    pub fn set_interrupt_active(&mut self, timer: usize, value: bool) {
        assert!(timer < 32);
        self.0.set_bit(timer as u32, value);
    }
}

impl From<u64> for GeneralInterruptStatus {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<GeneralInterruptStatus> for u64 {
    fn from(value: GeneralInterruptStatus) -> Self {
        value.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerConfigAndCapabilities(u64);

impl TimerConfigAndCapabilities {
    pub fn cnf_enabled(self) -> bool {
        self.0.get_bit(0)
    }

    pub fn set_cnf_enabled(&mut self, value: bool) {
        self.0.set_bit(0, value);
    }

    pub fn leg_rt_cnf(self) -> bool {
        self.0.get_bit(1)
    }

    pub fn set_leg_rt_cnf(&mut self, value: bool) {
        self.0.set_bit(1, value);
    }
}

impl From<u64> for TimerConfigAndCapabilities {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<TimerConfigAndCapabilities> for u64 {
    fn from(value: TimerConfigAndCapabilities) -> Self {
        value.0
    }
}
