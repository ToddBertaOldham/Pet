// *************************************************************************
// display.rs
// Copyright 2018-2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ptr::null_mut;
use core::option::Option;
use core::result::Result;
use super::ffi::{BltOperation, BltPixel, PhysicalAddress, PixelFormat, Status, GOP, GOP_GUID };
use super::error::UefiError;
use super::protocol::{ ProtocolHandleBuffer, Protocol, ProtocolProvider };

pub struct GraphicsOutputProvider {
    handle_buffer : ProtocolHandleBuffer
}

impl GraphicsOutputProvider {
    pub fn new() -> Result<Self, UefiError> {
        let handle_buffer = ProtocolHandleBuffer::new(GOP_GUID)?;
         Ok(GraphicsOutputProvider { handle_buffer })
    }
}

impl ProtocolProvider<GraphicsOutput> for GraphicsOutputProvider {
    fn len(&self) -> usize {
        self.handle_buffer.len()
    }

    fn open(&self, id : usize) -> Result<GraphicsOutput, UefiError> {
        unsafe {
            let protocol = self.handle_buffer.open(id)?;
            Ok(GraphicsOutput::new_unchecked(protocol))
        }
    }
}

pub struct GraphicsOutput {
    protocol : Protocol
}

impl GraphicsOutput {
    pub fn new(protocol : Protocol) -> Result<Self, UefiError> {
       if protocol.guid() != GOP_GUID {
           return Err(UefiError::InvalidArgument("protocol"));
       }
       Ok(GraphicsOutput { protocol })
    }

    pub unsafe fn new_unchecked(protocol : Protocol) -> Self {
        GraphicsOutput { protocol }
    }

    pub fn mode(&self) -> u32 {
        unsafe {
            let gop = &*self.protocol.interface::<GOP>();
            let gop_mode = &*gop.mode;

            gop_mode.mode
        }
    }

    pub fn set_mode(&self, mode : u32) -> Result<(), UefiError> {
        unsafe {
            let interface = self.protocol.interface::<GOP>();
            let gop = &*interface;
            let gop_mode = &*gop.mode;

            if mode == gop_mode.mode {
                return Ok(());
            }

            let status = (gop.set_mode)(interface, mode);

            match status {
                Status::SUCCESS => Ok(()),
                Status::UNSUPPORTED => Err(UefiError::InvalidArgument("mode")),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }

    pub fn mode_count(&self) -> u32 {
        unsafe {            
            let gop = &*self.protocol.interface::<GOP>();
            let gop_mode = &*gop.mode;
            
            gop_mode.max_mode
        }
    }

    pub fn query_mode(&self, mode : u32) -> Result<GraphicsOutModeInfo, UefiError> {
        unsafe {
            let interface = self.protocol.interface::<GOP>();
            let gop = &*interface;

            let mut info_size = 0;
            let mut info_ptr = null_mut(); 

            let status = (gop.query_mode)(interface, mode, &mut info_size, &mut info_ptr);

            match status {
                Status::SUCCESS => {
                    let info = &*info_ptr;
                    Ok(GraphicsOutModeInfo::new(info.horizontal_resolution, info.vertical_resolution, info.pixel_format != PixelFormat::BltOnly))
                },
                Status::INVALID_PARAMETER => Err(UefiError::InvalidArgument("mode")),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }

    pub fn set_closest_mode_from_resolution(&self, width : u32, height : u32, require_framebuffer_address : bool) -> Result<(), UefiError> {
        let mut closest_mode = None;
        let mut closest_score = 0;

        for mode in 0..self.mode_count() {
            let info = self.query_mode(mode)?;

            if require_framebuffer_address && !info.supports_framebuffer_address() {
                continue;
            }
            
            let mode_width = info.width();
            let mode_height = info.height();

            if mode_width == width && mode_height == height {
                return self.set_mode(mode);
            }

            // How many pixels are off from the desired resolution.           
            let score = (width as i64 - mode_width as i64).abs() + (height as i64 - mode_height as i64).abs();

            if closest_mode.is_none() || score < closest_score {
                closest_mode = Some(mode);
                closest_score = score;
            }
        }

        if let Some(mode) = closest_mode {
            self.set_mode(mode)
        }
        else {
            Err(UefiError::NotSupported)
        }    
    }

    pub fn set_mode_from_resolution(&self, width : u32, height : u32, require_framebuffer_address : bool) -> Result<(), UefiError> {
        for mode in 0..self.mode_count() {
            let info = self.query_mode(mode)?;

            if require_framebuffer_address && !info.supports_framebuffer_address() {
                continue;
            }

            if info.width() == width && info.height() == height {
                return self.set_mode(mode);
            }
        }
        
        Err(UefiError::NotSupported)  
    }

    pub fn maximize(&self, require_framebuffer_address : bool) -> Result<(), UefiError> {
        let mut best_mode = None;
        let mut largest_width = 0;
        let mut largest_height = 0;

        for mode in 0..self.mode_count() {
            let info = self.query_mode(mode)?;

            if require_framebuffer_address && !info.supports_framebuffer_address() {
                continue;
            }

            let mode_width = info.width();
            let mode_height = info.height();

            if  mode_width * mode_height > largest_width * largest_height {
                best_mode = Some(mode);
                largest_width = mode_width;
                largest_height = mode_height;
            }
        }

        if let Some(mode) = best_mode {
            self.set_mode(mode)
        }
        else {
            Err(UefiError::NotSupported)
        }
    }

    pub fn width(&self) -> u32 {
        unsafe {
            let gop = &*self.protocol.interface::<GOP>();
            let gop_mode = &*gop.mode;
            let gop_mode_info = &*gop_mode.info;

            gop_mode_info.horizontal_resolution
        }
    }

    pub fn height(&self) -> u32 {
        unsafe {
            let gop = &*self.protocol.interface::<GOP>();
            let gop_mode = &*gop.mode;
            let gop_mode_info = &*gop_mode.info;

            gop_mode_info.vertical_resolution
        }
    }

    pub fn framebuffer_address(&self) -> Option<PhysicalAddress> {
        unsafe {
            let gop = &*self.protocol.interface::<GOP>();
            let gop_mode = &*gop.mode;
            let gop_mode_info = &*gop_mode.info;

            if gop_mode_info.pixel_format == PixelFormat::BltOnly {
                return None;
            }

            Some(gop_mode.frame_buffer_base)
        }
    }
}

pub struct GraphicsOutModeInfo {
    width : u32,
    height : u32,
    //TODO Change this later to pixel format but keep method for it.
    supports_framebuffer_address : bool
}

impl GraphicsOutModeInfo {
    pub fn new(width : u32, height : u32, supports_framebuffer_address : bool) -> Self {
        GraphicsOutModeInfo { width, height, supports_framebuffer_address }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, value : u32) {
        self.width = value;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, value : u32) {
        self.height = value;
    }

    pub fn supports_framebuffer_address(&self) -> bool {
        self.supports_framebuffer_address
    }
}