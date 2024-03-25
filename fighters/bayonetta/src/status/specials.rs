use super::*;
use globals::*;

// FIGHTER_STATUS_KIND_SPECIAL_S //

unsafe extern "C" fn bayonetta_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT_BEFORE_GUARD);
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
}

unsafe extern "C" fn bayonetta_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    let frame = fighter.global_table[CURRENT_FRAME].get_i32() + 1; //0 index or whatev
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND { //gr checks
        bayonetta_special_s_slow_hit(fighter);
        if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) && !fighter.is_in_hitlag() {
            if fighter.is_cat_flag(Cat1::SpecialAny | Cat1::AttackN) && frame >= 20 && frame <= 35 {
                GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                fighter.change_status(statuses::bayonetta::SPECIAL_S_KICK.into(), true.into());
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());}
        if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK) {
            let mut touch_wall = false;
            if PostureModule::lr(fighter.module_accessor) > 0.0 {
                touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32);
            } else {
                touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32);
            }
            if touch_wall {fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END.into(), false.into()); }
        }
    } else { //slide-off
        if MotionModule::is_end(fighter.module_accessor) {fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());}
        if frame >= 45 {fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into()); }
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if frame < 45 {fighter.change_status(statuses::bayonetta::SPECIAL_S_EDGE.into(), false.into());}
        }
    }
    0.into()
}

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_edge //

unsafe extern "C" fn bayonetta_special_s_edge_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn bayonetta_special_s_edge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[PREV_STATUS_FRAME].get_f32() - 18.0;
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_edge"), frame.clamp(0.0, 16.0), 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_special_s_edge_main_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_special_s_edge_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn bayonetta_special_s_edge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into()); }
    0.into()
}

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_KICK //

unsafe extern "C" fn bayonetta_special_s_kick_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn bayonetta_special_s_kick_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_hold_end"), 0.0, 1.0, false, 0.0, false, false);
    let frame = fighter.global_table[PREV_STATUS_FRAME].get_i32() - 20;
    let speed = 1.12 - (0.016 * frame as f32); //instant kick = 1.12, last second kick ~ 0.88
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_special_s_kick_main_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_special_s_kick_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_FORBID);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    0.into()
}

unsafe extern "C" fn bayonetta_special_s_kick_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into()); }
    bayonetta_special_s_slow_hit(fighter);
    0.into()
}

unsafe extern "C" fn bayonetta_special_s_slow_hit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mul_x = fighter.get_param_float("param_special_s", "hs_shooting_speed_mul_x");
    let shield_x = fighter.get_param_float("param_special_s", "guard_speed_mul_x");
    if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, shield_x);
        } else {
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, mul_x);
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, bayonetta_special_s_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, bayonetta_special_s_end);
    agent.status(Pre, statuses::bayonetta::SPECIAL_S_KICK, bayonetta_special_s_kick_pre);
    agent.status(Main, statuses::bayonetta::SPECIAL_S_KICK, bayonetta_special_s_kick_main);
    agent.status(End, statuses::bayonetta::SPECIAL_S_KICK, bayonetta_special_s_kick_end);
    agent.status(Pre, statuses::bayonetta::SPECIAL_S_EDGE, bayonetta_special_s_edge_pre);
    agent.status(Main, statuses::bayonetta::SPECIAL_S_EDGE, bayonetta_special_s_edge_main);
    agent.status(End, statuses::bayonetta::SPECIAL_S_EDGE, bayonetta_special_s_edge_end);
}