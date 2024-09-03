use super::*;

// pub unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.set_int(0x50000000, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_BATTLE_OBJECT_ID_BLUNDERBUSS);
//     fighter.set_int(0x50000000, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_BATTLE_OBJECT_ID_SPITBALL);
//     if fighter.kind() != *FIGHTER_KIND_KIRBY {
//         ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT, false, -1);
//     }
//     ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, false, -1);
//     ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, *WEAPON_KROOL_BLUNDERBUSS_STATUS_KIND_FIRE);
//     special_n_set_kinetic(fighter);
//     if !StopModule::is_stop(fighter.module_accessor) {
//         special_n_substatus(fighter, false);
//     }
//     fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_n_substatus as *const () as _));
//     fighter.off_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_SUCTION_IRONBALL);
//     fighter.set_int(*FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_NONE, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_N_SPIT_TYPE);
//     fighter.off_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CATCH_CUT);

//     fighter.main_shift(special_n_main_loop)
// }

// // FUN_710002a5b0
// pub unsafe extern "C" fn special_n_substatus(fighter: &mut L2CFighterCommon, param_1: bool) -> L2CValue {
//     if !param_1 {
//         if !fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL_END) {
//             if fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL) {
//                 if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_IRONBALL) {
//                     fighter.on_flag(*FIGHTER_KROOL_GENERATE_ARTICLE_IRONBALL);
//                 }
//                 else {
//                     ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_IRONBALL, false, -1);
//                     notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
//                 }
//                 fighter.on_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL_END);
//                 let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
//                 let back_air_spd_y = fighter.get_param_float("param_special_n", "special_n_back_air_spd_y");
//                 if fighter.is_situation(*SITUATION_KIND_AIR) && sum_speed_y < -0.7 {
//                     sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
//                     sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, sum_speed_y + back_air_spd_y);
//                 }
//                 KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
//             }
//             let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
//             if fighter.is_situation(*SITUATION_KIND_AIR) {
//                 let back_air_spd_x = fighter.get_param_float("param_special_n", "special_n_back_air_spd_x");
//                 let facing = fighter.lr();
//                 sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
//                 sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, back_air_spd_x * facing - sum_speed_x, 0.0);
//             }
//             else {
//                 let back_spd_x = fighter.get_param_float("param_special_n", "special_n_back_spd_x");
//                 let facing = fighter.lr();
//                 sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
//                 sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, back_spd_x * facing - sum_speed_x, 0.0);
//             }
//         }
//     }

//     return 0.into();
// }

// pub unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if !StatusModule::is_changing(fighter.module_accessor) {
//         if fighter.is_prev_situation(*SITUATION_KIND_GROUND) {
//             if fighter.is_situation(*SITUATION_KIND_AIR) {
//                 special_n_change_motion(fighter, Hash40::new("special_n_fire"), Hash40::new("special_air_n_fire"));
//             }
//         }
//         if fighter.is_situation(*SITUATION_KIND_GROUND) {
//             special_n_change_motion(fighter, Hash40::new("special_n_fire"), Hash40::new("special_air_n_fire"));
//         }
//     }
//     if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
//         if CancelModule::is_enable_cancel(fighter.module_accessor) {
//             if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
//                 return 1.into();
//             }
//             else {
//                 if fighter.sub_air_check_fall_common().get_bool() {
//                     return 1.into();
//                 }
//             }
//         }
//     }
//     let pass_speed_y = fighter.get_param_float("common", "pass_speed_y");
//     if fighter.stick_y() <= pass_speed_y {
//         if GroundModule::is_passable_check(fighter.module_accessor) {
//             GroundModule::set_passable_check(fighter.module_accessor, false);
//         }
//     }
//     else {
//         if fighter.is_situation(*SITUATION_KIND_AIR) {
//             GroundModule::set_passable_check(fighter.module_accessor, true);
//         }
//         if GroundModule::is_passable_ground(fighter.module_accessor) {
//             let flick_speed_y = fighter.get_param_float("common", "pass_flick_y");
//             if fighter.global_table[FLICK_Y].get_f32() < flick_speed_y {
//                 GroundModule::set_passable_check(fighter.module_accessor, true);
//                 fighter.set_situation(SITUATION_KIND_AIR.into());
//                 KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
//                 KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
//             }
//         }
//     }
//     if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
//         if !fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_CANCEL) {
//             if fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL) {
//                 if fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_IRONBALL) {
//                     fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SUCTION.into(), true.into());
//                     return 0.into();
//                 }
//             }
//             if MotionModule::is_end(fighter.module_accessor) {
//                 let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT.into() } else { FIGHTER_STATUS_KIND_FALL.into() };
//                 fighter.change_status(status, false.into());
//                 return 0.into();
//             }
//             if fighter.is_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_DROP_CROWN) {
//                 VisibilityModule::set_int64(fighter.module_accessor, hash40("crown") as i64, hash40("crown_hide") as i64);
//             }
//         }
//         fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SUCTION.into(), true.into());
//         return 0.into();
//     }
//     else {
//         if MotionModule::is_end(fighter.module_accessor) {
//             let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT.into() } else { FIGHTER_STATUS_KIND_FALL.into() };
//             fighter.change_status(status, false.into());
//             return 0.into();
//         }
//         if fighter.is_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_DROP_CROWN) {
//             VisibilityModule::set_int64(fighter.module_accessor, hash40("crown") as i64, hash40("crown_hide") as i64);
//         }
//     }

