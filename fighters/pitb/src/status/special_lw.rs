use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_LW

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);

    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    VarModule::set_int(fighter.battle_object, vars::pitb::instance::SPECIAL_LW_SITUATION_START, situation_kind);
    VarModule::off_flag(fighter.battle_object, vars::pitb::instance::SPECIAL_LW_ENABLE_CANCEL);
    if !VarModule::is_flag(fighter.battle_object, vars::pitb::instance::SPECIAL_LW_DISABLE_STALL) {
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
    }
    VarModule::on_flag(fighter.battle_object, vars::pitb::instance::SPECIAL_LW_DISABLE_STALL);

    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
}