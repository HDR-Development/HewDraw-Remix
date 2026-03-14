#![cfg_attr(not(feature = "std"), no_std)]
#![feature(const_if_match, const_loop, track_caller, proc_macro_hygiene)]
#![feature(asm)]
#![feature(associated_type_bounds)]
#![feature(simd_ffi)] // lol sorry jam

pub mod crc32;

pub mod params;

#[doc(hidden)]
pub mod cpp;

#[doc(hidden)]
pub mod common;

#[doc(inline)]
pub use common::root::*;

#[doc(inline)]
pub use cpp::root::*;

// Find the hash40 of a given string
pub const fn hash40(string: &str) -> u64 {
    let bytes = string.as_bytes();

    ((bytes.len() as u64) << 32) + crc32::crc32(bytes) as u64
}

impl phx::Hash40 {
    pub fn new(string: &str) -> Self {
        Self {
            hash: hash40(string),
        }
    }

    pub fn new_raw(raw: u64) -> Self {
        Self { hash: raw }
    }
}

mod lua_const;

#[repr(C)]
pub struct CppHash40MapEntry<T> {
    pub next: *mut Self,
    pub key: phx::Hash40,
    pub also_key: phx::Hash40,
    pub value: T
}

#[repr(C)]
#[derive(Debug, Copy, Clone)] // really bad, don't do this :P
pub struct CppHash40Map<T: Sized> {
    pub buckets: *const *mut CppHash40MapEntry<T>,
    pub bucket_count: u64
}

impl<T: Sized> CppHash40Map<T> {
    pub fn get<'a>(&'a self, key: &phx::Hash40) -> Option<&'a T> {
        if !self.buckets.is_null() {
            unsafe {
                let bucket_idx = key.hash % self.bucket_count;
                let mut current = *self.buckets.add(bucket_idx as usize);
                if !current.is_null() {
                    current = (*current).next;
                    while !current.is_null() {
                        let current_key = (*current).key;
                        if current_key != *key && (current_key.hash % self.bucket_count) != bucket_idx {
                            break;
                        }
                        if current_key == *key && (*current).also_key == *key {
                            return Some(&(*current).value);
                        }
                        current = (*current).next;
                    }
                }
                None
            }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: &phx::Hash40) -> Option<&mut T> {
        let bucket_idx = key.hash % self.bucket_count;
        if !self.buckets.is_null() {
            unsafe {
                let mut current = *self.buckets.add(bucket_idx as usize);
                if !current.is_null() {
                    current = (*current).next;
                    while !current.is_null() {
                        let current_key = (*current).key;
                        if current_key != *key && (current_key.hash % self.bucket_count) != bucket_idx {
                            break;
                        }
                        if current_key == *key && (*current).also_key == *key {
                            return Some(&mut (*current).value);
                        }
                        current = (*current).next;
                    }
                }
                None
            }
        } else {
            None
        }
    }
}