//     return 0.into();
// }

unsafe extern "C" fn special_n_fire_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    0.into()
}

pub unsafe extern "C" fn special_n_fire_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_int(0x50000000, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_BATTLE_OBJECT_ID_BLUNDERBUSS);
    fighter.set_int(0x50000000, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_BATTLE_OBJECT_ID_SPITBALL);
    //fighter.on_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SPIT);
    if fighter.kind() != *FIGHTER_KIND_KIRBY {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_PIRATEHAT, false, -1);
    }
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, false, -1);
    //ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, *WEAPON_KROOL_BLUNDERBUSS_STATUS_KIND_SPIT);
    ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, *WEAPON_KROOL_BLUNDERBUSS_STATUS_KIND_FIRE);
    // if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
    //     let spit_type = fighter.get_int(*FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_N_SPIT_TYPE);
    //     if spit_type == *FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_F {
    //         ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("spit_f"), false, -1.0);
    //     }
    //     else if spit_type == *FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_B {
    //         ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("spit_b"), false, -1.0);
    //     }
    //     else {
    //         ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("spit_hi"), false, -1.0);
    //     }
    // }
    special_n_set_kinetic(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        special_n_fire_hi_substatus(fighter, false);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_n_fire_hi_substatus as *const () as _));
    fighter.main_shift(special_n_fire_hi_main_loop)
}

pub unsafe extern "C" fn special_n_fire_hi_substatus(fighter: &mut L2CFighterCommon, param_1: bool) -> L2CValue {
    if param_1 {
        let mut motion_2nd_waight = fighter.get_float(*FIGHTER_KROOL_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_2ND_WAIGHT);
        if motion_2nd_waight < 0.0 {
            motion_2nd_waight = (motion_2nd_waight - 0.1666667).abs().max(0.0);
            fighter.set_float(motion_2nd_waight, *FIGHTER_KROOL_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_2ND_WAIGHT);
        }
    }
    else {
        //if fighter.is_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_SUCTION_IRONBALL) {
            if !fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL_END) {
                if fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL) {
                    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_SPITBALL, false, -1);
                    fighter.on_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL_END);
                }
            }
        //}
    }

    return 0.into();
}

