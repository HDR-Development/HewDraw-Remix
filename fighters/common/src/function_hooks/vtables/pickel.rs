use super::*;

// lets the "stuff" article generate in new statuses
#[skyline::hook(offset = 0xf13d5c, inline)]
unsafe fn pickel_stuff_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let new_shield_statuses = &[
        0x1B, // GUARD_ON
        0x1C // GUARD
    ];
    let status = *ctx.registers[0].x.as_ref();
    if new_shield_statuses.contains(&status) {
        *ctx.registers[0].x.as_mut() = 0x1E;
    } 
}

pub fn install() {
    skyline::install_hooks!(
        pickel_stuff_hook
    );
}