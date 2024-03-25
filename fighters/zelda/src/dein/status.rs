use super::*;

unsafe extern "C" fn move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
		let zelda = utils::util::get_battle_object_from_id(owner_id);
		VarModule::set_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID, weapon.battle_object_id as i32);
	}
    smashline::original_status(Main, weapon, *WEAPON_ZELDA_DEIN_STATUS_KIND_MOVE)(weapon)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_ZELDA_DEIN_STATUS_KIND_MOVE, move_main);
}
