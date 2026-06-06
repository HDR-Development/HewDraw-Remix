use super::*;

extern "Rust" {
    #[link_name = "attack_air_float_pre"]
    fn attack_air_float_pre(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "attack_air_float_main"]
    fn attack_air_float_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

// FIGHTER_STATUS_KIND_ATTACK_AIR

unsafe extern "C" fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_float_pre(fighter)
}

unsafe extern "C" fn attack_air_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if prev_status_kind != VarModule::get_int(fighter.battle_object, vars::common::instance::FLOAT_STATUS_KIND) {
        VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
        fighter.off_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
        return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter);
    }
    fighter.sub_attack_air_uniq_process_init()
}

unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_float_main(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
}
