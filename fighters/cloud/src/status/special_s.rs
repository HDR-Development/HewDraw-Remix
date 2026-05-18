use super::*;

unsafe extern "C" fn special_s1_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // disable stall if exiting SSpecial in the air
    let next_status = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
    && ![*FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3].contains(&next_status){
        VarModule::on_flag(fighter.battle_object, vars::cloud::instance::SPECIAL_S_DISABLE_STALL);
    }
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
}

unsafe extern "C" fn special_s2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // disable stall if exiting SSpecial in the air
    let next_status = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
    && ![*FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3].contains(&next_status){
        VarModule::on_flag(fighter.battle_object, vars::cloud::instance::SPECIAL_S_DISABLE_STALL);
    }
    smashline::original_status(End, fighter, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2)(fighter)
}

unsafe extern "C" fn special_s3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // disable stall if exiting SSpecial in the air
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        VarModule::on_flag(fighter.battle_object, vars::cloud::instance::SPECIAL_S_DISABLE_STALL);
    }
    smashline::original_status(End, fighter, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3)(fighter)
}

unsafe extern "C" fn special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Exec, fighter, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3)(fighter);

    // no gravity while using this stall flag
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
    && VarModule::is_flag(fighter.battle_object, vars::cloud::status::SPECIAL_S_STALL)
    && !VarModule::is_flag(fighter.battle_object, vars::cloud::instance::SPECIAL_S_DISABLE_STALL) {
        fighter.on_flag(*FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    
    ret
}

unsafe extern "C" fn special_s_check_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_check_attack);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s1_end);

    agent.status(CheckAttack, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, special_s_check_attack);
    agent.status(Exec, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, special_s_exec);
    agent.status(End, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, special_s2_end);

    agent.status(End, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3, special_s3_end);
}