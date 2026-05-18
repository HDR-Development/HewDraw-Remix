// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe extern "C" fn spikeball_frame(weapon: &mut L2CFighterBase) {
    let owner_module_accessor = weapon.get_owner_boma();
    if [*FIGHTER_KIND_PACKUN, *FIGHTER_KIND_KIRBY].contains(&owner_module_accessor.kind()) {
        if weapon.is_status(*WEAPON_PACKUN_SPIKEBALL_STATUS_KIND_HOP)
        && VarModule::is_flag(weapon.battle_object, vars::packun_spikeball::instance::ENABLE_EXPLODE)
        && weapon.is_prev_situation(*SITUATION_KIND_AIR)
        && weapon.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(weapon, Hash40::new("packun_poison_mouth"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, false);
        }
        if weapon.is_status(*WEAPON_PACKUN_SPIKEBALL_STATUS_KIND_WAIT) || weapon.is_status(*WEAPON_PACKUN_SPIKEBALL_STATUS_KIND_HOP) {
            if weapon.status_frame() >= 60 && VarModule::is_flag(weapon.battle_object, vars::packun_spikeball::instance::ENABLE_EXPLODE) {
                if MotionModule::motion_kind(weapon.module_accessor) != hash40("explode") {
                    MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, spikeball_frame);
}