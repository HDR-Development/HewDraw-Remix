use super::*;

pub unsafe extern "C" fn attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_uniq_process_exec();
    if fighter.is_motion(Hash40::new("attack_air_lw")) {
        if fighter.is_flag(*FIGHTER_SIMON_STATUS_ATTACK_FLAG_HIT) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                VarModule::on_flag(fighter.battle_object, vars::richter::instance::ATTACK_AIR_LW_REBOUND);
            }
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall_leaning_c"), 0.0, 1.0, false, 0.0, false, false);
            fighter.set_int64(hash40("fall_leaning_c") as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
            fighter.on_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_exec);
}