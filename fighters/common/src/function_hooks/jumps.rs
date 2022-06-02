use super::*;
use utils::ext::*;
use std::arch::asm;


#[skyline::hook(offset = 0x6d2174, inline)]
unsafe fn fullhop_initial_y_speed_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(*ctx.registers[8].x.as_ref());
    let work_module = *ctx.registers[0].x.as_ref();
    let jump_y = callable(work_module, smash::hash40("jump_y"), 0);
    let gravity = callable(work_module, smash::hash40("air_accel_y"), 0);
    let initital_jump_vel = (jump_y * gravity * 2.0).sqrt() + (0.5 * gravity);
    asm!("fmov s0, w8", in("w8") initital_jump_vel)
}

pub fn install() {
    unsafe {
        // stubs vanilla fullhop initial y velocity calculations
        skyline::patching::nop_data(0x6d2174);
    }
    skyline::install_hooks!(fullhop_initial_y_speed_hook);
}