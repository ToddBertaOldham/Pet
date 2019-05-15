// *************************************************************************
// graphics.rs
// Copyright 2018-2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ptr;
use core::option::Option;
use core::result::Result;
use super::ffi::{ PhysicalAddress, Status };
use super::ffi::graphics_output;
use super::error::Error;
use super::protocol::{ ProtocolHandleBuffer, Protocol, ProtocolProvider };

pub struct OutputProvider(ProtocolHandleBuffer);

impl OutputProvider {
    pub fn new() -> Result<Self, Error> {
        let handle_buffer = ProtocolHandleBuffer::new(graphics_output::Protocol::GUID)?;
         Ok(Self(handle_buffer))
    }
}

impl ProtocolProvider<Output> for OutputProvider {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn open(&self, id : usize) -> Result<Output, Error> {
        let protocol = self.0.open(id)?;
        Ok(Output(protocol))       
    }
}

pub struct Output(Protocol);

impl Output {
    pub fn new(protocol : Protocol) -> Result<Self, Error> {
       if protocol.guid() != graphics_output::Protocol::GUID {
           return Err(Error::InvalidArgument("protocol"));
       }
       Ok(Self(protocol))
    }

    pub fn mode(&self) -> u32 {
        unsafe {
            let gop = &*self.0.interface::<graphics_output::Protocol>();
            let gop_mode = &*gop.mode;

            gop_mode.mode
        }
    }

    pub fn set_mode(&self, mode : u32) -> Result<(), Error> {
        unsafe {
            let interface = self.0.interface::<graphics_output::Protocol>();
            let gop = &*interface;
            let gop_mode = &*gop.mode;

            if mode == gop_mode.mode {
                return Ok(());
            }

            let status = (gop.set_mode)(interface, mode);

            match status {
                Status::SUCCESS => Ok(()),
                Status::UNSUPPORTED => Err(Error::InvalidArgument("mode")),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                _ => Err(Error::UnexpectedStatus(status))
            }
        }
    }

    pub fn mode_count(&self) -> u32 {
        unsafe {            
            let gop = &*self.0.interface::<graphics_output::Protocol>();
            let gop_mode = &*gop.mode;
            
            gop_mode.max_mode
        }
    }

    pub fn query_mode(&self, mode : u32) -> Result<ModeInfo, Error> {
        unsafe {
            let interface = self.0.interface::<graphics_output::Protocol>();
            let gop = &*interface;

            let mut info_size = 0;
            let mut info_ptr = ptr::null_mut(); 

            let status = (gop.query_mode)(interface, mode, &mut info_size, &mut info_ptr);

            match status {
                Status::SUCCESS => {
                    let info = &*info_ptr;
                    Ok(ModeInfo::new(info.horizontal_resolution, info.vertical_resolution, info.pixel_format != graphics_output::PixelFormat::BltOnly))
                },
                Status::INVALID_PARAMETER => Err(Error::InvalidArgument("mode")),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                _ => Err(Error::UnexpectedStatus(status))
            }
        }
    }

    pub fn set_closest_mode_from_resolution(&self, width : u32, height : u32, require_framebuffer_address : bool) -> Result<(), Error> {
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
            Err(Error::NotSupported)
        }    
    }

    pub fn set_mode_from_resolution(&self, width : u32, height : u32, require_framebuffer_address : bool) -> Result<(), Error> {
        for mode in 0..self.mode_count() {
            let info = self.query_mode(mode)?;

            if require_framebuffer_address && !info.supports_framebuffer_address() {
                continue;
            }

            if info.width() == width && info.height() == height {
                return self.set_mode(mode);
            }
        }
        
        Err(Error::NotSupported)  
    }

    pub fn maximize(&self, require_framebuffer_address : bool) -> Result<(), Error> {
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
            Err(Error::NotSupported)
        }
    }

    pub fn width(&self) -> u32 {
        unsafe {
            let gop = &*self.0.interface::<graphics_output::Protocol>();
            let gop_mode = &*gop.mode;
            let gop_mode_info = &*gop_mode.info;

            gop_mode_info.horizontal_resolution
        }
    }

    pub fn height(&self) -> u32 {
        unsafe {
            let gop = &*self.0.interface::<graphics_output::Protocol>();
            let gop_mode = &*gop.mode;
            let gop_mode_info = &*gop_mode.info;

            gop_mode_info.vertical_resolution
        }
    }

    pub fn framebuffer_address(&self) -> Option<PhysicalAddress> {
        unsafe {
            let gop = &*self.0.interface::<graphics_output::Protocol>();
            let gop_mode = &*gop.mode;
            let gop_mode_info = &*gop_mode.info;

            if gop_mode_info.pixel_format == graphics_output::PixelFormat::BltOnly {
                return None;
            }

            Some(gop_mode.frame_buffer_base)
        }
    }
}

pub struct ModeInfo {
    width : u32,
    height : u32,
    //TODO Change this later to pixel format but keep method for it.
    supports_framebuffer_address : bool
}

impl ModeInfo {
    pub fn new(width : u32, height : u32, supports_framebuffer_address : bool) -> Self {
        ModeInfo { width, height, supports_framebuffer_address }
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