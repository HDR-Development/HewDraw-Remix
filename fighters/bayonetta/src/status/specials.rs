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
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND { //gr checks
        bayonetta_special_s_slow_hit(fighter);
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        && !fighter.is_in_hitlag()
        && VarModule::get_int(fighter.battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID) < 3 {
            if fighter.is_cat_flag(Cat1::SpecialAny | Cat1::AttackN)
            && fighter.global_table[CURRENT_FRAME].get_i32() >= 20
            && fighter.global_table[CURRENT_FRAME].get_i32() <= 35 {
                GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                fighter.change_status(statuses::bayonetta::SPECIAL_S_KICK.into(), true.into());
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT_CANCEL_OK);
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
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
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.45);
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.67);
        }
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_run_smoke"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_heelslide_trace"), false, false);
        if fighter.motion_frame() >= 45.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

// FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_KICK //

unsafe extern "C" fn bayonetta_special_s_kick_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn bayonetta_special_s_kick_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x12b6d89003), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.1);
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
    if VarModule::get_int(fighter.battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID) < 3 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, shield_x);
        } else if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, mul_x);
        }
    }
    0.into()
}

pub fn install() {
    smashline::Agent::new("bayonetta")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, bayonetta_special_s_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, bayonetta_special_s_end)
        .install();
    smashline::Agent::new("bayonetta")
        .status(Pre, statuses::bayonetta::SPECIAL_S_KICK, bayonetta_special_s_kick_pre)
        .status(Main, statuses::bayonetta::SPECIAL_S_KICK, bayonetta_special_s_kick_main)
        .status(End, statuses::bayonetta::SPECIAL_S_KICK, bayonetta_special_s_kick_end)
        .install();
}