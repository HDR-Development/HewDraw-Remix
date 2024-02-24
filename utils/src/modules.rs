mod input;
mod meter;
mod param;
mod var;

use std::sync::Once;

pub use input::*;
pub use meter::*;
pub use param::*;
pub use var::*;

use skyline::hooks::*;
use crate::offsets;

const ADDITIONAL_VTABLE_ENTRIES: usize = 7;
const HDR_BATTLE_OBJECT_MAGIC: u64 = 0x5443454A424F5F48; // H_OBJECT

const VAR_MODULE_OFFSET:            isize = -1;
const PARAM_MODULE_OFFSET:          isize = -2;
const INPUT_MODULE_OFFSET:         isize = -3;
const METER_MODULE_OFFSET:          isize = -4;
const HDR_MAGIC_OFFSET:             isize = -5;
const TOTAL_SIZE_OFFSET:            isize = -6;

pub fn within_address_space(address: u64) -> bool {
    let text = unsafe {
        let text_start = getRegionAddress(Region::Text) as u64;
        let text_end = getRegionAddress(Region::Rodata) as u64;
        text_start..text_end
    };
    text.contains(&address)
}

pub fn count_vtable_length(address: *const *const u64) -> usize {
    if address.is_null() {
        return 0;
    }

    let mut count = 0;
    unsafe {
        let mut current = *address;
        while !current.is_null() && within_address_space(current as u64) {
            count += 1;
            current = *address.add(count);
        }
    }
    count
}

#[allow(dropping_references)]
pub fn recreate_vtable_with_space(address: *const *const u64) -> *mut *mut u64 {
    let vtable_length = count_vtable_length(address);
    let current_vtable = unsafe {
        std::slice::from_raw_parts(address, vtable_length)
    };

    let new_vtable_length = vtable_length + ADDITIONAL_VTABLE_ENTRIES;
    let new_vtable_ptr = unsafe {
        std::alloc::alloc_zeroed(std::alloc::Layout::from_size_align(
            new_vtable_length * std::mem::size_of::<*mut u64>(),
            std::mem::size_of::<*mut u64>()
        ).unwrap()) as *mut *mut u64
    };
    let new_vtable = unsafe {
        std::slice::from_raw_parts_mut(new_vtable_ptr.add(ADDITIONAL_VTABLE_ENTRIES) as *mut *const u64, vtable_length)
    };
    new_vtable.copy_from_slice(current_vtable);
    drop(new_vtable);
    unsafe {
        let new_vtable_ptr = new_vtable_ptr.add(ADDITIONAL_VTABLE_ENTRIES);
        *new_vtable_ptr.offset(HDR_MAGIC_OFFSET) = HDR_BATTLE_OBJECT_MAGIC as _;
        *new_vtable_ptr.offset(TOTAL_SIZE_OFFSET) = new_vtable_length as _;
        new_vtable_ptr
    }
}

pub fn is_hdr_object(address: *const *const u64) -> bool {
    if address.is_null() {
        false
    } else {
        unsafe {
            *address.offset(HDR_MAGIC_OFFSET) == HDR_BATTLE_OBJECT_MAGIC as _
        }
    }
}

pub fn set_entry<T>(address: *mut *mut u64, pointer: *mut T, index: isize) -> bool {
    if !is_hdr_object(address as _) || index <= HDR_MAGIC_OFFSET {
        false
    } else {
        unsafe {
            *address.offset(index) = pointer as _;
        }
        true
    }
}

pub fn get_entry<T>(address: *mut *mut u64, index: isize) -> Option<*mut T> {
    if !is_hdr_object(address as _) || index <= HDR_MAGIC_OFFSET {
        None
    } else {
        Some(unsafe {
            *address.offset(index) as _
        })
    }
}

