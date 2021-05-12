//**************************************************************************************************
// memory_map.rs                                                                                   *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use alloc::vec::Vec;
use bits::GetBit;
use core::mem;
use core::ptr;
use core::slice;
use enums::c_enum;
use uefi::memory::MemoryType as UefiMemoryType;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MemoryMap {
    pub ptr: *mut MemorySection,
    pub len: usize,
    pub capacity: usize,
}

impl MemoryMap {
    pub const fn new() -> Self {
        MemoryMap {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn from_vec(mut vector: Vec<MemorySection>) -> Self {
        let memory_map = Self {
            ptr: vector.as_mut_ptr(),
            len: vector.len(),
            capacity: vector.capacity(),
        };
        mem::forget(vector);
        memory_map
    }

    pub unsafe fn into_vec(self) -> Vec<MemorySection> {
        Vec::from_raw_parts(self.ptr, self.len, self.capacity)
    }

    //TODO This is no longer used in the UEFI boot loader, but it will likely be useful both
    // in the kernel and other boot loaders in the future, so it's worth keeping this and finishing
    // it and cleaning it up.

    pub unsafe fn declare_section(&mut self, new_section: MemorySection) -> bool {
        let mut vector = self.into_vec();
        let original_capacity = vector.capacity();

        let mut index = 0;
        let new_segment = new_section.as_segment();
        let mut found_area = false;

        loop {
            let current_index = index;

            if current_index < vector.len() {
                let existing_section = vector[current_index];
                let existing_segment = existing_section.as_segment();

                if found_area {
                    // Once the existing section has been found then clean up sections that are
                    // also included in the area of the new section.

                    if new_segment.contains(existing_segment) {
                        vector.remove(index);
                        continue;
                    } else if existing_segment.start() > new_segment.start()
                        && existing_segment.start() < new_segment.end()
                    {
                        vector[current_index].start = new_segment.end();
                    }

                    break;
                } else if existing_segment.start() <= new_segment.start()
                    && existing_segment.end() > new_segment.start()
                {
                    // Begin by finding the existing section where the new section is located.

                    vector[current_index].len = new_section.start - existing_segment.start();
                    let remaining_length = existing_segment.end().saturating_sub(new_segment.end());

                    index += 1;
                    vector.insert(index, new_section);

                    if remaining_length > 0 {
                        index += 1;
                        vector.insert(
                            index,
                            MemorySection {
                                start: new_segment.end(),
                                len: remaining_length,
                                memory_type: existing_section.memory_type,
                            },
                        );
                        break;
                    }

                    found_area = true;
                }

                index += 1;
            } else {
                // If an area could not be found then add the section to the memory map.

                vector.push(new_section);
                break;
            }
        }

        *self = Self::from_vec(vector);

        original_capacity != self.capacity
    }

    pub unsafe fn as_slice<'a>(self) -> &'a [MemorySection] {
        slice::from_raw_parts(self.ptr, self.len)
    }
}

unsafe impl Send for MemoryMap {}

impl Default for MemoryMap {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MemorySection {
    pub start: usize,
    pub len: usize,
    pub memory_type: MemoryType,
}

impl MemorySection {
    pub fn as_segment(self) -> memory::Segment {
        memory::Segment::with_len(self.start, self.len)
    }
}

const MEMORY_UNUSABLE_BIT: u32 = 0x80000000;

//TODO Eventually the init portion of this lib will require a restructure for different platforms
// and archs.

pub const KERNEL_UEFI_MEMORY_TYPE: UefiMemoryType = UefiMemoryType::new(0x80000000);

pub const KERNEL_STACK_UEFI_MEMORY_TYPE: UefiMemoryType = UefiMemoryType::new(0x80000001);

c_enum!(
    pub enum MemoryType : u32 {
        // Usable section.
        CONVENTIONAL = 0,
        PERSISTENT = 1,

        // Unusable section.
        UNUSABLE = 0 | MEMORY_UNUSABLE_BIT,
        RESERVED = 1 | MEMORY_UNUSABLE_BIT,
        FIRMWARE = 2 | MEMORY_UNUSABLE_BIT,
        BOOT_RECLAIM = 3 | MEMORY_UNUSABLE_BIT,
        KERNEL = 4 | MEMORY_UNUSABLE_BIT,
        KERNEL_STACK = 5 | MEMORY_UNUSABLE_BIT,
        MEMORY_MAPPED_IO = 6 | MEMORY_UNUSABLE_BIT,
        ACPI_NVS = 7 | MEMORY_UNUSABLE_BIT,
        ACPI_RECLAIM = 8 | MEMORY_UNUSABLE_BIT,
    }
);

impl MemoryType {
    pub fn is_usable(self) -> bool {
        let value = u32::from(self);
        !value.get_bit(31)
    }
}

impl From<UefiMemoryType> for MemoryType {
    fn from(value: UefiMemoryType) -> Self {
        match value {
            UefiMemoryType::RESERVED => MemoryType::RESERVED,
            UefiMemoryType::LOADER_CODE => MemoryType::BOOT_RECLAIM,
            UefiMemoryType::LOADER_DATA => MemoryType::BOOT_RECLAIM,
            UefiMemoryType::BOOT_SERVICES_CODE => MemoryType::BOOT_RECLAIM,
            UefiMemoryType::BOOT_SERVICES_DATA => MemoryType::BOOT_RECLAIM,
            UefiMemoryType::RUNTIME_SERVICES_CODE => MemoryType::FIRMWARE,
            UefiMemoryType::RUNTIME_SERVICES_DATA => MemoryType::FIRMWARE,
            UefiMemoryType::CONVENTIONAL => MemoryType::CONVENTIONAL,
            UefiMemoryType::UNUSABLE => MemoryType::UNUSABLE,
            UefiMemoryType::ACPI_RECLAIM => MemoryType::ACPI_RECLAIM,
            UefiMemoryType::ACPI_MEMORY_NVS => MemoryType::ACPI_NVS,
            UefiMemoryType::MEMORY_MAPPED_IO => MemoryType::MEMORY_MAPPED_IO,
            UefiMemoryType::MEMORY_MAPPED_IO_PORT_SPACE => MemoryType::MEMORY_MAPPED_IO,
            UefiMemoryType::PAL_CODE => MemoryType::RESERVED,
            UefiMemoryType::PERSISTENT_MEMORY => MemoryType::PERSISTENT,
            KERNEL_UEFI_MEMORY_TYPE => MemoryType::KERNEL,
            KERNEL_STACK_UEFI_MEMORY_TYPE => MemoryType::KERNEL_STACK,
            _ => MemoryType::RESERVED,
        }
    }
}
