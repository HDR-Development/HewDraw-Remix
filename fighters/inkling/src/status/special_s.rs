use super::*;
use globals::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);

    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        return ret;
    }
    
    // Gives Inkling a slight hop on sideB start
    let speed_y = 0.75;
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);

    ret
}

// FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK

pub unsafe extern "C" fn special_s_walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Once per airtime
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S);
    }

    smashline::original_status(Main, fighter, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK)(fighter);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_walk_main_loop as *const () as _))    
}

unsafe extern "C" fn special_s_walk_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_S_WORK_INT_ALL_FRAME);
    let enable_finish_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_S_WORK_INT_ENABLE_FINISH_FRAME);

    if current_frame >= enable_finish_frame
    && (fighter.is_cat_flag(Cat2::CommonGuard)
        || fighter.is_cat_flag(Cat1::AttackN)
        || fighter.is_cat_flag(Cat1::SpecialAny)) {
        fighter.change_status(FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }

    if let Some(target) = smashline::api::get_target_function("lua2cpp_inkling.nrs", 0x28790) {
        let og_special_s_walk_main_loop: fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(target);
        return og_special_s_walk_main_loop(fighter);
    }

    0.into()
}

// FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN

pub unsafe extern "C" fn special_s_run_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Once per airtime
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S);
    }

    smashline::original_status(Main, fighter, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN)(fighter);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_run_main_loop as *const () as _))  
}

unsafe extern "C" fn special_s_run_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_S_WORK_INT_ALL_FRAME);
    let enable_finish_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_S_WORK_INT_ENABLE_FINISH_FRAME);

    if current_frame >= enable_finish_frame
    && (fighter.is_cat_flag(Cat2::CommonGuard)
        || fighter.is_cat_flag(Cat1::AttackN)
        || fighter.is_cat_flag(Cat1::SpecialAny)) {
        fighter.change_status(FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }

    if let Some(target) = smashline::api::get_target_function("lua2cpp_inkling.nrs", 0x24250) {
        let og_special_s_run_main_loop: fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(target);
        return og_special_s_run_main_loop(fighter);
    }

    0.into()
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
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(Main, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK, special_s_walk_main);
    agent.status(Main, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN, special_s_run_main);
    agent.status(Init, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_JUMP_END, special_s_jump_end_init);
}
