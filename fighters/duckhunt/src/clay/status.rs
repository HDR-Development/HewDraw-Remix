use super::*;
use globals::*;

// WEAPON_DUCKHUNT_CLAY_STATUS_KIND_FLY

pub unsafe extern "C" fn fly_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let duckhunt = utils::util::get_battle_object_from_id(owner_id);
    if VarModule::is_flag(duckhunt, vars::duckhunt::status::CLAY_SMASH_INPUT) {
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_DUCKHUNT_CLAY_INSTANCE_WORK_ID_FLAG_BY_SMASH);
    }
    smashline::original_status(Init, weapon, *WEAPON_DUCKHUNT_CLAY_STATUS_KIND_FLY)(weapon)
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *WEAPON_DUCKHUNT_CLAY_STATUS_KIND_FLY, fly_init);
}