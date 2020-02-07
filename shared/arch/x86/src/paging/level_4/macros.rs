//**************************************************************************************************
// macros.rs                                                                                       *
// Copyright (c) 2020 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[macro_export]
macro_rules! level_4_paging_entry {
    (
        $(#[$attribute:meta])*
        $visibility:vis struct $name:ident
    ) => {
        basic_u64_paging_entry!($visibility struct $name);

        impl $name {
            pub fn execute_disabled(self) -> bool {
                self.0.is_bit_set(63).unwrap()
            }

            pub fn set_execute_disabled(&mut self, value: bool) {
                self.0.set_bit(63, value).unwrap();
            }
        }
    };
}