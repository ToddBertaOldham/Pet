//**************************************************************************************************
// privilege.rs                                                                                    *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProtectionRing {
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3
}

impl TryFrom<u8> for ProtectionRing {
    type Error = ();

    fn try_from(value : u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ProtectionRing::Level0),
            1 => Ok(ProtectionRing::Level1),
            2 => Ok(ProtectionRing::Level2),
            3 => Ok(ProtectionRing::Level3),
            _ => Err(())
        }
    }
}