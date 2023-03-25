//! This library is a utility for modifying and calling tables of virtual functions (vtables)
//! from a C++ source.
//!
//! This library makes no assumptions about where vtables are located, and it is expected that
//! the caller is properly obtaining a reference to a vtable.

use rtld::Section;
use thiserror::Error;

/// Virtual table type with no strongly defined functions
pub type Vtable = *const *const ();

/// Error for operations related to rebuilding and restoring vtables
#[derive(Debug, Error)]
pub enum VirtManipError {
    #[error("The vtable exhausts addressable memory after {0} entries")]
    OutOfAddressableMemory(usize),

    #[error("The vtable contains a non-pointer field at entry {0}")]
    InvalidPointerField(usize),

    #[error("This vtable was not in a .rodata section, was it already rebuilt?")]
    NotInRoData,

    #[error("This vtable did not have the right magic, expected {0:#x} but found {1:#x}")]
    WrongMagic(u64, u64),

    #[error("Index greater than length of the vtable")]
    IndexOutOfBounds,
}

type Result<T> = std::result::Result<T, VirtManipError>;

fn discover_num_entries(vtable: Vtable) -> Result<usize> {
    let Some(object) = rtld::find_module_for_address(vtable as u64, Section::RoData) else {
        return Err(VirtManipError::NotInRoData);
    };

    let text_range = object.get_address_range(Section::Text);
    let rodata_range = object.get_address_range(Section::RoData);

    let mut current = vtable;

    let mut count = 0;

    loop {
        if !rodata_range.contains(&(current as u64)) {
            break Err(VirtManipError::OutOfAddressableMemory(count));
        }

        let address = unsafe { (*current) as u64 };

        if address == 0 {
            break Ok(count);
        }

        if !text_range.contains(&address) {
            break Err(VirtManipError::InvalidPointerField(count));
        }

        count += 1;

        current = unsafe { current.add(1) };
    }
}

/// Rebuilds a vtable
/// # Arguments
/// * `magic` - The 8-byte magic to associate with this vtable
/// * `known_size` - The size of the vtable (in entries) if it is known
/// * `extra_entries` - The number of extra entries to allocate in this vtable
/// * `vtable` - The vtable to modify
pub fn rebuild(
    magic: [u8; 8],
    known_size: Option<usize>,
    extra_entries: usize,
    vtable: &mut Vtable,
) -> Result<()> {
    let magic = u64::from_le_bytes(magic);

    let size = if let Some(size) = known_size {
        size
    } else {
        discover_num_entries(*vtable)?
    };

    let mut entries = vec![0u64; extra_entries];

    let old_vtable = *vtable;

    let Some(rodata_range) = rtld::find_module_for_address(old_vtable as u64, Section::RoData).map(|module| module.get_address_range(Section::RoData)) else {
        return Err(VirtManipError::NotInRoData);
    };

    entries.push(0);
    entries.push(0);
    entries.push(old_vtable as u64);
    entries.push(extra_entries as u64);
    entries.push(magic);

    for x in 0..size {
        unsafe {
            if !rodata_range.contains(&(old_vtable.add(x) as u64)) {
                return Err(VirtManipError::OutOfAddressableMemory(x));
            }

            entries.push(*old_vtable.add(x) as u64);
        }
    }

    entries[0] = entries.len() as u64;
    entries[1] = entries.capacity() as u64;

    let ptr = entries.as_ptr();
    std::mem::forget(entries);

    *vtable = unsafe { ptr.add(5) as Vtable };

    Ok(())
}

/// Restores a vtable to its original pointer
///
/// This should ALWAYS be called before the destructor, as unless you are 100%
/// certain that the destructor in a vtable is *never* inlined, calling this inside
/// of a replaced destructor is not safe and could case memory leaks
///
/// # Arguments
/// * `magic` - The 8-byte magic associated with this vtable
/// * `vtable` - The vtable to restore
pub fn restore(magic: [u8; 8], vtable: &mut Vtable) -> Result<()> {
    let magic = u64::from_le_bytes(magic);

    let new_vtable = *vtable;
    let vtable_magic = unsafe { *new_vtable.sub(1) as u64 };

    if vtable_magic != magic {
        return Err(VirtManipError::WrongMagic(magic, vtable_magic));
    }
    let (vec_len, vec_cap, old_vtable, num_entries) = unsafe {
        (
            *new_vtable.sub(5) as usize,
            *new_vtable.sub(4) as usize,
            *new_vtable.sub(3) as Vtable,
            *new_vtable.sub(2) as usize,
        )
    };

    *vtable = old_vtable;

    let vec_ptr = unsafe { new_vtable.sub(5 + num_entries) as *mut u64 };

    unsafe {
        drop(Vec::from_raw_parts(vec_ptr, vec_len, vec_cap));
    }

    Ok(())
}

/// Retrieves the extra entries from the rebuilt vtable
/// # Arguments
/// * `magic` - The 8-byte magic associated with the vtable
/// * `vtable` - The vtable to pull the entries from
pub fn entries(magic: [u8; 8], vtable: &Vtable) -> Result<&'static mut [u64]> {
    let magic = u64::from_le_bytes(magic);

    let vtable = *vtable;

    let vtable_magic = unsafe { *vtable.sub(1) as u64 };

    if vtable_magic != magic {
        return Err(VirtManipError::WrongMagic(magic, vtable_magic));
    }

    unsafe {
        let num_entries = *vtable.sub(2) as usize;
        let ptr = *vtable.sub(5 + num_entries) as *mut u64;
        Ok(std::slice::from_raw_parts_mut(ptr, num_entries))
    }
}

/// Replaces a function in the rebuilt vtable
/// # Arguments
/// * `magic` - The 8-byte magic associated with the vtable
/// * `entry_number` - The entry in the vtable to replace
/// * `function` - The function to put in the vtable
/// * `vtable` - The vtable to replace inside of
pub fn replace_function(
    magic: [u8; 8],
    entry_number: usize,
    function: *const (),
    vtable: &Vtable,
) -> Result<()> {
    let magic = u64::from_le_bytes(magic);

    let vtable = *vtable;

    let vtable_magic = unsafe { *vtable.sub(1) as u64 };

    if vtable_magic != magic {
        return Err(VirtManipError::WrongMagic(magic, vtable_magic));
    }

    let num_funcs = unsafe { *vtable.sub(5) as usize - *vtable.sub(2) as usize };

    if num_funcs <= entry_number {
        return Err(VirtManipError::IndexOutOfBounds);
    }

    unsafe {
        *(vtable as *mut *const ()).add(entry_number) = function;
    }

    Ok(())
}

/// Restores a function from the original vtable to the new one
/// # Arguments
/// * `magic` - The 8-byte magic associated with the vtable
/// * `entry_number` - The entry in the vtable to replace
/// * `vtable` - The vtable to replace inside of
pub fn restore_function(magic: [u8; 8], entry_number: usize, vtable: &Vtable) -> Result<()> {
    let magic = u64::from_le_bytes(magic);

    let vtable = *vtable;

    let vtable_magic = unsafe { *vtable.sub(1) as u64 };

    if vtable_magic != magic {
        return Err(VirtManipError::WrongMagic(magic, vtable_magic));
    }

    let num_funcs = unsafe { *vtable.sub(5) as usize - *vtable.sub(2) as usize };

    if num_funcs <= entry_number {
        return Err(VirtManipError::IndexOutOfBounds);
    }

    unsafe {
        let function = *(*vtable.sub(3) as Vtable).add(entry_number);
        *(vtable as *mut *const ()).add(entry_number) = function;
    }

    Ok(())
}
