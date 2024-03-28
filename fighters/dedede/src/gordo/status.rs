use super::*;
use globals::*;

unsafe extern "C" fn dead_end(weapon: &mut L2CWeaponCommon) -> L2CValue{
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_DEDEDE{
        let dedede = utils::util::get_battle_object_from_id(owner_id);
        VarModule::set_flag(dedede, vars::dedede::instance::CAN_WADDLE_DASH_FLAG, true); 
        VarModule::set_int(dedede, vars::dedede::instance::RECATCH_COUNTER, 0); 
        VarModule::set_flag(dedede, vars::dedede::instance::IS_DASH_GORDO, false);
    }
    return smashline::original_status(End, weapon, *WEAPON_DEDEDE_GORDO_STATUS_KIND_DEAD)(weapon)
}

pub fn install(agent: &mut Agent){
    agent.status(End, *WEAPON_DEDEDE_GORDO_STATUS_KIND_DEAD, dead_end);
}