pub fn clean_hdr_object(address: *mut *mut u64) {
    if let Some(var_module) = get_entry::<VarModule>(address, VAR_MODULE_OFFSET) {
        if !var_module.is_null() {
            unsafe {
                drop(Box::from_raw(var_module))
            }
        }
    }

    if let Some(param_module) = get_entry::<ParamModule>(address, PARAM_MODULE_OFFSET) {
        if !param_module.is_null() {
            unsafe {
                drop(Box::from_raw(param_module))
            }
        }
    }

    if let Some(buffer_module) = get_entry::<InputModule>(address, INPUT_MODULE_OFFSET) {
        if !buffer_module.is_null() {
            unsafe {
                drop(Box::from_raw(buffer_module))
            }
        }
    }

    if let Some(meter_module) = get_entry::<MeterModule>(address, METER_MODULE_OFFSET) {
        if !meter_module.is_null() {
            unsafe {
                drop(Box::from_raw(meter_module))
            }
        }
    }

    if is_hdr_object(address as _) {
        unsafe {
            std::alloc::dealloc(address.offset(TOTAL_SIZE_OFFSET) as _, std::alloc::Layout::from_size_align(
                *address.offset(TOTAL_SIZE_OFFSET) as usize,
                std::mem::size_of::<*mut u64>()
            ).unwrap());
        }
    }
}

#[skyline::hook(offset = offsets::set_fighter_vtable(), inline)]
fn set_fighter_vtable_hook(ctx: &mut InlineCtx) {
    static mut FIGHTER_DESTRUCTOR: usize = 0;
    static mut FIGHTER_DELETER: usize = 0;

    #[skyline::hook(replace = FIGHTER_DESTRUCTOR)]
    unsafe fn fighter_destructor_hook(fighter: *mut *mut *mut u64) {
        clean_hdr_object(*fighter);
        call_original!(fighter)
    }

    #[skyline::hook(replace = FIGHTER_DELETER)]
    unsafe fn fighter_deleter_hook(fighter: *mut *mut *mut u64) {
        clean_hdr_object(*fighter);
        call_original!(fighter)
    }

    static DESTRUCTOR_HOOK: Once = Once::new();
    DESTRUCTOR_HOOK.call_once(|| {
        unsafe {
            FIGHTER_DESTRUCTOR = *((*ctx.registers[8].x.as_ref() as usize + offsets::BATTLE_OBJECT_VTABLE_DESTRUCTOR_OFFSET) as *mut usize) as usize;
            FIGHTER_DELETER = *((*ctx.registers[8].x.as_ref() as usize + offsets::BATTLE_OBJECT_VTABLE_DELETER_OFFSET) as *mut usize) as usize;
        }
        skyline::install_hooks!(fighter_destructor_hook, fighter_deleter_hook);
    });

    unsafe {
        let new_vtable = recreate_vtable_with_space(*ctx.registers[8].x.as_ref() as _);
        let buffer_module = Box::new(InputModule::new(*ctx.registers[25].x.as_ref() as _));
        let param_module = Box::new(ParamModule::new(*ctx.registers[25].x.as_ref() as _));
        let meter_module = Box::new(MeterModule::new(*ctx.registers[25].x.as_ref() as _));
        let var_module = Box::new(VarModule::new());
        set_entry(new_vtable, Box::leak(buffer_module), INPUT_MODULE_OFFSET);
        set_entry(new_vtable, Box::leak(param_module), PARAM_MODULE_OFFSET);
        set_entry(new_vtable, Box::leak(var_module), VAR_MODULE_OFFSET);
        set_entry(new_vtable, Box::leak(meter_module), METER_MODULE_OFFSET);
        *ctx.registers[8].x.as_mut() = new_vtable as _;
    };
}

