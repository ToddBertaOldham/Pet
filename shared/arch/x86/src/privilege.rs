//**************************************************************************************************
// privilege.rs                                                                                    *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use enums::numeric_enum;

numeric_enum!(
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum ProtectionRing {
        Level0 = 0,
        Level1 = 1,
        Level2 = 2,
        Level3 = 3,
    }

    impl TryFrom<u8>;
);
