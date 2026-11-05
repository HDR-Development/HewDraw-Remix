use super::*;

// Inline hook at 0x34cec44 (installed from common).
#[no_mangle]
pub unsafe extern "C" fn ptrainer_swap_backwards_hook_inner(ctx: &mut skyline::hooks::InlineCtx) {
    let object = ctx.registers[20].x() as *mut BattleObject;
    if VarModule::is_flag(object, vars::ptrainer::instance::SPECIAL_LW_BACKWARDS_SWITCH) {
        let new = match ctx.registers[8].x() {
            0 => 1,
            1 => 2,
            2 => 0,
            _ => unreachable!()
        };

        ctx.registers[8].set_x(new);
        VarModule::off_flag(object, vars::ptrainer::instance::SPECIAL_LW_BACKWARDS_SWITCH)
    }
}
