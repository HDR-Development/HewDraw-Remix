use skyline::hooks::InlineCtx;
use std::sync::atomic::{AtomicBool, Ordering};

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

#[skyline::hook(offset = 0x1d39500)]
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

#[skyline::hook(offset = 0x1d329e8, inline)]
unsafe fn add_footstool_to_gc(ctx: &skyline::hooks::InlineCtx) {
    let button = *ctx.registers[25].w.as_ref();
    if ![0x3, 0x4, 0x5, 0x8].contains(&button) {
        let input_list_vector =
            &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);

        if input_list_vector.len() < 9 {
            input_list_vector.push(utils::ext::InputKind::Parry as u8);
            input_list_vector.push(utils::ext::InputKind::JumpMini as u8);
            input_list_vector.push(utils::ext::InputKind::TiltAttack as u8);
            input_list_vector.push(utils::ext::InputKind::SmashAttack as u8);
            input_list_vector.push(utils::ext::InputKind::AppealHi as u8);
        }
    }
}

#[skyline::hook(offset = 0x1d326f8, inline)]
unsafe fn add_footstool_to_fk(ctx: &skyline::hooks::InlineCtx) {
    let button = *ctx.registers[25].w.as_ref();
    if [0x4, 0x5, 0x6, 0x9].contains(&button) {
        return;
    }
    let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);

    if input_list_vector.len() < 9 {
        input_list_vector.push(utils::ext::InputKind::Parry as u8);
        input_list_vector.push(utils::ext::InputKind::JumpMini as u8);
        input_list_vector.push(utils::ext::InputKind::TiltAttack as u8);
        input_list_vector.push(utils::ext::InputKind::SmashAttack as u8);
        input_list_vector.push(utils::ext::InputKind::AppealHi as u8);
    }
}

#[skyline::hook(offset = 0x1d3395c, inline)]
unsafe fn add_footstool_to_jc(ctx: &skyline::hooks::InlineCtx) {
    let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);

    if input_list_vector.len() < 9 {
        input_list_vector.push(utils::ext::InputKind::Parry as u8);
        input_list_vector.push(utils::ext::InputKind::JumpMini as u8);
        input_list_vector.push(utils::ext::InputKind::TiltAttack as u8);
        input_list_vector.push(utils::ext::InputKind::SmashAttack as u8);
        input_list_vector.push(utils::ext::InputKind::AppealHi as u8);
    }
}

#[skyline::hook(offset = 0x1d34e4c, inline)]
unsafe fn add_more_buttons(ctx: &mut skyline::hooks::InlineCtx) {
    let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);
    // panic!("{}", input_list_vector.len());
    *ctx.registers[25].x.as_mut() = input_list_vector.len() as u64;
}

unsafe fn calc_nnsdk_offset() -> u64 {
    let mut symbol = 0usize;
    skyline::nn::ro::LookupSymbol(&mut symbol, b"_ZN7android7IBinderD1Ev\0".as_ptr());
    (symbol - 0x240) as u64
}

static mut DUMMY_BLOCK: [u8; 0x100] = [0; 0x100];

#[skyline::hook(offset = 0x3746afc, inline)]
unsafe fn run_scene_update(_: &skyline::hooks::InlineCtx) {
    while !RUN.swap(false, Ordering::SeqCst) {
        skyline::nn::hid::GetNpadFullKeyState(DUMMY_BLOCK.as_mut_ptr() as _, &0);
    }
}

#[skyline::hook(replace = OFFSET1)]
unsafe fn set_interval_1(window: u64, _: i32) {
    call_original!(window, 0);
}

#[skyline::hook(replace = OFFSET2, inline)]
unsafe fn set_interval_2(ctx: &mut InlineCtx) {
    *ctx.registers[8].x.as_mut() = 0;
}

static mut RUN: AtomicBool = AtomicBool::new(false);

#[skyline::hook(offset = 0x380f9e4, inline)]
unsafe fn vsync_count_thread(_: &skyline::hooks::InlineCtx) {
    RUN.store(true, Ordering::SeqCst);
}

static mut OFFSET1: u64 = 0;
static mut OFFSET2: u64 = 0;

pub fn install() {
    unsafe {
        skyline::patching::Patch::in_text(0x1d34e4c).nop();
    }

    css::install();

    if !super::is_on_ryujinx() {
        unsafe {
            OFFSET1 = calc_nnsdk_offset() + 0x429d60;
            OFFSET2 = calc_nnsdk_offset() + 0x26e94;
        }

        skyline::install_hooks!(
            set_interval_1,
            set_interval_2,
            run_scene_update,
            vsync_count_thread,
        );
    }

    skyline::install_hooks!(
        get_button_label_by_operation_kind,
        add_footstool_to_gc,
        add_footstool_to_fk,
        add_footstool_to_jc,
        add_more_buttons
    );
}
