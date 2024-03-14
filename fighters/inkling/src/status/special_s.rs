use super::*;
use globals::*;

// FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK

pub unsafe extern "C" fn special_s_walk(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Once per airtime
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S);
    }
    smashline::original_status(Main, fighter, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK)(fighter)
}

// FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN

pub unsafe extern "C" fn special_s_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Once per airtime
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S);
    }
    smashline::original_status(Main, fighter, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN)(fighter)
}

// FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_JUMP_END

pub unsafe extern "C" fn special_s_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Burn double jump when jumping out of Splat Roller
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
    && fighter.get_num_used_jumps() < fighter.get_jump_count_max() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    0.into()
}

pub fn install() {
    smashline::Agent::new("inkling")
        .status(
            Main,
            *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK,
            special_s_walk,
        )
        .status(
            Main,
            *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN,
            special_s_run,
        )
        .status(
            Init,
            *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_JUMP_END,
            special_s_jump_init,
        )
        .install();
}
