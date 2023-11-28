use super::*;
use globals::*;


// FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK

#[status_script(agent = "inkling", status = FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_walk(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Once per airtime
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S);
    }
    original!(fighter)
}

// FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN

#[status_script(agent = "inkling", status = FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Once per airtime
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S);
    }
    original!(fighter)
}

// FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_JUMP_END

#[status_script(agent = "inkling", status = FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_JUMP_END, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn special_s_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Burn double jump when jumping out of Splat Roller
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
    && fighter.get_num_used_jumps() < fighter.get_jump_count_max() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        special_s_walk,
        special_s_run,
        special_s_jump_init
    );
}