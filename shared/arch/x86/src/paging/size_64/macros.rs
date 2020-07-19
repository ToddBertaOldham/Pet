//**************************************************************************************************
// macros.rs                                                                                       *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

macro_rules! level_4_paging_entry {
    (
        $(#[$attribute:meta])*
        $visibility:vis struct $name:ident
    ) => {
        basic_u64_paging_entry!($visibility struct $name);

        impl $name {
            pub fn execute_disabled(self) -> bool {
                self.0.read_bit(63).unwrap()
            }

            pub fn set_execute_disabled(&mut self, value: bool) -> &mut Self {
                self.0.write_bit_assign(63, value).unwrap();
                self
            }
        }
    };
}