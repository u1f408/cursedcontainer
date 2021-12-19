#![cfg_attr(not(feature = "std"), no_std)]

//! **Please, for the love of all things good in the world, do not use this crate unless you've
//! read the code thoroughly and understand the implications of using such an utterly unsafe piece
//! of software. _You have been warned._**
//!
//! A "cursed" container with an opaque key type, that allows for retrieving mutable references to
//! the objects contained within.
//!
//! The `CursedContainer` is a synchronized init-on-first-use `Vec<T>` wrapper, where the objects
//! within the inner Vec are themselves contained within an [`UnsafeCell`], allowing for retrieval
//! of mutable references to those objects without a mutable reference to the `CursedContainer`
//! itself.
//!
//! This design allows for assigning a `CursedContainer` to a `static` variable, like so:
//!
//! ```
//! # use cursedcontainer::CursedContainer;
//! static CONTAINER: CursedContainer<usize> = CursedContainer::new();
//!
//! let key = CONTAINER.insert(69420);
//! assert_eq!(CONTAINER.get(key), Some(&mut 69420));
//! ```
//!
//! # Safety
//!
//! Hahah, good joke.
//!
//! There is some safety built into `CursedContainer` around initialization race conditions, but
//! accessing items within the container is unsafe by design - it allows for retrieving multiple
//! mutable references to the same object stored within the container.
//!
//! It is the responsibility of your application code to make sure that things don't go horribly
//! wrong when using a `CursedContainer`.
//!
//! # But... why?
//!
//! This crate was developed for the author's hobby operating system project, and the lack of
//! safety in here is designed for that purpose.

extern crate alloc;

use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use spin::Mutex;

/// The container ID value of the next-created container, incremented on container initialization.
static NEXT_CONTAINER_ID: AtomicUsize = AtomicUsize::new(1);

/// Opaque key type for the "cursed" container
#[derive(Copy, Clone, PartialEq)]
pub struct CursedKey {
    pub(crate) container_id: usize,
    pub(crate) key: usize,
}

/// A "cursed" container
pub struct CursedContainer<T> {
    initialized: AtomicBool,
    lock: Mutex<()>,
    id: AtomicUsize,
    inner: UnsafeCell<Option<Vec<Option<UnsafeCell<T>>>>>,
}

unsafe impl<T: Send> Sync for CursedContainer<T> {}
unsafe impl<T: Send> Send for CursedContainer<T> {}

// implementation details
impl<T> CursedContainer<T> {
    fn get_inner<'a>(&self) -> &'a mut Vec<Option<UnsafeCell<T>>> {
        match unsafe { (&mut *self.inner.get()).as_mut() } {
            None => panic!("attempt to use uninitialized CursedContainer"),
            Some(r) => r.as_mut(),
        }
    }
}

// public API
impl<T> CursedContainer<T> {
    /// Create a new container
    pub const fn new() -> CursedContainer<T> {
        CursedContainer {
            initialized: AtomicBool::new(false),
            lock: Mutex::new(()),
            id: AtomicUsize::new(0),
            inner: UnsafeCell::new(None),
        }
    }

    /// Initialize this container, if it hasn't already been initialized
    pub fn init(&self) {
        // First check to see if we're uninitialized
        if !self.initialized.load(Ordering::Acquire) {
            // We might not be initialized, lock and check again
            let _lock = self.lock.lock();
            if !self.initialized.load(Ordering::Relaxed) {
                // We're uninitialized, let's set up
                let _ = core::mem::replace(unsafe { &mut *self.inner.get() }, Some(Vec::new()));
                self.id.store(
                    NEXT_CONTAINER_ID.fetch_add(1, Ordering::Release),
                    Ordering::Release,
                );

                self.initialized.store(true, Ordering::Release);
            }
        }
    }

    /// Insert a value into the container, returning a key object
    pub fn insert(&self, value: T) -> CursedKey {
        self.init();

        // Insertion requires locking
        let _lock = self.lock.lock();

        let inner = self.get_inner();
        inner.push(Some(UnsafeCell::new(value)));
        CursedKey {
            container_id: self.id.load(Ordering::Relaxed),
            key: inner.len() - 1,
        }
    }

    /// Get a mutable reference to a value in the container
    pub fn get<'a>(&self, key: CursedKey) -> Option<&'a mut T> {
        self.init();

        if key.container_id != self.id.load(Ordering::Relaxed) {
            return None;
        }

        let inner = self.get_inner();
        if let Some(vc) = inner.get(key.key) {
            if let Some(vci) = vc {
                return Some(unsafe { &mut *vci.get() });
            }
        }

        None
    }
}
