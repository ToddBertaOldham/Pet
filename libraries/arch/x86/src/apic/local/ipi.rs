//**************************************************************************************************
// ipi.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use enums::c_enum;
use memory::split::Halves;
use memory::{GetBit, SetBitAssign};

macro_rules! create_ipi_base {
    ($name:ident) => {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        #[repr(transparent)]
        pub struct $name(u64);

        impl $name {
            pub fn new() -> Self {
                Self(0)
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> $name {
                Self(value)
            }
        }

        impl From<$name> for u64 {
            fn from(value: $name) -> u64 {
                value.0
            }
        }
    };
}

c_enum!(
    pub enum DestinationShorthand : u8 {
        NO_SHORTHAND = 0b00,
        SELF = 0b01,
        ALL_INCLUDING_SELF = 0b10,
        ALL_EXCLUDING_SELF = 0b11,
    }
);

create_ipi_base!(Ipi);

impl Ipi {
    pub fn send_pending(self) -> bool {
        self.0.get_bit(12)
    }

    pub fn set_destination_id(&mut self, id: u8) {
        self.0.set_bits_assign(id as u64, 56, 0, 0);
    }

    pub fn destination_id(self) -> u8 {
        self.0.get_bits(56, 0, 8) as u8
    }
}

create_ipi_base!(X2Ipi);

impl X2Ipi {
    pub fn set_destination_id(&mut self, id: u32) {
        self.0.set_bits_assign(id as u64, 32, 0, 32);
    }

    pub fn destination_id(self) -> u32 {
        self.0.lower_half()
    }
}
