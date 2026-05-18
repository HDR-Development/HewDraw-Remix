use skyline::hooks::InlineCtx;
// use ssbusync::*;

mod css;
mod submenu;
mod swkbd;

#[repr(C)]
pub struct HashedString {
    pub hash: smash::phx::Hash40,
    pub contents: [u8; 0x100],
}

#[repr(C)]
pub struct CppVector<T> {
    start: *mut T,
    end: *mut T,
    eos: *mut T,
}

impl<T> CppVector<T> {
    pub fn len(&self) -> usize {
        unsafe { self.end.offset_from(self.start) as usize }
    }
}

impl<T: Copy> CppVector<T> {
    pub fn push(&mut self, value: T) {
        unsafe {
            let length = self.end.offset_from(self.start) as usize;
            let cap = self.eos.offset_from(self.start) as usize;
            if length == cap {
                let new_ptr = skyline::libc::malloc(std::mem::size_of::<T>() * cap * 2);
                skyline::libc::memcpy(new_ptr, self.start as _, std::mem::size_of::<T>() * length);
                let old = self.start;
                self.start = new_ptr as _;
                self.end = self.start.add(length as usize);
                self.eos = self.start.add((cap * 2) as usize);

                skyline::libc::free(old as _);
            }

            *self.end = value;
            self.end = self.end.add(1);
        }
    }
}

#[skyline::hook(offset = 0x1D3A000)]
unsafe fn get_button_label_by_operation_kind(
    hashed_string: &mut HashedString,
    operation: u8,
    arg: bool,
) {
    if operation == utils::ext::InputKind::JumpMini as u8 {
        for (index, byte) in "mnu_opt_btn_key_short_hop\0".as_bytes().iter().enumerate() {
            hashed_string.contents[index] = *byte;
        }
        hashed_string.hash = smash::phx::Hash40::new("mnu_opt_btn_key_short_hop");
    } else if operation == utils::ext::InputKind::TiltAttack as u8 {
        for (index, byte) in "mnu_opt_btn_key_tilt_attack\0"
            .as_bytes()
            .iter()
            .enumerate()
        {
            hashed_string.contents[index] = *byte;
        }
        hashed_string.hash = smash::phx::Hash40::new("mnu_opt_btn_key_tilt_attack");
    } else if operation == utils::ext::InputKind::Parry as u8 {
        for (index, byte) in "mnu_opt_btn_key_parry\0".as_bytes().iter().enumerate() {
            hashed_string.contents[index] = *byte;
        }
        hashed_string.hash = smash::phx::Hash40::new("mnu_opt_btn_key_parry");
    } else {
        return call_original!(hashed_string, operation, arg);
    }
}

#[skyline::hook(offset = 0x1d334e8, inline)]
unsafe fn add_footstool_to_gc(ctx: &skyline::hooks::InlineCtx) {
    let button = ctx.registers[25].w();
    if ![0x3, 0x4, 0x5, 0x8].contains(&button) {
        let input_list_vector =
            &mut *((ctx.registers[24].x() + 0x148) as *mut CppVector<u8>);

        if input_list_vector.len() < 9 {
            input_list_vector.push(utils::ext::InputKind::Parry as u8);
            input_list_vector.push(utils::ext::InputKind::JumpMini as u8);
            input_list_vector.push(utils::ext::InputKind::TiltAttack as u8);
            input_list_vector.push(utils::ext::InputKind::SmashAttack as u8);
            input_list_vector.push(utils::ext::InputKind::AppealHi as u8);
        }
    }
}

#[skyline::hook(offset = 0x1D331F8, inline)]
unsafe fn add_footstool_to_fk(ctx: &skyline::hooks::InlineCtx) {
    let button = ctx.registers[25].w();
    if [0x4, 0x5, 0x6, 0x9].contains(&button) {
        return;
    }
    let input_list_vector = &mut *((ctx.registers[24].x() + 0x148) as *mut CppVector<u8>);

    if input_list_vector.len() < 9 {
        input_list_vector.push(utils::ext::InputKind::Parry as u8);
        input_list_vector.push(utils::ext::InputKind::JumpMini as u8);
        input_list_vector.push(utils::ext::InputKind::TiltAttack as u8);
        input_list_vector.push(utils::ext::InputKind::SmashAttack as u8);
        input_list_vector.push(utils::ext::InputKind::AppealHi as u8);
    }
}

#[skyline::hook(offset = 0x1D33CD8, inline)]
unsafe fn add_footstool_to_jc(ctx: &skyline::hooks::InlineCtx) {
    let input_list_vector = &mut *((ctx.registers[24].x() + 0x148) as *mut CppVector<u8>);

    if input_list_vector.len() < 9 {
        input_list_vector.push(utils::ext::InputKind::Parry as u8);
        input_list_vector.push(utils::ext::InputKind::JumpMini as u8);
        input_list_vector.push(utils::ext::InputKind::TiltAttack as u8);
        input_list_vector.push(utils::ext::InputKind::SmashAttack as u8);
        input_list_vector.push(utils::ext::InputKind::AppealHi as u8);
    }
}

#[skyline::hook(offset = 0x1D3594C, inline)]
unsafe fn add_more_buttons(ctx: &mut skyline::hooks::InlineCtx) {
    let input_list_vector = &mut *((ctx.registers[24].x() + 0x148) as *mut CppVector<u8>);
    // panic!("{}", input_list_vector.len());
    ctx.registers[25].set_x(input_list_vector.len() as u64);
}

// The function this hook replaces gets the number of missing required buttons
// (or some other type of error code)
// Returning 0 bypasses the check entirely
#[skyline::hook(offset = 0x1d2c8d0)] 
unsafe fn get_missing_button_count_hook(
    _context: u64,
    mode: i32,
    _p3: u64,
    _p4: u64,
    _p5: u64,
    _p6: u64,
    _p7: u64,
    _p8: u64
) -> i32 {
    0
}

pub fn install() {
    unsafe {
        skyline::patching::Patch::in_text(0x1D3594C).nop();
        css::install();
    }

    skyline::install_hooks!(
        get_button_label_by_operation_kind,
        add_footstool_to_gc,
        add_footstool_to_fk,
        add_footstool_to_jc,
        add_more_buttons,
        get_missing_button_count_hook,
    );
}
