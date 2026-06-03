// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe extern "C" fn gigafire_frame(weapon: &mut L2CFighterBase) {
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR | *COLLISION_KIND_MASK_PARRY) 
    && weapon.is_status_one_of(&[*WEAPON_REFLET_GIGAFIRE_STATUS_KIND_BURN, *WEAPON_REFLET_GIGAFIRE_STATUS_KIND_RISE]) {
        AttackModule::clear_inflict_kind_status(weapon.module_accessor);
        weapon.change_status(WEAPON_REFLET_GIGAFIRE_STATUS_KIND_RISE.into(), true.into());
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, gigafire_frame);
}