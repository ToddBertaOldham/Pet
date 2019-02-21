// *************************************************************************
// text_io.rs
// Copayright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::ffi::{ SimpleTextOutputProtocol };
use core::fmt::{ Write, Error };
use core::alloc::Layout;
use alloc::alloc::{ alloc, dealloc };

pub struct TextOuputWriter(*mut SimpleTextOutputProtocol);

impl TextOuputWriter {
    pub unsafe fn new(protocol : *mut SimpleTextOutputProtocol) -> Self {
        TextOuputWriter(protocol)
    }
}

impl Write for TextOuputWriter {
    fn write_str(&mut self, s : &str) -> Result<(), Error> {
        let length = s.encode_utf16().count();

        if length == 0 {
            return Ok(());
        }

        unsafe {
            let protocol = &*self.0;

            let layout = Layout::array::<u16>(length + 1).expect("Invalid parameters for layout.");
            let buffer = alloc(layout);
            let characters = buffer as *mut u16;

            let mut next_character = characters;
            for char16 in s.encode_utf16() {
                (*next_character) = char16;
                next_character = next_character.offset(1);
            }

            (*next_character) = 0;

            (protocol.output_string)(self.0, characters);

            dealloc(buffer, layout); 

            Ok(())
        }
    }
}