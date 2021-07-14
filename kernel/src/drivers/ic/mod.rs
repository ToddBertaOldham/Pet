//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use kernel_interface::init::Args;

#[derive(Copy, Clone)]
pub struct Device {
    id: &'static str,
    init_ptr: Option<fn(&Args)>,
}

impl Device {
    pub fn new(id: &'static str, init_ptr: Option<fn(&Args)>) -> Self {
        Self { id, init_ptr }
    }

    pub fn id(&self) -> &'static str {
        self.id
    }

    pub fn init(&self, args: &Args) {
        // Some devices might not need to initialize.
        if let Some(value) = self.init_ptr {
            (value)(args);
        }
    }
}
