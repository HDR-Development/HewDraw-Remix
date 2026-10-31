use super::*;

#[skyline::from_offset(0x116a420)]
unsafe extern "C" fn reset_arts(fighter: *mut Fighter, param_2: u64);

// Inline hooks at 0x1167bbc and 0x1168d94 (installed from common).
// Drops the active art when Monado Beat is in effect.
#[no_mangle]
pub unsafe extern "C" fn shulk_arts_change_status_inner(ctx: &mut skyline::hooks::InlineCtx) {
    let fighter = ctx.registers[21].x() as *mut Fighter;
    let object = &mut (*fighter).battle_object;
    if VarModule::is_flag(object, vars::shulk::status::MONADO_BEAT)
    && WorkModule::is_flag(object.module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
        reset_arts(fighter, 1);
    }
}
