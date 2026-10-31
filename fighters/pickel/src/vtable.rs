use super::*;

// Inline hook at 0xf13d5c (installed from common).
// lets the "stuff" article generate in new statuses
#[no_mangle]
pub unsafe extern "C" fn pickel_stuff_hook_inner(ctx: &mut skyline::hooks::InlineCtx) {
    let new_shield_statuses = &[
        0x1B, // GUARD_ON
        0x1C // GUARD
    ];
    let status = ctx.registers[0].x();
    if new_shield_statuses.contains(&status) {
        ctx.registers[0].set_x(0x1E);
    }
}
