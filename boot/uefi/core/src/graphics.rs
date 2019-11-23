//**************************************************************************************************
// graphics.rs                                                                                     *
// Copyright (c) 2018-2019 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::error::Error;
use super::ffi::graphics_output;
use super::ffi::{PhysicalAddress, Status};
use super::protocol;
use core::iter::FusedIterator;
use core::ptr;

#[derive(Debug)]
pub struct OutputBuffer(protocol::HandleBuffer);

impl OutputBuffer {
    pub fn locate() -> Result<Self, Error> {
        let handle_buffer = protocol::HandleBuffer::locate(graphics_output::Protocol::GUID)?;
        Ok(Self(handle_buffer))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn open(&self, index: usize) -> Result<Output, Error> {
        let protocol = self.0.open(index)?;
        Ok(Output(protocol))
    }

    pub fn iter(&self) -> OutputIterator {
        OutputIterator(self.0.iter())
    }
}

impl<'a> IntoIterator for &'a OutputBuffer {
    type Item = Result<Output, Error>;
    type IntoIter = OutputIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct OutputIterator<'a>(protocol::InterfaceIterator<'a>);

impl<'a> Iterator for OutputIterator<'a> {
    type Item = Result<Output, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|result| result.map(|interface| Output(interface)))
    }
}

impl<'a> FusedIterator for OutputIterator<'a> {}

#[derive(Debug)]
pub struct Output(protocol::Interface);

impl Output {
    pub fn new(interface: protocol::Interface) -> Result<Self, Error> {
        if interface.protocol_guid() != graphics_output::Protocol::GUID {
            return Err(Error::InvalidArgument("interface"));
        }
        Ok(Self(interface))
    }

    pub unsafe fn new_unchecked(protocol: protocol::Interface) -> Self {
        Output(protocol)
    }

    pub fn mode(&self) -> u32 {
        unsafe {
            let gop = &*self.0.get::<graphics_output::Protocol>();
            let gop_mode = &*gop.mode;

            gop_mode.mode
        }
    }

    pub fn set_mode(&mut self, mode: Mode) -> Result<(), Error> {
        self.set_mode_with_index(mode.index())
    }

    pub fn set_mode_with_index(&mut self, index: u32) -> Result<(), Error> {
        unsafe {
            let interface = self.0.get::<graphics_output::Protocol>();
            let gop = &*interface;
            let gop_mode = &*gop.mode;

            if index == gop_mode.mode {
                return Ok(());
            }

            let status = (gop.set_mode)(interface, index);

            match status {
                Status::SUCCESS => Ok(()),
                Status::UNSUPPORTED => Err(Error::InvalidArgument("index")),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                _ => Err(Error::UnexpectedStatus(status)),
            }
        }
    }

    pub fn mode_len(&self) -> u32 {
        unsafe {
            let gop = &*self.0.get::<graphics_output::Protocol>();
            let gop_mode = &*gop.mode;

            gop_mode.max_mode
        }
    }

    pub fn query_mode(&self, mode: u32) -> Result<ModeInfo, Error> {
        unsafe {
            let interface = self.0.get::<graphics_output::Protocol>();
            let gop = &*interface;

            let mut info_size = 0;
            let mut info_ptr = ptr::null_mut();

            let status = (gop.query_mode)(interface, mode, &mut info_size, &mut info_ptr);

            match status {
                Status::SUCCESS => {
                    let info = &*info_ptr;
                    Ok(ModeInfo {
                        width: info.horizontal_resolution,
                        height: info.vertical_resolution,
                        supports_framebuffer: info.pixel_format
                            != graphics_output::PixelFormat::BltOnly,
                    })
                }
                Status::INVALID_PARAMETER => Err(Error::InvalidArgument("mode")),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                _ => Err(Error::UnexpectedStatus(status)),
            }
        }
    }

    pub fn iter_modes(&self) -> ModeIterator {
        ModeIterator {
            output: self,
            index: 0,
        }
    }

