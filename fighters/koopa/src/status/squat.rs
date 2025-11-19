use super::*;

const KNOCKBACK_ARMOR: f32 = 30.0;

// FIGHTER_STATUS_KIND_SQUAT
unsafe extern "C" fn squat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, KNOCKBACK_ARMOR);
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SQUAT)(fighter);
}
unsafe extern "C" fn squat_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SQUAT)(fighter);
}

// FIGHTER_STATUS_KIND_SQUAT_B
unsafe extern "C" fn squat_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, KNOCKBACK_ARMOR);
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SQUAT_B)(fighter);
}
unsafe extern "C" fn squat_b_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SQUAT_B)(fighter);
}

// FIGHTER_STATUS_KIND_SQUAT_F
unsafe extern "C" fn squat_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, KNOCKBACK_ARMOR);
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SQUAT_F)(fighter);
}
unsafe extern "C" fn squat_f_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SQUAT_F)(fighter);
}

// FIGHTER_STATUS_KIND_SQUAT_RV
unsafe extern "C" fn squat_rv_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, KNOCKBACK_ARMOR);
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SQUAT_RV)(fighter);
}
unsafe extern "C" fn squat_rv_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SQUAT_RV)(fighter);
}

// FIGHTER_STATUS_KIND_SQUAT_WAIT
unsafe extern "C" fn squat_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, KNOCKBACK_ARMOR);
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SQUAT_WAIT)(fighter);
}
unsafe extern "C" fn squat_wait_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SQUAT_WAIT)(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SQUAT, squat_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SQUAT, squat_end);

    agent.status(Main, *FIGHTER_STATUS_KIND_SQUAT_B, squat_b_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SQUAT_B, squat_b_end);

    agent.status(Main, *FIGHTER_STATUS_KIND_SQUAT_F, squat_f_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SQUAT_F, squat_f_end);

    agent.status(Main, *FIGHTER_STATUS_KIND_SQUAT_RV, squat_rv_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SQUAT_RV, squat_rv_end);

    agent.status(Main, *FIGHTER_STATUS_KIND_SQUAT_WAIT, squat_wait_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SQUAT_WAIT, squat_wait_end);
}