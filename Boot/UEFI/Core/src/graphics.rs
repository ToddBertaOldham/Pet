// *************************************************************************
// display.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ffi::c_void;
use core::ptr::null_mut;
use core::option::Option;
use core::result::Result;
use alloc::vec::Vec;
use super::drawing::{Color, Rectangle};
use super::ffi::{BltOperation, BltPixel, Handle, PhysicalAddress, PixelFormat, Status, GOP, GOP_GUID, OPEN_PROTOCOL_BY_HANDLE_PROTOCOL};
use super::error::UEFIError;
use super::system as uefi_system;
use super::protocol::{ ProtocolHandleBuffer, Protocol };

pub struct GraphicsOutputProvider {
    handle_buffer : ProtocolHandleBuffer
}

impl GraphicsOutputProvider {
    pub fn new() -> Result<Self, UEFIError> {
        let handle_buffer = ProtocolHandleBuffer::new(GOP_GUID)?;
         Ok(GraphicsOutputProvider { handle_buffer })
    }

    pub fn len(&self) -> usize {
        self.handle_buffer.len()
    }

    pub fn get(&self, id : usize) -> Result<GraphicsOutput, UEFIError> {
        unsafe {
            let protocol = self.handle_buffer.get(id)?;
            Ok(GraphicsOutput::new_unchecked(protocol))
        }
    }

    pub fn collect(&self) -> Result<Vec<GraphicsOutput>, UEFIError> {
        let mut vec = Vec::with_capacity(self.handle_buffer.len());
        
        for id in 0..vec.capacity() {
            let output = self.get(id)?;
            vec.push(output);
        }

        Ok(vec)
    }
}

pub struct GraphicsOutput {
    protocol : Protocol
}

impl GraphicsOutput {
    pub fn new(protocol : Protocol) -> Result<Self, UEFIError> {
       if protocol.guid() != GOP_GUID {
           return Err(UEFIError::InvalidArgument("protocol"));
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

    pub fn set_mode(&self, mode : u32) -> Result<(), UEFIError> {
        unsafe {
            let gop = &*self.protocol.interface::<GOP>();
            let gop_mode = &*gop.mode;

            if mode == gop_mode.mode {
                return Ok(());
            }

            let status = (gop.set_mode)(self.protocol.interface::<GOP>(), mode);

            match status {
                Status::Success => Ok(()),
                Status::Unsupported => Err(UEFIError::InvalidArgument("mode")),
                Status::DeviceError => Err(UEFIError::HardwareFailure),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
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

    pub fn query_mode(&self, mode : u32) -> Result<GraphicsOutModeInfo, UEFIError> {
        unsafe {
            let gop = &*self.protocol.interface::<GOP>();

            let mut info_size = 0;
            let mut info_ptr = null_mut(); 

            let status = (gop.query_mode)(self.protocol.interface::<GOP>(), mode, &mut info_size, &mut info_ptr);

            match status {
                Status::Success => {
                    let info = &*info_ptr;
                    Ok(GraphicsOutModeInfo::new(info.horizontal_resolution, info.vertical_resolution, info.pixel_format != PixelFormat::BltOnly))
                },
                Status::InvalidParameter => Err(UEFIError::InvalidArgument("mode")),
                Status::DeviceError => Err(UEFIError::HardwareFailure),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    pub fn maximize(&self, require_framebuffer_address : bool)-> Result<(), UEFIError> {
        let mut best_mode = self.mode();
        let mut largest_width = self.width();
        let mut largest_height = self.height();

        for mode in 0..self.mode_count() {
            let info = self.query_mode(mode)?;

            if require_framebuffer_address && !info.supports_framebuffer_address() {
                continue;
            }

            if info.width() > largest_width || info.height() > largest_height {
                best_mode = mode;
                largest_width = info.width();
                largest_height = info.height();
            }
        }

        self.set_mode(best_mode)
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