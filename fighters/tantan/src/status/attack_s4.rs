use super::*;

// FIGHTER_STATUS_KIND_ATTACK_S4_START

unsafe extern "C" fn attack_s4_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackS4Start();
}

unsafe extern "C" fn attack_s4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackS4Start();
}

// FIGHTER_STATUS_KIND_ATTACK_S4_HOLD

unsafe extern "C" fn attack_s4_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    return fighter.status_pre_AttackS4Hold();
}

unsafe extern "C" fn attack_s4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackS4Hold as *const () as _))
}

pub unsafe extern "C" fn attack_s4_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bigFrame =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME);
    if 0 < bigFrame && bigFrame < 2 {
        WorkModule::set_int(fighter.module_accessor, 2,*FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME);
    }
    return 0.into()
}

// FIGHTER_STATUS_KIND_ATTACK_S4

pub unsafe extern "C" fn attack_s4_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bigFrame =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME);
    if 0 < bigFrame && bigFrame < 2 {
        WorkModule::set_int(fighter.module_accessor, 2,*FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME);
    }
    return 0.into()
}

unsafe extern "C" fn attack_s4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bigFrame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME);
    if fighter.motion_frame() > 16.0 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 0 {
        if bigFrame > 0 {
            let maxBigFrame = WorkModule::get_param_int(fighter.module_accessor,hash40("param_private"),hash40("arm_l_big_frame"));
            let newBigFrame = (bigFrame-(maxBigFrame/2)).max(1);
            WorkModule::set_int(fighter.module_accessor, newBigFrame, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME);
        }
    }
    return fighter.status_end_AttackS4();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S4_START, attack_s4_start_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4_START, attack_s4_start_main);

    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attack_s4_hold_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attack_s4_hold_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attack_s4_hold_exec);

    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_S4, attack_s4_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, attack_s4_end);
}
