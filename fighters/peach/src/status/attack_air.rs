use super::*;

// FIGHTER_STATUS_KIND_ATTACK_AIR

::utils::import!(
    common::djc::{
        sub_attack_air_inherit_jump_aerial_motion_uniq_process_init,
        sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec,
        attack_air_main_status
    }
);

extern "Rust" {
    #[link_name = "attack_air_float_pre"]
    fn attack_air_float_pre(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "attack_air_float_main"]
    fn attack_air_float_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_float_pre(fighter)
}

unsafe extern "C" fn attack_air_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if prev_status_kind != VarModule::get_int(fighter.battle_object, vars::common::instance::FLOAT_STATUS_KIND) {
        return fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_init();
    }
    fighter.sub_attack_air_uniq_process_init()
}

pub unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if prev_status_kind != VarModule::get_int(fighter.battle_object, vars::common::instance::FLOAT_STATUS_KIND) {
        return common::djc::attack_air_main_status(fighter);
    }
    attack_air_float_main(fighter)
}

unsafe extern "C" fn attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::djc::sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_exec);
}