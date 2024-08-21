use super::*;

// FIGHTER_STATUS_KIND_WAIT
unsafe extern "C" fn wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if status == statuses::bayonetta::SPECIAL_S_KICK {
        fighter.global_table[PREV_STATUS_KIND].assign(&L2CValue::I32(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_HOLD_END));
    } else if status == statuses::bayonetta::SPECIAL_N_CANCEL {
        fighter.global_table[PREV_STATUS_KIND].assign(&L2CValue::I32(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_END));
    } //fixes glitched idle without having to rewrite EVERYTHING
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_WAIT)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_WAIT, wait_main);
}