//**************************************************************************************************
// spinlock.rs                                                                                     *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct Spinlock<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> Spinlock<T> {
    pub const fn new(value: T) -> Self {
        Spinlock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(value),
        }
    }

    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

impl<T: ?Sized> Spinlock<T> {
    pub fn try_lock(&self) -> Option<SpinlockGuard<T>> {
        if self.lock.compare_and_swap(false, true, Ordering::Acquire) {
            Some(SpinlockGuard::new(&self))
        } else {
            None
        }
    }

    pub fn lock(&self) -> SpinlockGuard<T> {
        while !self.lock.compare_and_swap(false, true, Ordering::Acquire) {
            arch::sync::spin_loop_hint();
        }
        SpinlockGuard::new(&self)
    }
}

unsafe impl<T: ?Sized + Send> Send for Spinlock<T> {}

unsafe impl<T: ?Sized + Send> Sync for Spinlock<T> {}

pub struct SpinlockGuard<'a, T: ?Sized + 'a> {
    spinlock: &'a Spinlock<T>,
    arch_state: arch::sync::LockState,
}

impl<'a, T: ?Sized> SpinlockGuard<'a, T> {
    fn new(spinlock: &'a Spinlock<T>) -> Self {
        SpinlockGuard {
            spinlock,
            arch_state: arch::sync::start_lock(),
        }
    }
}

impl<'a, T: ?Sized> Deref for SpinlockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.spinlock.data.get() }
    }
}

impl<'a, T: ?Sized> DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.spinlock.data.get() }
    }
}

impl<'a, T: ?Sized> Drop for SpinlockGuard<'a, T> {
    fn drop(&mut self) {
        arch::sync::end_lock(self.arch_state);
        self.spinlock.lock.store(false, Ordering::Release);
    }
}

impl<'a, T: ?Sized> !Send for SpinlockGuard<'a, T> {}

unsafe impl<'a, T: ?Sized + Sync> Sync for SpinlockGuard<'a, T> {}
