#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ops::DerefMut;

use parking_lot::lock_api::RawRwLock as RawRwLockTrait;
use parking_lot::RawRwLock;

/// Wraps a 'T' that can only be accessed through global mutexes at zero memory overhead per object.
#[derive(Debug)]
#[repr(transparent)]
pub struct ShardedRwLock<T>(UnsafeCell<T>);

// impl<T> ShardedRwLock<T> {
//     /// Create a new ShardedRwLock from the given value.
//     pub fn new(value: T) -> Self {
//         ShardedRwLock(UnsafeCell::new(value))
//     }
// 
//     // idea borrowed from crossbeam SeqLock
//     fn get_mutex(&self) -> &'static RawRwLock {
//         const LEN: usize = 127;
//         #[repr(align(128))] // cache line aligned
//         struct Locks([RawRwLock; LEN]);
//         static LOCKS: Locks = Locks([RawRwLock::INIT; LEN]);
//         &LOCKS.0[self as *const ShardedRwLock<T> as usize % LEN]
//     }
// 
//     /// Acquire a global sharded lock guard with unlock on drop semantics
//     pub fn lock(&self) -> ShardedRwLockGuard<T> {
//         self.get_mutex().lock();
//         ShardedRwLockGuard(&self)
//     }
// 
//     /// Acquire a global sharded lock guard with unlock on drop semantics
//     pub fn try_lock(&self) -> Option<ShardedRwLockGuard<T>> {
//         self.get_mutex()
//             .try_lock()
//             .then(|| ShardedRwLockGuard(&self))
//     }
// }
// 
// /// The guard returned from locking a ShardedRwLock. Dropping this will unlock the mutex.
// /// Access to the underlying value is done by dereferencing this guard.
// pub struct ShardedRwLockGuard<'a, T>(&'a ShardedRwLock<T>);
// 
// impl<T> Deref for ShardedRwLockGuard<'_, T> {
//     type Target = T;
// 
//     fn deref(&self) -> &Self::Target {
//         unsafe {
//             // SAFETY: the guard gurantees that the we own the object
//             &*self.0 .0.get()
//         }
//     }
// }
// 
// impl<T> DerefMut for ShardedRwLockGuard<'_, T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         unsafe {
//             // SAFETY: the guard gurantees that the we own the object
//             &mut *self.0 .0.get()
//         }
//     }
// }
// 
// impl<T> AsRef<T> for ShardedRwLockGuard<'_, T> {
//     fn as_ref(&self) -> &T {
//         unsafe {
//             // SAFETY: the guard gurantees that the we own the object
//             &*self.0 .0.get()
//         }
//     }
// }
// 
// impl<T> AsMut<T> for ShardedRwLockGuard<'_, T> {
//     fn as_mut(&mut self) -> &mut T {
//         unsafe {
//             // SAFETY: the guard gurantees that the we own the object
//             &mut *self.0 .0.get()
//         }
//     }
// }
// 
// impl<T> Drop for ShardedRwLockGuard<'_, T> {
//     fn drop(&mut self) {
//         unsafe {
//             // SAFETY: the guard gurantees that the we have the lock
//             self.0.get_mutex().unlock();
//         }
//     }
// }
// 
// #[cfg(test)]
// mod tests {
//     use crate::ShardedRwLock;
// 
//     #[test]
//     fn smoke() {
//         let x = ShardedRwLock::new(123);
//         assert_eq!(*x.lock(), 123);
//     }
// 
//     #[test]
//     fn simple_lock() {
//         let x = ShardedRwLock::new(123);
//         assert_eq!(*x.lock(), 123);
// 
//         let mut guard = x.lock();
// 
//         *guard = 234;
//         drop(guard);
// 
//         assert_eq!(*x.lock(), 234);
//     }
// }