    pub fn set_closest_mode_from_resolution(
        &mut self,
        width: u32,
        height: u32,
        require_framebuffer: bool,
    ) -> Result<(), Error> {
        let mut closest_mode_index = None;
        let mut closest_score = 0;

        for mode_result in self.iter_modes() {
            let mode = mode_result?;

            if require_framebuffer && !mode.info().supports_framebuffer() {
                continue;
            }

            let mode_width = mode.info().width();
            let mode_height = mode.info().height();

            if mode_width == width && mode_height == height {
                return self.set_mode(mode);
            }

            // How many pixels are off from the desired resolution.
            let score = (width as i64 - mode_width as i64).abs()
                + (height as i64 - mode_height as i64).abs();

            if closest_mode_index.is_none() || score < closest_score {
                closest_mode_index = Some(mode.index());
                closest_score = score;
            }
        }

        if let Some(mode) = closest_mode_index {
            self.set_mode_with_index(mode)
        } else {
            Err(Error::NotSupported)
        }
    }

    pub fn set_mode_from_resolution(
        &mut self,
        width: u32,
        height: u32,
        require_framebuffer: bool,
    ) -> Result<(), Error> {
        for mode_result in self.iter_modes() {
            let mode = mode_result?;

            if require_framebuffer && !mode.info().supports_framebuffer() {
                continue;
            }

            if mode.info().width() == width && mode.info().height() == height {
                return self.set_mode(mode);
            }
        }

        Err(Error::NotSupported)
    }

    pub fn maximize(&mut self, require_framebuffer: bool) -> Result<(), Error> {
        let mut best_mode_index = None;
        let mut largest_width = 0;
        let mut largest_height = 0;

        for mode_result in self.iter_modes() {
            let mode = mode_result?;

            if require_framebuffer && !mode.info().supports_framebuffer() {
                continue;
            }

            let mode_width = mode.info().width();
            let mode_height = mode.info().height();

            if mode_width * mode_height > largest_width * largest_height {
                best_mode_index = Some(mode.index());
                largest_width = mode_width;
                largest_height = mode_height;
            }
        }

        if let Some(mode) = best_mode_index {
            self.set_mode_with_index(mode)
        } else {
            Err(Error::NotSupported)
        }
    }

    pub fn width(&self) -> u32 {
        unsafe {
            let gop = &*self.0.get::<graphics_output::Protocol>();
            let gop_mode = &*gop.mode;
            let gop_mode_info = &*gop_mode.info;

            gop_mode_info.horizontal_resolution
        }
    }

    pub fn height(&self) -> u32 {
        unsafe {
            let gop = &*self.0.get::<graphics_output::Protocol>();
            let gop_mode = &*gop.mode;
            let gop_mode_info = &*gop_mode.info;

            gop_mode_info.vertical_resolution
        }
    }

    pub fn framebuffer_address(&self) -> Option<PhysicalAddress> {
        unsafe {
            let gop = &*self.0.get::<graphics_output::Protocol>();
            let gop_mode = &*gop.mode;
            let gop_mode_info = &*gop_mode.info;

            if gop_mode_info.pixel_format == graphics_output::PixelFormat::BltOnly {
                return None;
            }

            Some(gop_mode.frame_buffer_base)
        }
    }
}

impl<'a> IntoIterator for &'a Output {
    type Item = Result<Mode, Error>;
    type IntoIter = ModeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_modes()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Mode {
    index: u32,
    info: ModeInfo,
}

impl Mode {
    pub fn index(self) -> u32 {
        self.index
    }

    pub fn info(self) -> ModeInfo {
        self.info
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ModeInfo {
    width: u32,
    height: u32,
    supports_framebuffer: bool,
}

impl ModeInfo {
    pub fn width(self) -> u32 {
        self.width
    }

    pub fn height(self) -> u32 {
        self.height
    }

    //TODO Change this later to pixel format but keep method for it.
    pub fn supports_framebuffer(self) -> bool {
        self.supports_framebuffer
    }
}

#[derive(Debug)]
pub struct ModeIterator<'a> {
    output: &'a Output,
    index: u32,
}

impl<'a> Iterator for ModeIterator<'a> {
    type Item = Result<Mode, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.output.mode_len() {
            None
        } else {
            let index = self.index;
            let info = self.output.query_mode(index).ok()?;
            self.index += 1;
            Some(Ok(Mode { index, info }))
        }
    }
}

impl<'a> FusedIterator for ModeIterator<'a> {}
