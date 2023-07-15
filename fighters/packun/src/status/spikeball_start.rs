use super::*;
use globals::*;

#[status_script(agent = "packun_spikeball", status = WEAPON_PACKUN_SPIKEBALL_STATUS_KIND_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn spikeball_start_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_PACKUN {
		let packun = utils::util::get_battle_object_from_id(owner_id);
		VarModule::set_int(packun, vars::packun::instance::PTOOIE_STANCE, VarModule::get_int(packun, vars::packun::instance::CURRENT_STANCE));
	}
    original!(agent)
}

pub fn install() {
    install_status_scripts!(
        spikeball_start_main
    );
}