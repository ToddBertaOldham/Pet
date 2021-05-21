//**************************************************************************************************
// doubly_linked_list.rs                                                                           *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::ptr;

#[derive(Copy, Clone)]
pub struct DoublyLinkedList<T> {
    list: Option<*mut DoublyLinkedNode<T>>,
}

impl<T> DoublyLinkedList<T> {
    pub const fn new() -> Self {
        Self { list: None }
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_none()
    }

    pub unsafe fn pop(&mut self) -> Option<*mut DoublyLinkedNode<T>> {
        if let Some(list_ptr) = self.list {
            let list = &mut *list_ptr;

            let last_element_ptr = list.previous;
            let last_element = &mut *last_element_ptr;

            if list_ptr == last_element_ptr {
                self.list = None;
            } else {
                let second_last_element_ptr = last_element.previous;
                let second_last_element = &mut *second_last_element_ptr;

                list.previous = second_last_element_ptr;
                second_last_element.next = list_ptr;
            }

            Some(last_element)
        } else {
            None
        }
    }

    pub unsafe fn push(&mut self, node_ptr: *mut DoublyLinkedNode<T>) {
        let node = &mut *node_ptr;

        if let Some(list_ptr) = self.list {
            let list = &*list_ptr;

            let last_element_ptr = list.previous;
            let last_element = &mut *last_element_ptr;

            last_element.next = node_ptr;
            node.previous = last_element_ptr;
            node.next = list_ptr;
        } else {
            self.list = Some(node_ptr);
            node.next = node_ptr;
            node.previous = node_ptr;
        }
    }

    pub unsafe fn remove(&mut self, node_ptr: *mut DoublyLinkedNode<T>) {
        let node = &*node_ptr;

        let previous_node = &mut *node.previous;
        let next_node = &mut *node.next;

        previous_node.next = node.next;
        next_node.previous = node.previous;
    }

    pub unsafe fn iter(&mut self) -> DoublyLinkedNodeIterator<T> {
        DoublyLinkedNodeIterator {
            list: self.list,
            next: self.list.unwrap_or(ptr::null_mut()),
        }
    }
}

#[repr(C)]
pub struct DoublyLinkedNode<T> {
    previous: *mut DoublyLinkedNode<T>,
    next: *mut DoublyLinkedNode<T>,
    value: T,
}

pub struct DoublyLinkedNodeIterator<T> {
    list: Option<*mut DoublyLinkedNode<T>>,
    next: *mut DoublyLinkedNode<T>,
}

impl<T> Iterator for DoublyLinkedNodeIterator<T> {
    type Item = *mut DoublyLinkedNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let list_ptr = self.list?;

        if self.next == list_ptr {
            return None;
        }

        let next_value = unsafe { &*self.next };
        self.next = next_value.next;

        Some(self.next)
    }
}
