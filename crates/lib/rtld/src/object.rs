use std::ops::Range;

use ::object::{elf, LittleEndian};

use crate::{Result, RtldError, Section};

#[repr(C)]
pub struct ModuleObject {
    next: *mut Self,
    prev: *mut Self,
    rela_or_rel_plt: *mut (),
    rela_or_rel: *mut (),
    module_base: *mut u8,
    dynamic: *mut elf::Dyn64<LittleEndian>,
    is_rela: bool,
    rela_or_rel_plt_size: u64,
    dt_init: extern "C" fn(),
    dt_fini: extern "C" fn(),
    hash_bucket: *mut u32,
    hash_chain: *mut u32,
    dynstr: *mut u8,
    dynsym: *mut elf::Sym64<LittleEndian>,
    dynstr_size: u64,
    got: *mut *mut (),
    rela_dyn_size: u64,
    rel_dyn_size: u64,
    rel_count: u64,
    rela_count: u64,
    hash_nchain_value: u64,
    hash_nbucket_value: u64,
    got_stub_ptr: *mut (),
    soname_idx: u64,
    nro_size: usize,
    cannot_revert_symbols: usize,
}

impl ModuleObject {
    /// Gets the name of this module object, the same way as [Atmosphere](https://github.com/Atmosphere-NX/Atmosphere/blob/1afb184c143f4319e5d6d4ea27260e61830c42a0/stratosphere/creport/source/creport_modules.cpp#L188-L216)
    pub fn name(&self) -> Result<&'static str> {
        let info = nx::query_memory(self.module_base as u64);
        let ro_info = nx::query_memory(info.addr + info.size);

        if ro_info.perm & 0x2 == 0 {
            return Err(RtldError::RoNotReadOnly);
        }

        let rw_data_offset = unsafe { *(ro_info.addr as *const u32) };

        if rw_data_offset as u64 + info.addr == ro_info.addr + ro_info.size {
            return Err(RtldError::DeprecatedFormat);
        }

        let name_length = unsafe { *(ro_info.addr as *const i32).add(1) };

        if name_length <= 0 {
            return Err(RtldError::InvalidNameLength(name_length));
        }

        let name_slice = unsafe {
            std::slice::from_raw_parts((ro_info.addr as *const u8).add(0x8), name_length as usize)
        };

        std::str::from_utf8(name_slice).map_err(Into::into)
    }

    pub fn get_address_range(&self, section: Section) -> Range<u64> {
        match section {
            Section::Text => {
                let info = nx::query_memory(self.module_base as u64);
                info.addr..info.addr + info.size
            }
            Section::RoData => {
                let text = self.get_address_range(Section::Text);
                let info = nx::query_memory(text.end);
                info.addr..info.addr + info.size
            }
            Section::Data => {
                let ro_data = self.get_address_range(Section::RoData);
                let info = nx::query_memory(ro_data.end);
                info.addr..info.addr + info.size
            }
        }
    }
}

#[repr(C)]
pub(crate) struct ModuleObjectList {
    front: *mut ModuleObject,
    back: *mut ModuleObject,
}

impl ModuleObjectList {
    pub fn iter(&self) -> ModuleObjectListIterator {
        ModuleObjectListIterator {
            end: self as *const ModuleObjectList as *const ModuleObject,
            current: self.front,
        }
    }
}

pub(crate) struct ModuleObjectListIterator {
    end: *const ModuleObject,
    current: *const ModuleObject,
}

impl Iterator for ModuleObjectListIterator {
    type Item = &'static ModuleObject;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let next = unsafe { (*self.current).next };
            let current = unsafe { &*self.current };
            self.current = next;
            Some(current)
        }
    }
}

extern "C" {
    #[link_name = "_ZN2nn2ro6detail15g_pAutoLoadListE"]
    pub(crate) static AUTO_LOAD_LIST: &'static mut ModuleObjectList;

    #[link_name = "_ZN2nn2ro6detail17g_pManualLoadListE"]
    pub(crate) static MANUAL_LOAD_LIST: &'static mut ModuleObjectList;
}
