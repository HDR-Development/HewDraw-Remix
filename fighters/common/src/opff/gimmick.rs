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
use super::physics::groups;

#[no_mangle]
pub unsafe extern "Rust" fn gimmick_flash(boma: &mut BattleObjectModuleAccessor) {
	if !app::sv_information::is_ready_go() {
		return
	}
    let fighter = get_fighter_common_from_accessor(boma);
    let lr = PostureModule::lr(fighter.module_accessor);
    let group = ParamModule::get_int(fighter.battle_object, ParamType::Shared, "ecb_group_shift");
    let mut flash_y_offset: f32 = match group {
        groups::SMALL   => 10.0,
        groups::MEDIUM  => 13.0,
        groups::LARGE   => 16.0,
        groups::XLARGE  => 19.0,
        groups::XXLARGE => 22.0,
        _ => panic!("malformed parammodule file! unknown group number for ecb shift: {}", group.to_string())
    };

    app::FighterUtil::flash_eye_info(fighter.module_accessor);

    if WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, flash_y_offset, 2, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    else {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, flash_y_offset, 2, 0, 0, 0, 0.7, true);
    }
    LAST_EFFECT_SET_RATE(fighter, 0.5);
    LAST_EFFECT_SET_COLOR(fighter, 0.831, 0.686, 0.216);	
}