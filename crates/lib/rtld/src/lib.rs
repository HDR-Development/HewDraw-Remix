//! Crate for interacting with RTLD systems from HDR
//!
//! This code was also taken from [skyline-ex](https://github.com/Skyline-ex/skylinex),
//! however the original author of the code which inspired this would be Thog.
use std::str::Utf8Error;
use thiserror::Error;

mod object;

use self::object::{ModuleObject, AUTO_LOAD_LIST, MANUAL_LOAD_LIST};

#[derive(Error, Debug)]
pub enum RtldError {
    #[error(".rodata section is not read only")]
    RoNotReadOnly,

    #[error("Module is in a deprecated format")]
    DeprecatedFormat,

    #[error("Module name has invalid length ({0})")]
    InvalidNameLength(i32),

    #[error("{0}")]
    InvalidUtf8(#[from] Utf8Error),
}

type Result<T> = std::result::Result<T, RtldError>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Section {
    Text,
    RoData,
    Data,
}

pub fn find_module_for_address(address: u64, section: Section) -> Option<&'static ModuleObject> {
    unsafe {
        AUTO_LOAD_LIST
            .iter()
            .chain(MANUAL_LOAD_LIST.iter())
            .find(|object| object.get_address_range(section).contains(&address))
    }
}

pub fn find_module_by_name(name: &str) -> Option<&'static ModuleObject> {
    let mut objects = unsafe { AUTO_LOAD_LIST.iter().chain(MANUAL_LOAD_LIST.iter()) };

    objects.find(|object| object.name().unwrap_or("__invalid_name") == name)
}

pub fn is_valid_pointer_for_section(address: u64, section: Section) -> bool {
    find_module_for_address(address, section).is_some()
}
