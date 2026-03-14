use std::fmt::Debug;

use skyline::libc::c_char;

#[repr(C)]
pub struct String {
    data: [u8; 0x18],
}

impl String {
    fn is_compact(&self) -> bool {
        self.data[0] & 1 == 0
    }

    pub fn len(&self) -> usize {
        if self.is_compact() {
            (self.data[0] >> 1) as usize
        } else {
            unsafe { *(self.data.as_ptr().add(0x8) as *const usize) }
        }
    }

    pub fn max_length(&self) -> usize {
        if self.is_compact() {
            0x16
        } else {
            unsafe { *(self.data.as_ptr() as *const usize) - 1 }
        }
    }

    pub fn capacity(&self) -> usize {
        if self.is_compact() {
            0x17
        } else {
            unsafe { *(self.data.as_ptr() as *const usize) }
        }
    }

    pub fn c_string(&self) -> *const c_char {
        if self.is_compact() {
            unsafe { self.data.as_ptr().add(1) as *const c_char }
        } else {
            unsafe { *(self.data.as_ptr().add(0x10) as *const *const c_char) }
        }
    }

    pub fn to_str(&self) -> &str {
        let len = self.len();
        let c_string = self.c_string();
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(c_string, len)) }
    }

    pub fn to_string(&self) -> std::string::String {
        self.to_str().to_string()
    }
}

impl Debug for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}
