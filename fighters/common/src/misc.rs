use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::app::*;
use smash::lib::lua_const::*;
use smash::hash40;
use utils::ext::*;
use utils::consts::*;
use utils::*;

use globals::*;

pub fn install() {
    smashline::install_agent_resets!(fighter_reset);
    //skyline::install_hooks!(
    //    set_hit_team_hook,
    //);
}

/*#[skyline::hook(replace=TeamModule::set_hit_team)]
unsafe fn set_hit_team_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    if (boma.kind() == *ITEM_KIND_BARREL) {
        return;
    }
}*/

#[smashline::fighter_reset]
pub fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let ratio = (WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x_max"), 0) / WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0));
        VarModule::set_float(fighter.battle_object, vars::common::instance::JUMP_SPEED_RATIO, ratio);
        if fighter.kind() == *FIGHTER_KIND_KEN || fighter.kind() == *FIGHTER_KIND_RYU || fighter.kind() == *FIGHTER_KIND_DOLLY {
            MeterModule::reset(fighter.battle_object);
        }
    }

}