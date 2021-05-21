//**************************************************************************************************
// buddy.rs                                                                                        *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::allocators::AllocatorInterface;
use crate::structures::{DoublyLinkedList, DoublyLinkedNode};
use core::{cmp, mem};

pub const MIN_ALLOCATION_SIZE: usize = mem::size_of::<DoublyLinkedNode<()>>().next_power_of_two();

pub const BASE_LEVEL: usize = power_of_two_exp_from_size(MIN_ALLOCATION_SIZE);

pub struct Allocator<TInterface: AllocatorInterface, const LEVELS: usize> {
    levels: [DoublyLinkedList<()>; LEVELS],
    interface: TInterface,
}

impl<TInterface: AllocatorInterface, const LEVELS: usize> Allocator<TInterface, LEVELS> {
    pub const fn new(interface: TInterface) -> Self {
        Self {
            levels: [DoublyLinkedList::<()>::new(); LEVELS],
            interface,
        }
    }

    pub unsafe fn alloc(&mut self, size: usize) -> *mut u8 {
        let power_of_two = get_allocation_size(size);
        let index = get_index(power_of_two);

        assert!(index < LEVELS);

        self.pop(index) as *mut u8
    }

    pub unsafe fn dealloc(&mut self, address: *mut u8, size: usize) {
        let power_of_two = get_allocation_size(size);
        let index = get_index(power_of_two);

        assert!(index < LEVELS);

        self.push(index, address as *mut DoublyLinkedNode<()>);
    }

    unsafe fn pop(&mut self, level: usize) -> *mut DoublyLinkedNode<()> {
        let mut level_list = &mut self.levels[level];

        if let Some(node) = level_list.pop() {
            node
        } else if level == LEVELS - 1 {
            let level_size = get_size_from_index(LEVELS - 1);

            let pages_len = cmp::max(level_size / TInterface::PAGE_SIZE, 1);
            let pages_ptr = self.interface.get_pages(pages_len);

            let extra_nodes = ((pages_len * TInterface::PAGE_SIZE) / level_size) - 1;

            for i in 0..extra_nodes {
                let offset = (i + 1) * level_size;
                let extra_node = pages_ptr.add(offset) as *mut DoublyLinkedNode<()>;
                level_list.push(extra_node);
            }

            pages_ptr as *mut DoublyLinkedNode<()>
        } else {
            // Get the node from the next level and split it in half. Store the first half
            // and then return the other.

            let node_area_size = get_size_from_index(level);

            let new_node_a = self.pop(level + 1);
            let next_level_ptr = new_node_a as *mut u8;

            let new_node_b = next_level_ptr.add(node_area_size) as *mut DoublyLinkedNode<()>;

            self.levels[level].push(new_node_b);

            new_node_a
        }
    }

    unsafe fn push(&mut self, level: usize, node: *mut DoublyLinkedNode<()>) {
        let mut level_list = &mut self.levels[level];

        // Try and see if the matching node is available for combining.

        if level < LEVELS - 1 {
            let base_address = node as *mut u8;
            let level_size = get_size_from_index(level);

            let even_node = (node as usize) % 2 == 0;

            let other_address;
            let next_start_address;

            if even_node {
                other_address = base_address.add(level_size) as *mut DoublyLinkedNode<()>;
                next_start_address = node;
            } else {
                other_address = base_address.sub(level_size) as *mut DoublyLinkedNode<()>;
                next_start_address = other_address;
            }

            // Check and see if the level list contains the other node.

            let search_result = level_list
                .iter()
                .find(|search_node| *search_node == other_address);

            // If the level list does contain the other node then remove it and push the
            // combined nodes into the next list.

            if let Some(matching_node) = search_result {
                level_list.remove(matching_node);
                self.push(level + 1, next_start_address);
                return;
            }
        }

        level_list.push(node as *mut DoublyLinkedNode<()>);
    }
}

unsafe impl<TInterface: AllocatorInterface, const LEVELS: usize> Send
    for Allocator<TInterface, LEVELS>
{
}

pub fn get_allocation_size(size: usize) -> usize {
    cmp::max(size.next_power_of_two(), MIN_ALLOCATION_SIZE)
}

const fn get_index(size: usize) -> usize {
    power_of_two_exp_from_size(size) - BASE_LEVEL
}

const fn get_size_from_index(index: usize) -> usize {
    size_from_power_of_two_exp(index + BASE_LEVEL)
}

const fn size_from_power_of_two_exp(exp: usize) -> usize {
    2_usize.pow(exp as u32)
}

const fn power_of_two_exp_from_size(size: usize) -> usize {
    size.trailing_zeros() as usize
}
