// *************************************************************************
// macros.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#[macro_use]
macro_rules! read_constructor {
    () => {
        pub fn read<'a>(data : &'a[u8]) -> Result<&'a Self, ElfError> {
            unsafe {
                if data.len() < mem::size_of::<Self>() {
                    return Err(ElfError::SourceTooSmall);
                }

                Ok(&*(data.as_ptr() as *const Self))
            }
        }
    };
}