//**************************************************************************************************
// cursor.rs                                                                                       *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::{Read, Write};
#[cfg(feature = "alloc-impl")]
use alloc::vec::Vec;
use core::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Error(usize);

impl Error {
    pub fn bytes_over(self) -> usize {
        self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Read {} bytes over.", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Cursor<T> {
    source: T,
    position: usize,
}

impl<T> Cursor<T> {
    pub fn new(source: T) -> Self {
        Cursor {
            source,
            position: 0,
        }
    }

    pub fn with_position(source: T, position: usize) -> Self {
        Cursor { source, position }
    }

    pub fn source(&self) -> &T {
        &self.source
    }

    pub fn source_mut(&mut self) -> &mut T {
        &mut self.source
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position
    }
}

impl<T: AsRef<[u8]>> Read for Cursor<T> {
    type Error = Error;

    default fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let source = self.source.as_ref();

        let end = self.position + buffer.len();
        if end > source.len() {
            return Err(Error(end - source.len()));
        }

        let slice = &source[self.position..end];
        buffer.copy_from_slice(slice);
        self.position = end;

        Ok(())
    }
}

impl<T: AsMut<[u8]>> Write for Cursor<T> {
    type Error = Error;

    default fn write(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let source = self.source.as_mut();

        let end = self.position + buffer.len();
        if end > source.len() {
            return Err(Error(end - source.len()));
        }

        let slice = &mut source[self.position..end];
        slice.copy_from_slice(buffer);
        self.position = end;

        Ok(())
    }
}

#[cfg(feature = "alloc-impl")]
impl Write for Cursor<Vec<u8>> {
    fn write(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let end = self.position + buffer.len();
        if end > self.source.len() {
            self.source.resize(end, 0);
        }

        let slice = &mut self.source[self.position..end];
        slice.copy_from_slice(buffer);
        self.position = end;

        Ok(())
    }
}
