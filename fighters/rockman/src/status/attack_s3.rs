use super::*;

unsafe extern "C" fn attack_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_AttackS3()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S3, attack_s3_pre);
}
