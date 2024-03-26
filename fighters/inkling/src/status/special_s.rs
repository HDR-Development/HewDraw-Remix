use super::*;
use globals::*;

// FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK

pub unsafe extern "C" fn special_s_walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Once per airtime
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S);
    }
    smashline::original_status(Main, fighter, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK)(fighter)
}

// FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN

pub unsafe extern "C" fn special_s_run_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Once per airtime
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S);
    }
    smashline::original_status(Main, fighter, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN)(fighter)
}

// special_s_jump_end_init

pub unsafe extern "C" fn special_s_jump_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Burn double jump when jumping out of Splat Roller
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
    && fighter.get_num_used_jumps() < fighter.get_jump_count_max() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK, special_s_walk_main);
    agent.status(Main, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN, special_s_run_main);
    agent.status(Init, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_JUMP_END, special_s_jump_end_init);
}
