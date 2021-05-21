//**************************************************************************************************
// mps.rs                                                                                          *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use memory::flags;

flags!(
    pub struct MpsInti : u16 {
        ACTIVE_LOW_POLARITY = 0b01;
        ACTIVE_HIGH_POLARITY = 0b11;
        EDGE_TRIGGER_MODE = 0b0100;
        LEVEL_TRIGGER_MODE = 0b1100;
    }
);
