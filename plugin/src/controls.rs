use std::sync::atomic::{Ordering, AtomicBool};
use skyline::hooks::InlineCtx;

#[repr(C)]
pub struct HashedString {
    pub hash: smash::phx::Hash40,
    pub contents: [u8; 0x100]
}

#[repr(C)]
pub struct CppVector<T> {
    start: *mut T,
    end: *mut T,
    eos: *mut T
}

impl<T> CppVector<T> {
    pub fn len(&self) -> usize {
        unsafe {
            self.end.offset_from(self.start) as usize
        }
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
unsafe fn get_button_label_by_operation_kind(hashed_string: &mut HashedString, operation: u8, arg: bool) {
    if operation == utils::ext::InputKind::JumpMini as u8 {
        for (index, byte) in "mnu_opt_btn_key_short_hop\0".as_bytes().iter().enumerate() {
            hashed_string.contents[index] = *byte;
        }
        hashed_string.hash = smash::phx::Hash40::new("mnu_opt_btn_key_short_hop");
    } else if operation == utils::ext::InputKind::TiltAttack as u8 {
        for (index, byte) in "mnu_opt_btn_key_tilt_attack\0".as_bytes().iter().enumerate() {
            hashed_string.contents[index] = *byte;
        }
        hashed_string.hash = smash::phx::Hash40::new("mnu_opt_btn_key_tilt_attack");
    } 
    else if operation == utils::ext::InputKind::Parry as u8 {
        for (index, byte) in "mnu_opt_btn_key_parry\0".as_bytes().iter().enumerate() {
            hashed_string.contents[index] = *byte;
        }
        hashed_string.hash = smash::phx::Hash40::new("mnu_opt_btn_key_parry");
    } else {
        return call_original!(hashed_string, operation, arg)
    }
}

#[skyline::hook(offset = 0x1d329e8, inline)]
unsafe fn add_footstool_to_gc(ctx: &skyline::hooks::InlineCtx) {
    let button = *ctx.registers[25].w.as_ref();
    if ![0x3, 0x4, 0x5, 0x8].contains(&button) {
        let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);

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

/*
* The less delay mod does 2 things: double the game engine speed, and then only render new frames on the vsync timer.
* Without the 2nd part, the game will try to render 120 fps (which is too much computational power for the switch so only around 80-90 frames will actually be rendered per second). Vsync can actually not be turned off on the nintendo switch (that we know of), maybe blujay's mod disabled some in game vsync.
* With the 2nd part, the game still renders 60 frames per second despite the doubled game engine speed.
* Doubling the game engine speed is the source for the decreased delay -> this same concept have been verified for a long time in the yuzu smash community, where users double the game engine speed and half the game speed resulting in the game running at 120fps but the game still running at regular speed. On yuzu it cuts the delay in half, but unfortunately the switch does not have strong enough hardware to run smash at 120fps, so this mod will be the best we can do for the time being.
* 
* Optimization to existing code:
* The original logic is when it's time to render a new frame, it stalls until the next vsync timer, then renders the frame
* This results in stalling for up to a frame until the next vsync timer cycle, then 1 more vsync cycle for the frame to actually display
* The optimization is to render the frame right after the current vsync cycle so that it is queued in time to be displayed by the next
* vsync cycle. This saves an average of half a frame, since it eliminates the original code's stalling, which is on average half a frame.
* Note in worst cases, it doesn't reduce any input delay compared to the original, in best case, it saves almost a frame,
* depending on when the call to render the frame is made. For example if the frame render call is near the middle or late into the vsync
* cycle, it is not queued/rendered in time to be picked up by the immediate next vsync cycle, so in that case no delay is saved compared to the original.
* Because rendering a frame takes most of the duration of a vsync cycle.  (see note above, the switch is capable of just rendering 80-90 fps for smash, though that would be increased with overclocking but still not enough for 120fps)
* Also removed consideration of ordering/synchronization of the atomic variable, this may not improve performance by a noticeable margin at all, but current code does the trick so leaving as is.

* I was able to observe just 7 frames of total delay with this mod and latency mod set to 2 frames (best case). The original would have 8 frames best case for latency slider 2. Worst case remains the same as the original.
* One thing to note, just simply being online adds a frame of delay. The latency slider value is the additional frames on top of that. 
* So for example without the vsync mod, if latency mod has value 3, the number of additional frames is actually 4. That's why in best case scenarios for Auto, we sometimes see 3, it doesn't mean 3 frames of additional delay, but actually 4 -> which is in par with base netcode.

* The delay is consistent during the course of a game but can vary game to game. For example in one game, there's 1 frame less delay comapred to the original mod, but in the next game it will be the same delay as the original mod."
*/


// called when the game wants to render a frame and queue it to be displayed. When vsync happens, it displays the latest queued frame
// updated to render the frame right after the current vsync cycle
#[skyline::hook(offset = 0x3746afc)]
unsafe fn run_scene_update(arg: u64) {
    //check if we have rendered a frame on the current vsync cycle
    if *RUN.get_mut() { 
        // new vsync cycle, update the flag and render frame
        RUN = AtomicBool::new(false);
        call_original!(arg);
    }else{ //a frame is already rendered in the current vsync cycle, do not render a new frame
        skyline::nn::hid::GetNpadFullKeyState(DUMMY_BLOCK.as_mut_ptr() as _, &0);
    }
}

// appears to disable some kind of ingame vsync, the switch vsync itself cannot be disabled, will always be 60hz
#[skyline::hook(replace = OFFSET1)]
unsafe fn set_interval_1(window: u64, _: i32) {
    call_original!(window, 0);
}

// doubles the game engine speed
#[skyline::hook(replace = OFFSET2, inline)]
unsafe fn set_interval_2(ctx: &mut InlineCtx) {
    *ctx.registers[8].x.as_mut() = 0;
}

static mut RUN: AtomicBool = AtomicBool::new(false);

// called every time vsync happens (60hz)
#[skyline::hook(offset = 0x380f9e4, inline)]
unsafe fn vsync_count_thread(_: &skyline::hooks::InlineCtx) {
    RUN = AtomicBool::new(true);
}

static mut OFFSET1: u64 = 0;
static mut OFFSET2: u64 = 0;


pub fn install() {
    unsafe {
        skyline::patching::Patch::in_text(0x1d34e4c).nop();
    }

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