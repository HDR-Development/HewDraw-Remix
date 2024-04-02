use super::*;
use globals::*;
// status script import

// WEAPON_MIISWORDSMAN_CHAKRAM_STATUS_KIND_HOP

unsafe extern "C" fn pre_chakram_hop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let lua_state = weapon.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    VarModule::off_flag(owner_module_accessor.object(), vars::miiswordsman::instance::CHAKRAM_STICK_ATTACK);
    smashline::original_status(Pre, weapon, *WEAPON_MIISWORDSMAN_CHAKRAM_STATUS_KIND_HOP)(weapon)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *WEAPON_MIISWORDSMAN_CHAKRAM_STATUS_KIND_HOP, pre_chakram_hop);
}