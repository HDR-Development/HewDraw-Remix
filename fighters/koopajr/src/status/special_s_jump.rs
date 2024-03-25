use super::*;
use globals::*;
// status script import

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP

pub unsafe extern "C" fn special_s_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Burn double jump when jumping out of Clown Kart Dash
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
    && fighter.get_num_used_jumps() < fighter.get_jump_count_max() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_init);
}