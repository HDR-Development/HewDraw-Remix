use super::*;

//Ftilt//

unsafe extern "C" fn tantan_attack_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackS3();
}

unsafe extern "C" fn tantan_attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackS3_Main();
}

unsafe extern "C" fn tantan_attack_s3_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor){
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    else{
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S3, tantan_attack_s3_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_S3, tantan_attack_s3_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_S3, tantan_attack_s3_exec);
}
