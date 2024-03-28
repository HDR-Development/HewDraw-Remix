use super::*;

// WEAPON_WARIO_WARIOBIKE_STATUS_KIND_SPECIAL_S_ESCAPE_START

pub unsafe extern "C" fn special_s_escape_start_exit(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let wario = utils::util::get_battle_object_from_id(owner_id);
    let wario_boma = &mut *(*wario).module_accessor;
    if weapon.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
    && wario_boma.get_num_used_jumps() < wario_boma.get_jump_count_max() {
        WorkModule::inc_int(wario_boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Exit, *WEAPON_WARIO_WARIOBIKE_STATUS_KIND_SPECIAL_S_ESCAPE_START, special_s_escape_start_exit);
}
