//**************************************************************************************************
// allocator.rs                                                                                    *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::memory::Frame;
use crate::spinlock::Spinlock;
use core::alloc::{GlobalAlloc, Layout};
use core::mem;
use core::ptr;
use memory::{Align, CheckAlignment};

#[derive(Debug)]
pub struct Allocator {
    first_node: *mut Node,
}

impl Allocator {
    pub const MINIMUM_ALLOCATION_SIZE: usize = 16;
    const SPLIT_THRESHOLD: usize = mem::size_of::<Node>() + Self::MINIMUM_ALLOCATION_SIZE;

    pub fn new(heap_start: usize, heap_frame_size: usize) -> Self {
        let mut allocator = Self::uninitialized();
        allocator.init(heap_start, heap_frame_size);
        allocator
    }

    pub const fn uninitialized() -> Self {
        Self {
            first_node: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, heap_start: usize, heap_frame_size: usize) {
        assert!(
            self.first_node.is_null(),
            "Allocator can only be initialized once."
        );

        assert!(heap_start.check_alignment(4096));

        let aligned_heap_start = heap_start.align_up(mem::align_of::<Node>()).unwrap();
        let aligned_heap_size =
            (heap_frame_size * Frame::BYTE_WIDTH) - (aligned_heap_start - heap_start);

        let node = Node {
            next: ptr::null_mut(),
            data_start_offset: aligned_heap_size - mem::size_of::<Node>(),
            data_size: 0,
            data_end_offset: 0,
        };

        self.first_node = aligned_heap_start as *mut Node;

        //TODO Map memory.

        unsafe {
            ptr::write(self.first_node, node);
        }
    }

    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return ptr::null_mut();
        }

        let final_layout = {
            if layout.size() < Self::MINIMUM_ALLOCATION_SIZE {
                Layout::from_size_align(Self::MINIMUM_ALLOCATION_SIZE, layout.align())
                    .expect("Failed to create new layout.")
            } else {
                layout
            }
        };

        for node in self
            .first_node
            .as_mut()
            .expect("Allocator was not initialized before allocating.")
            .iter_inclusive_mut()
        {
            let node_value = &mut *node;

            if !node_value.is_free() {
                continue;
            }

            let memory_address = node_value.memory_address();
            let aligned_memory_address = memory_address.align_up(final_layout.align()).unwrap();

            let mut data_start_offset = aligned_memory_address - memory_address;
            let data_size = final_layout.size();
            let mut data_end_offset;

            let required_memory_size = data_start_offset + data_size;
            let remaining_memory = node_value
                .memory_size()
                .saturating_sub(required_memory_size);

            if remaining_memory == 0 {
                continue;
            }

            let final_node = {
                if data_start_offset >= Self::SPLIT_THRESHOLD {
                    let new_node = node_value.split_at(data_start_offset).as_mut().unwrap();
                    data_start_offset = 0;
                    data_end_offset = new_node.data_start_offset - data_size;
                    new_node
                } else {
                    data_end_offset = remaining_memory;
                    node_value
                }
            };

            let data_end = data_start_offset + data_size;

            let data_end_address = final_node.memory_address() + data_end;
            let aligned_data_end_address =
                data_end_address.align_up(mem::align_of::<Node>()).unwrap();

            let data_end_address_offset = aligned_data_end_address - data_end_address;

            if final_node
                .data_end_offset
                .saturating_sub(data_end_address_offset)
                > Self::SPLIT_THRESHOLD
            {
                final_node.split_at(data_end + data_end_address_offset);
                data_end_offset = data_end_address_offset;
            }

            final_node.data_start_offset = data_start_offset;
            final_node.data_size = data_size;
            final_node.data_end_offset = data_end_offset;

            return final_node.data_address() as *mut u8;
        }

        unimplemented!()
    }

    pub unsafe fn free(&mut self, ptr: *mut u8, layout: Layout) {
        let address = ptr as usize;

        let mut previous: Option<&mut Node> = None;
        for node in self
            .first_node
            .as_mut()
            .expect("Allocator was not initialized before freeing.")
            .iter_inclusive_mut()
        {
            let node_value = &mut *node;

            if node_value.data_address() == address {
                node_value.free();
                node_value.try_merge_with_next();
                if let Some(previous) = previous {
                    previous.try_merge_with_next();
                }
                //TODO Shrink capacity in certain cases.
                return;
            }

            previous = Some(node_value);
        }
    }

    unsafe fn map(&mut self) {}

    unsafe fn expand(&mut self) {}
}

unsafe impl Send for Allocator {}

unsafe impl GlobalAlloc for Spinlock<Allocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().free(ptr, layout);
    }
}

// | Header (Node) |                  Memory Size                    |
// | Header (Node) | Data Start Offset | Data Size | Data End Offset |

#[derive(Debug)]
struct Node {
    next: *mut Node,
    data_start_offset: usize,
    data_size: usize,
    data_end_offset: usize,
}

impl Node {
    unsafe fn try_merge_with_next(&mut self) {
        if !self.is_free() {
            return;
        }

        if let Some(next) = self.next.as_mut() {
            if next.is_free() {
                self.data_start_offset += next.total_size();
                self.next = next.next;
            }
        }
    }

    unsafe fn split_at(&mut self, memory_offset: usize) -> *mut Node {
        let new_total_size = self.memory_size() - memory_offset;
        let new_memory_size = new_total_size - mem::size_of::<Self>();

        let new_node = Self {
            next: self.next,
            data_start_offset: new_memory_size,
            data_size: 0,
            data_end_offset: 0,
        };

        self.data_start_offset -= new_total_size;

        let new_ptr = (self.memory_address() + memory_offset) as *mut Node;
        ptr::write(new_ptr, new_node);
        new_ptr
    }

    fn is_free(&self) -> bool {
        self.data_size == 0
    }

    fn free(&mut self) {
        self.data_start_offset += self.data_size + self.data_end_offset;
        self.data_size = 0;
        self.data_end_offset = 0;
    }

    fn total_size(&self) -> usize {
        mem::size_of::<Self>() + self.memory_size()
    }

    fn memory_size(&self) -> usize {
        self.data_start_offset + self.data_size + self.data_end_offset
    }

    fn memory_address(&self) -> usize {
        (self as *const Node as usize) + mem::size_of::<Self>()
    }

    fn data_address(&self) -> usize {
        self.memory_address() + self.data_start_offset
    }

    fn iter_inclusive_mut(&mut self) -> NodeIteratorMut {
        NodeIteratorMut(self)
    }
}

struct NodeIteratorMut(*mut Node);

impl<'a> Iterator for NodeIteratorMut {
    type Item = *mut Node;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if let Some(inner) = self.0.as_mut() {
                let current = self.0;
                self.0 = inner.next;
                Some(current)
            } else {
                None
            }
        }
    }
}
