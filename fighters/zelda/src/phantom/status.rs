use super::*;

unsafe extern "C" fn build_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let ret = smashline::original_status(Main, weapon, *WEAPON_ZELDA_PHANTOM_STATUS_KIND_BUILD)(weapon);
    GroundModule::set_passable_check(weapon.module_accessor, false);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let zelda = utils::util::get_battle_object_from_id(owner_id);
    let zelda_boma: &mut BattleObjectModuleAccessor = &mut *(*zelda).module_accessor;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
        VarModule::set_int(zelda, vars::zelda::instance::SPECIAL_LW_PHANTOM_OBJECT_ID, weapon.battle_object_id as i32);
        if VarModule::is_flag(zelda, vars::zelda::instance::SPECIAL_LW_FORWARD_PHANTOM) {
            let pos = *PostureModule::pos(weapon.module_accessor);
            let zelda_pos = *PostureModule::pos(zelda_boma);      
            let pos = Vector3f { x: zelda_pos.x + 18.0*(PostureModule::lr(weapon.module_accessor)) , y: pos.y, z: pos.z }; //18 in front of her, base spawn point? is -16.2
            PostureModule::set_pos(weapon.module_accessor, &pos);
            PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
            VarModule::off_flag(zelda, vars::zelda::instance::SPECIAL_LW_FORWARD_PHANTOM);
        }
    }
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_ZELDA_PHANTOM_STATUS_KIND_BUILD, build_main);
}
