use super::*;

pub unsafe extern "C" fn krool_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int(0x50000000, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_BATTLE_OBJECT_ID_BLUNDERBUSS);
    fighter.set_int(0x50000000, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_BATTLE_OBJECT_ID_SPITBALL);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, false, -1);
    ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, *WEAPON_KROOL_BLUNDERBUSS_STATUS_KIND_FIRE);
    krool_special_n_change_motion(fighter, Hash40::new("krool_special_n_fire"), Hash40::new("krool_special_air_n_fire"));
    krool_special_n_set_kinetic(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        krool_special_n_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(krool_special_n_substatus as *const () as _));
    fighter.off_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_SUCTION_IRONBALL);
    fighter.set_int(*FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_NONE, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_N_SPIT_TYPE);
    fighter.off_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CATCH_CUT);

    fighter.main_shift(krool_special_n_main_loop)
}

// // FUN_710002a5b0
pub unsafe extern "C" fn krool_special_n_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if !fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL_END) {
            if fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL) {
                if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_IRONBALL) {
                    fighter.on_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_IRONBALL);
                }
                else {
                    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_IRONBALL, false, -1);
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
                }
                fighter.on_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL_END);
                let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                let back_spd_x = fighter.get_param_float("param_special_n", "special_n_back_spd_x");
                let back_air_spd_x = fighter.get_param_float("param_special_n", "special_n_back_air_spd_x");
                let back_air_spd_y = fighter.get_param_float("param_special_n", "special_n_back_air_spd_y");
                if fighter.is_situation(*SITUATION_KIND_AIR) && sum_speed_y < -0.7 {
                    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
                    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, sum_speed_y + back_air_spd_y);
                }
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                if fighter.is_situation(*SITUATION_KIND_AIR) {
                    let facing = fighter.lr();
                    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
                    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x - back_air_spd_x * facing, 0.0);
                }
                else {
                    let facing = fighter.lr();
                    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
                    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x - back_spd_x * facing, 0.0);
                }
            }
        }
    }

    return 0.into();
}

pub unsafe extern "C" fn krool_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_motion_one_of(&[Hash40::new("krool_special_n_fire_hi"), Hash40::new("krool_special_air_n_fire_hi")]) {
            krool_special_n_change_motion(fighter, Hash40::new("krool_special_n_fire_hi"), Hash40::new("krool_special_air_n_fire_hi"));
        }
        else if fighter.is_motion_one_of(&[Hash40::new("krool_special_n_fire_b"), Hash40::new("krool_special_air_n_fire_b")]) {
            krool_special_n_change_motion(fighter, Hash40::new("krool_special_n_fire_b"), Hash40::new("krool_special_air_n_fire_b"));
        }
        else {
            krool_special_n_change_motion(fighter, Hash40::new("krool_special_n_fire"), Hash40::new("krool_special_air_n_fire"));
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
        else {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    let pass_speed_y = fighter.get_param_float("common", "pass_speed_y");
    if fighter.stick_y() <= pass_speed_y {
        if GroundModule::is_passable_check(fighter.module_accessor) {
            GroundModule::set_passable_check(fighter.module_accessor, false);
        }
    }
    else {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            GroundModule::set_passable_check(fighter.module_accessor, true);
        }
        if GroundModule::is_passable_ground(fighter.module_accessor) {
            let flick_speed_y = fighter.get_param_float("common", "pass_flick_y");
            if fighter.global_table[FLICK_Y].get_f32() < flick_speed_y {
                GroundModule::set_passable_check(fighter.module_accessor, true);
                fighter.set_situation(SITUATION_KIND_AIR.into());
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            }
        }
    }
    // angled shots
    if VarModule::is_flag(fighter.battle_object, vars::krool::status::SPECIAL_N_ANGLED) {
        VarModule::off_flag(fighter.battle_object, vars::krool::status::SPECIAL_N_ANGLED);
        if fighter.stick_y() > 0.5 {
            let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) { Hash40::new("krool_special_n_fire_hi") } else { Hash40::new("krool_special_air_n_fire_hi") };
            MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, *WEAPON_KROOL_BLUNDERBUSS_STATUS_KIND_SPIT);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("spit_hi"), true, 0.0);
            fighter.set_int(*FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_HI, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_N_SPIT_TYPE);
            return 0.into();
        }
        else if PostureModule::lr(fighter.module_accessor) * fighter.stick_x() < 0.0 {
            let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) { Hash40::new("krool_special_n_fire_b") } else { Hash40::new("krool_special_air_n_fire_b") };
            MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, *WEAPON_KROOL_BLUNDERBUSS_STATUS_KIND_SPIT);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("spit_b"), true, 0.0);
            return 0.into();
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::krool::instance::SPECIAL_N_GRAB) {
        if fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_CANCEL) {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_KROOL_SPECIAL_N_SUCTION.into(), true.into());
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
    }
    if fighter.is_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_DROP_CROWN) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("crown") as i64, hash40("crown_hide") as i64);
    }

    return 0.into();
}

// FUN_7100025c00
pub unsafe extern "C" fn krool_special_n_set_kinetic(fighter: &mut L2CFighterCommon) {
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_mul_spd_x = fighter.get_param_float("param_special_n", "special_n_start_mul_spd_x");
    let start_air_mul_spd_x = fighter.get_param_float("param_special_n", "special_n_start_air_mul_spd_x");
    let start_mul_spd_y = fighter.get_param_float("param_special_n", "special_n_start_mul_spd_y");
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_N {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, sum_speed_y * start_mul_spd_y);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x * start_mul_spd_x, 0.0);
        }
        else {
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x * start_air_mul_spd_x, 0.0);
        }
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

// FUN_7100027ed0
pub unsafe extern "C" fn krool_special_n_change_motion(fighter: &mut L2CFighterCommon, hash1: Hash40, hash2: Hash40) {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_FIRST) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, hash2, 0.0, 1.0, false, 0.0, false, false);
            fighter.on_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_FIRST);
            return;
        }
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, hash2, -1.0, 1.0, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if !fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_FIRST) {
            MotionModule::change_motion(fighter.module_accessor, hash1, 0.0, 1.0, false, 0.0, false, false);
            fighter.on_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_FIRST);
            return;
        }
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, hash1, -1.0, 1.0, 0.0, false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_KROOL_SPECIAL_N, krool_special_n_main);
}