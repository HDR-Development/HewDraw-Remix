use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::{
	self,
	BattleObjectModuleAccessor,
};
use smash::phx::{Vector2f, Vector3f, Vector4f, Hash40};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;
use smash_script::macros::{EFFECT_FOLLOW, EFFECT_FOLLOW_FLIP, LAST_EFFECT_SET_RATE, LAST_EFFECT_SET_COLOR};
use crate::get_fighter_common_from_accessor;

#[no_mangle]
pub unsafe extern "Rust" fn gimmick_flash(boma: &mut BattleObjectModuleAccessor) {
	if !app::sv_information::is_ready_go() {
		return
	}
    let fighter = get_fighter_common_from_accessor(boma);
    let lr = PostureModule::lr(fighter.module_accessor);
    let flash_y_offset = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);

    app::FighterUtil::flash_eye_info(fighter.module_accessor);

    if WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, flash_y_offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
    else {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, flash_y_offset, 2, 0, 0, 0, 1.0, true);
    }
    LAST_EFFECT_SET_COLOR(fighter, 0.831, 0.686, 0.216);	
}