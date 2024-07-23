use super::*;

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color_1 = VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1);
    let color_2 = VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2);
    if color_1 == 1 {
        if color_2 == 2 {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, statuses::palutena::SPECIAL_N_P);
            //println!("and why he ourple");
            return 1.into();
        }
        else if color_2 == 3 {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, statuses::palutena::SPECIAL_N_O);
            //println!("bornana");
            return 1.into();
        }
        else {
            if VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 1 {
                VarModule::on_flag(fighter.object(), vars::palutena::instance::POWERED);
            }
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, statuses::palutena::SPECIAL_N_R);
            //println!("red");
            return 1.into();
        }
    }
    else if color_1 == 2 {
        if color_2 == 1 {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, statuses::palutena::SPECIAL_N_P);
            //println!("and why he ourple");
            return 1.into();
        }
        else if color_2 == 3 {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, statuses::palutena::SPECIAL_N_G);
            //println!("i like cash from my hair to my ass");
            return 1.into();
        }
        else {
            if VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 2 {
                VarModule::on_flag(fighter.object(), vars::palutena::instance::POWERED);
            }
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, statuses::palutena::SPECIAL_N_B);
            //println!("blud");
            return 1.into();
        }
    }
    else if color_1 == 3 {
        if color_2 == 1 {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, statuses::palutena::SPECIAL_N_O);
            //println!("bornana");
            return 1.into();
        }
        else if color_2 == 2 {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, statuses::palutena::SPECIAL_N_G);
            //println!("i like cash from my hair to my ass");
            return 1.into();
        }
        else {
            if VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 3 {
                VarModule::on_flag(fighter.object(), vars::palutena::instance::POWERED);
            }
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, statuses::palutena::SPECIAL_N_Y);
            //println!("ielo");
            return 1.into();
        }
    }

    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
}

unsafe extern "C" fn special_n_color_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_n_main_common(fighter: &mut L2CFighterCommon, g_mot: u64, a_mot: u64) {
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(g_mot), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(a_mot), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe extern "C" fn special_n_main_loop_common(fighter: &mut L2CFighterCommon, g_mot: u64, a_mot: u64) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(g_mot), -1.0, 1.0, 0.0, false, false);
            }
            else {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(a_mot), -1.0, 1.0, 0.0, false, false);
            }
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    return 0.into()
}

// red: burn attack
unsafe extern "C" fn special_n_r_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_common(fighter, hash40("special_n_r"), hash40("special_air_n_r"));
    
    if VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 1 {
        VarModule::on_flag(fighter.object(), vars::palutena::instance::POWERED);
    }
    
    fighter.main_shift(special_n_r_main_loop)
}

unsafe extern "C" fn special_n_r_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_loop_common(fighter, hash40("special_n_r"), hash40("special_air_n_r"))
}

unsafe extern "C" fn special_n_r_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.object(), vars::palutena::instance::POWERED);
    return 0.into()
}

// blue: ice attack
unsafe extern "C" fn special_n_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_common(fighter, hash40("special_n_b"), hash40("special_air_n_b"));

    if VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 2 {
        VarModule::on_flag(fighter.object(), vars::palutena::instance::POWERED);
    }
    
    fighter.main_shift(special_n_b_main_loop)
}

unsafe extern "C" fn special_n_b_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_loop_common(fighter, hash40("special_n_b"), hash40("special_air_n_b"))
}

unsafe extern "C" fn special_n_b_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.object(), vars::palutena::instance::POWERED);
    return 0.into()
}

// yellow: paralyze attack
unsafe extern "C" fn special_n_y_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_common(fighter, hash40("special_n_y"), hash40("special_air_n_y"));

    if VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 3 {
        VarModule::on_flag(fighter.object(), vars::palutena::instance::POWERED);
    }
    
    fighter.main_shift(special_n_y_main_loop)
}

unsafe extern "C" fn special_n_y_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_loop_common(fighter, hash40("special_n_y"), hash40("special_air_n_y"))
}

unsafe extern "C" fn special_n_y_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.object(), vars::palutena::instance::POWERED);
    return 0.into()
}

// purple: shake attack
unsafe extern "C" fn special_n_p_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_common(fighter, hash40("special_n_p"), hash40("special_air_n_p"));

    fighter.main_shift(special_n_p_main_loop)
}

unsafe extern "C" fn special_n_p_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_loop_common(fighter, hash40("special_n_p"), hash40("special_air_n_p"))
}

unsafe extern "C" fn special_n_p_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.boma(), *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return 0.into()
}

// orange: libra sponge
unsafe extern "C" fn special_n_o_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_common(fighter, hash40("special_n_o"), hash40("special_air_n_o"));

    fighter.main_shift(special_n_o_main_loop)
}

unsafe extern "C" fn special_n_o_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_loop_common(fighter, hash40("special_n_o"), hash40("special_air_n_o"))
}

// green: spin attack
unsafe extern "C" fn special_n_g_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_common(fighter, hash40("special_n_g"), hash40("special_air_n_g"));

    fighter.main_shift(special_n_g_main_loop)
}

unsafe extern "C" fn special_n_g_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_loop_common(fighter, hash40("special_n_g"), hash40("special_air_n_g"))
}

unsafe extern "C" fn special_n_g_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    STOP_SE(fighter, Hash40::new("se_item_club_wind"));
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre);

    agent.status(Pre, statuses::palutena::SPECIAL_N_R, special_n_color_pre);
    agent.status(Main, statuses::palutena::SPECIAL_N_R, special_n_r_main);
    agent.status(End, statuses::palutena::SPECIAL_N_R, special_n_r_end);

    agent.status(Pre, statuses::palutena::SPECIAL_N_B, special_n_color_pre);
    agent.status(Main, statuses::palutena::SPECIAL_N_B, special_n_b_main);
    agent.status(End, statuses::palutena::SPECIAL_N_B, special_n_b_end);

    agent.status(Pre, statuses::palutena::SPECIAL_N_Y, special_n_color_pre);
    agent.status(Main, statuses::palutena::SPECIAL_N_Y, special_n_y_main);
    agent.status(End, statuses::palutena::SPECIAL_N_Y, special_n_y_end);

    agent.status(Pre, statuses::palutena::SPECIAL_N_P, special_n_color_pre);
    agent.status(Main, statuses::palutena::SPECIAL_N_P, special_n_p_main);
    agent.status(End, statuses::palutena::SPECIAL_N_P, special_n_p_end);

    agent.status(Pre, statuses::palutena::SPECIAL_N_O, special_n_color_pre);
    agent.status(Main, statuses::palutena::SPECIAL_N_O, special_n_o_main);

    agent.status(Pre, statuses::palutena::SPECIAL_N_G, special_n_color_pre);
    agent.status(Main, statuses::palutena::SPECIAL_N_G, special_n_g_main);
    agent.status(End, statuses::palutena::SPECIAL_N_G, special_n_g_end);
}