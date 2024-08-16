use super::*;

pub unsafe extern "C" fn attack_s3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AttackS3();
    ComboModule::reset(fighter.module_accessor);
    VarModule::off_flag(fighter.battle_object, vars::trail::instance::ATTACK_12_INTO_S3);
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S3, attack_s3_end);
}