// FUN_7100025c00
pub unsafe extern "C" fn special_n_set_kinetic(fighter: &mut L2CFighterCommon) {
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_mul_spd_x = fighter.get_param_float("param_special_n", "special_n_start_mul_spd_x");
    let start_air_mul_spd_x = fighter.get_param_float("param_special_n", "special_n_start_air_mul_spd_x");
    let start_mul_spd_y = fighter.get_param_float("param_special_n", "special_n_start_mul_spd_y");
    if !fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_N {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    else {
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
    }
}

pub unsafe extern "C" fn special_n_fire_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let spit_type = fighter.get_int(*FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_N_SPIT_TYPE);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if spit_type == *FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_F {
                special_n_change_motion(fighter, Hash40::new("special_n_spit_f"), Hash40::new("special_air_n_spit_f"));
            }
            else if spit_type == *FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_B {
                special_n_change_motion(fighter, Hash40::new("special_n_spit_b"), Hash40::new("special_air_n_spit_b"));
            }
            else {
                special_n_change_motion(fighter, Hash40::new("special_n_spit_hi"), Hash40::new("special_air_n_spit_hi"));
            }
        }
    }
    if spit_type == *FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_F {
        special_n_change_motion(fighter, Hash40::new("special_n_spit_f"), Hash40::new("special_air_n_spit_f"));
    }
    else if spit_type == *FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_B {
        special_n_change_motion(fighter, Hash40::new("special_n_spit_b"), Hash40::new("special_air_n_spit_b"));
    }
    else {
        special_n_change_motion(fighter, Hash40::new("special_n_spit_hi"), Hash40::new("special_air_n_spit_hi"));
    }
    if MotionModule::motion_kind_2nd(fighter.module_accessor) != hash40("special_n_spit_f") {
        let mut motion_2nd_waight = fighter.get_float(*FIGHTER_KROOL_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_2ND_WAIGHT);
        MotionModule::set_weight(fighter.module_accessor, 1.0 - motion_2nd_waight, true);
        if motion_2nd_waight <= 0.0 {
            let frame = MotionModule::frame(fighter.module_accessor);
            let rate = MotionModule::rate(fighter.module_accessor);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("invalid"), frame, rate, false, 0.0);
        }
    }
    if spit_type == *FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_F {
        if !fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SPIT_TYPE_DECIDE) {
            decide_spit_type(fighter);
            if !spit_type == *FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_HI {
                if spit_type == *FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_B {
                    special_n_change_motion(fighter, Hash40::new("special_n_spit_b"), Hash40::new("special_air_n_spit_b"));
                    let frame = MotionModule::frame(fighter.module_accessor);
                    let rate = MotionModule::rate(fighter.module_accessor);
                    MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_n_spit_f"), frame, rate, false, 0.0);
                    fighter.set_float(1.0, *FIGHTER_KROOL_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_2ND_WAIGHT);
                    let attack_abs_frame = fighter.get_param_int("param_special_n", "special_n_attack_abs_frame");
                    if !AttackModule::is_attack(fighter.module_accessor, 0, true) {
                        if (attack_abs_frame - 1).to_f32() <= frame {
                            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_specialnspitbattackabs"), -1);
                        }
                    }
                }
            }
            else {
                special_n_change_motion(fighter, Hash40::new("special_n_spit_hi"), Hash40::new("special_air_n_spit_hi"));
                let frame = MotionModule::frame(fighter.module_accessor);
                let rate = MotionModule::rate(fighter.module_accessor);
                MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_n_spit_f"), frame, rate, false, 0.0);
                fighter.set_float(1.0, *FIGHTER_KROOL_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_2ND_WAIGHT);
                let lvar1 = -0x60;
            }
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
                let spit_type = fighter.get_int(*FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_N_SPIT_TYPE);
                if spit_type != *FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_HI {
                    if spit_type == *FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_B {
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("spit_b"), true, -1.0);
                    }
                }
                else {
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, Hash40::new("spit_hi"), true, -1.0);
                }
            }
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT.into() } else { FIGHTER_STATUS_KIND_FALL.into() };
        fighter.change_status(status, false.into());
    }
    else {
        if fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SPIT) {
            fighter.on_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SPIT_END);
            fighter.off_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SPIT);
            //if fighter.is_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_SUCTION_IRONBALL) {
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_SPITBALL, false, -1);
            //}
        }
        check_crown_vis(fighter);
    }

    return 0.into();
}

// FUN_7100027ed0
pub unsafe extern "C" fn special_n_change_motion(fighter: &mut L2CFighterCommon, hash1: Hash40, hash2: Hash40) {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_FIRST) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if fighter.is_flag(*FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_FIRST) {
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

// FUN_7100028320
unsafe extern "C" fn decide_spit_type(fighter: &mut L2CFighterCommon) {
    let spit_angle = fighter.get_param_float("param_special_n", "special_n_spit_angle");
    let this_00 = fighter.stick_y();
    let plvar5 = fighter.stick_x();
    let l90 = fighter.lr();
    let plvar4 = this_00.atan2(plvar5);
    let la0 = plvar5 * plvar5;
    let lb0 = this_00 * this_00;
    let a90 = la0 + lb0;
    let plvar6 = a90.sqrt();
    if plvar6 >= 0.1 {
        let a90_20 = (90.0 - spit_angle).to_radians();
        if a90_20 >= plvar6 {
            let a90_10 = (90.0 + spit_angle).to_radians();
            if a90_10 >= a90_20 {
                fighter.set_int(*FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_HI, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_N_SPIT_TYPE);
            }
            else {
                fighter.set_int(*FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_B, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_N_SPIT_TYPE);
            }
        }
        else {
            fighter.set_int(*FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_F, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_N_SPIT_TYPE);
        }
    }
    else {
        fighter.set_int(*FIGHTER_KROOL_SPECIAL_N_SPIT_TYPE_F, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_N_SPIT_TYPE);
    }
}

unsafe extern "C" fn check_crown_vis(fighter: &mut L2CFighterCommon) {
    if fighter.is_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_DROP_CROWN) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("crown") as i64, hash40("crown_hide") as i64);
    }
}

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(statuses::krool::SPECIAL_N_FIRE_HI.into(), false.into());
   
    0.into()
}

pub fn install(agent: &mut Agent) {
    //agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    //agent.status(Pre, statuses::krool::SPECIAL_N_FIRE_HI, special_n_fire_hi_pre);
    //agent.status(Main, statuses::krool::SPECIAL_N_FIRE_HI, special_n_fire_hi_main);
}