#[skyline::hook(offset = offsets::set_weapon_vtable(), inline)]
fn set_weapon_vtable_hook(ctx: &mut InlineCtx) {
    static mut WEAPON_DESTRUCTOR: usize = 0;
    static mut WEAPON_DELETER: usize = 0;

    #[skyline::hook(replace = WEAPON_DESTRUCTOR)]
    unsafe fn weapon_fighter_destructor_hook(weapon: *mut *mut *mut u64) {
        clean_hdr_object(*weapon);
        call_original!(weapon)
    }

    #[skyline::hook(replace = WEAPON_DELETER)]
    unsafe fn weapon_fighter_deleter_hook(weapon: *mut *mut *mut u64) {
        clean_hdr_object(*weapon);
        call_original!(weapon)
    }

    static DESTRUCTOR_HOOK: Once = Once::new();
    DESTRUCTOR_HOOK.call_once(|| {
        unsafe {
            WEAPON_DESTRUCTOR = *((*ctx.registers[25].x.as_ref() as usize + offsets::BATTLE_OBJECT_VTABLE_DESTRUCTOR_OFFSET) as *mut usize) as usize;
            WEAPON_DELETER = *((*ctx.registers[25].x.as_ref() as usize + offsets::BATTLE_OBJECT_VTABLE_DELETER_OFFSET) as *mut usize) as usize;
        }
        skyline::install_hooks!(weapon_fighter_destructor_hook, weapon_fighter_deleter_hook);
    });

    unsafe {
        let new_vtable = recreate_vtable_with_space(*ctx.registers[25].x.as_ref() as _);
        let buffer_module = Box::new(InputModule::new(*ctx.registers[25].x.as_ref() as _));
        let param_module = Box::new(ParamModule::new(*ctx.registers[25].x.as_ref() as _));
        let meter_module = Box::new(MeterModule::new(*ctx.registers[25].x.as_ref() as _));
        let var_module = Box::new(VarModule::new());
        set_entry(new_vtable, Box::leak(buffer_module), INPUT_MODULE_OFFSET);
        set_entry(new_vtable, Box::leak(param_module), PARAM_MODULE_OFFSET);
        set_entry(new_vtable, Box::leak(var_module), VAR_MODULE_OFFSET);
        set_entry(new_vtable, Box::leak(meter_module), METER_MODULE_OFFSET);
        *(*ctx.registers[22].x.as_ref() as *mut *mut *mut u64) = new_vtable;
    };
}

#[skyline::hook(offset = offsets::set_item_vtable(), inline)]
fn set_item_vtable_hook(ctx: &mut InlineCtx) {
    static mut ITEM_DESTRUCTOR: usize = 0;
    static mut ITEM_DELETER: usize = 0;

    #[skyline::hook(replace = ITEM_DESTRUCTOR)]
    unsafe fn item_destructor_hook(item: *mut *mut *mut u64) {
        clean_hdr_object(*item);
        call_original!(item)
    }
    
    #[skyline::hook(replace = ITEM_DELETER)]
    unsafe fn item_deleter_hook(item: *mut *mut *mut u64) {
        clean_hdr_object(*item);
        call_original!(item)
    }

    static DESTRUCTOR_HOOK: Once = Once::new();
    DESTRUCTOR_HOOK.call_once(|| {
        unsafe {
            ITEM_DESTRUCTOR = *((*ctx.registers[23].x.as_ref() as usize + offsets::BATTLE_OBJECT_VTABLE_DESTRUCTOR_OFFSET) as *mut usize) as usize;
            ITEM_DELETER = *((*ctx.registers[23].x.as_ref() as usize + offsets::BATTLE_OBJECT_VTABLE_DELETER_OFFSET) as *mut usize) as usize;
        }
        skyline::install_hooks!(item_destructor_hook, item_deleter_hook);
    });

    unsafe {
        let new_vtable = recreate_vtable_with_space(*ctx.registers[23].x.as_ref() as _);
        let buffer_module = Box::new(InputModule::new(*ctx.registers[28].x.as_ref() as _));
        let param_module = Box::new(ParamModule::new(*ctx.registers[28].x.as_ref() as _));
        let meter_module = Box::new(MeterModule::new(*ctx.registers[28].x.as_ref() as _));
        let var_module = Box::new(VarModule::new());
        set_entry(new_vtable, Box::leak(buffer_module), INPUT_MODULE_OFFSET);
        set_entry(new_vtable, Box::leak(param_module), PARAM_MODULE_OFFSET);
        set_entry(new_vtable, Box::leak(var_module), VAR_MODULE_OFFSET);
        set_entry(new_vtable, Box::leak(meter_module), METER_MODULE_OFFSET);
        *(*ctx.registers[28].x.as_ref() as *mut *mut *mut u64) = new_vtable as _;
    };
}

pub(crate) fn init() {
    skyline::install_hooks!(
        set_fighter_vtable_hook,
        set_weapon_vtable_hook
    );
    input::init();
    param::init();
    meter::init();
}

#[allow(dead_code)]
pub(crate) unsafe fn init_items() {
    skyline::install_hooks!(
        set_item_vtable_hook
    );
}