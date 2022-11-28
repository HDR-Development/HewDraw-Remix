use super::*;

#[status_script(agent = "zelda_dein", status = WEAPON_ZELDA_DEIN_STATUS_KIND_MOVE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
		let zelda = utils::util::get_battle_object_from_id(owner_id);
		VarModule::set_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID, weapon.battle_object_id as i32);
	}
    original!(weapon)
}

pub fn install() {
    install_status_scripts!(
        move_main
    );
}