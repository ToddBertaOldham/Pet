//**************************************************************************************************
// acpi_interface.rs                                                                               *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch::vmm;
use acpi;
use core::convert::TryInto;
use core::fmt::Debug;

//TODO Is there a better place for this?

pub struct AcpiInterface;

impl acpi::Interface for AcpiInterface {
    unsafe fn convert_to_virtual_ptr<TPtr, TAddress: TryInto<*mut TPtr, Error: Debug>>(
        &self,
        address: TAddress,
    ) -> *mut TPtr {
        let ptr = address
            .try_into()
            .expect("Address cannot be converted to pointer.");

        vmm::convert_physical_ptr_mut(ptr)
    }
}
