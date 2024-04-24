// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

use vars::rosetta::instance::*;

unsafe extern "C" fn tico_frame(weapon: &mut L2CWeaponCommon) {
	let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	let rosetta = utils::util::get_battle_object_from_id(owner_id);
	let rosetta_boma = &mut *(*rosetta).module_accessor;

	if StatusModule::is_changing(weapon.module_accessor) || weapon.is_status(*WEAPON_ROSETTA_TICO_STATUS_KIND_REBIRTH) {
		VarModule::off_flag(rosetta, IS_TICO_UNAVAILABLE);
	}

	if weapon.is_status_one_of(&[
		*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD,
		*WEAPON_ROSETTA_TICO_STATUS_KIND_DOWN,
		*WEAPON_ROSETTA_TICO_STATUS_KIND_STANDBY,
		*WEAPON_ROSETTA_TICO_STATUS_KIND_NONE,
		*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE,
		*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_AIR,
		*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FALL,
		*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FLY,
		*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
		*WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FLY_REFLECT_U]) {
		VarModule::on_flag(rosetta, IS_TICO_UNAVAILABLE);
	}
}

pub fn install(agent: &mut Agent) {
	agent.on_line(Main, tico_frame);
}
