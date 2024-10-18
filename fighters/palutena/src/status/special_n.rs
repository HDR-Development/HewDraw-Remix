use super::*;

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color_1 = VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1);
    let color_2 = VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2);
    if color_1 == 1 {
        if color_2 == 2 {
            fighter.set_status_kind_interrupt(statuses::palutena::SPECIAL_N_P);
            //println!("and why he ourple");
            return 1.into();
        }
        else if color_2 == 3 {
            fighter.set_status_kind_interrupt(statuses::palutena::SPECIAL_N_O);
            //println!("bornana");
            return 1.into();
        }
        else {
            fighter.set_status_kind_interrupt(statuses::palutena::SPECIAL_N_R);
            //println!("red");
            return 1.into();
        }
    }
    else if color_1 == 2 {
        if color_2 == 1 {
            fighter.set_status_kind_interrupt(statuses::palutena::SPECIAL_N_P);
            //println!("and why he ourple");
            return 1.into();
        }
        else if color_2 == 3 {
            fighter.set_status_kind_interrupt(statuses::palutena::SPECIAL_N_G);
            //println!("i like cash from my hair to my ass");
            return 1.into();
        }
        else {
            fighter.set_status_kind_interrupt(statuses::palutena::SPECIAL_N_B);
            //println!("blud");
            return 1.into();
        }
    }
    else if color_1 == 3 {
        if color_2 == 1 {
            fighter.set_status_kind_interrupt(statuses::palutena::SPECIAL_N_O);
            //println!("bornana");
            return 1.into();
        }
        else if color_2 == 2 {
            fighter.set_status_kind_interrupt(statuses::palutena::SPECIAL_N_G);
            //println!("i like cash from my hair to my ass");
            return 1.into();
        }
        else {
            fighter.set_status_kind_interrupt(statuses::palutena::SPECIAL_N_Y);
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

#[no_mangle]
unsafe extern "C" fn palutena_special_n_init_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    palutena_special_n_momentum_helper(fighter, true.into());

    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_SPECIAL_N_LANDING) {
        let special_n_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_speed_y_mul"));
        speed_y *= special_n_speed_y_mul;
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            let special_n_speed_y_add = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_speed_y_add"));
            speed_y += special_n_speed_y_add;
        }
    }
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        speed_y
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
    }
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        0.0
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.0,
        0.0
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    0.into()
}

#[no_mangle]
unsafe extern "C" fn palutena_special_n_momentum_helper(fighter: &mut L2CFighterCommon, start: L2CValue) {
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if start.get_bool() {
        let special_n_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_speed_x_mul"));
        speed_x *= special_n_speed_x_mul;
    }

    let reset_type = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        ENERGY_STOP_RESET_TYPE_GROUND
    }
    else {
        ENERGY_STOP_RESET_TYPE_AIR
    };
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        reset_type,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    if !start.get_bool() {
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
        }
    }
}

unsafe extern "C" fn special_n_main_common(fighter: &mut L2CFighterCommon, g_mot: u64, a_mot: u64) {
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(a_mot), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(g_mot), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe extern "C" fn special_n_main_loop_common(fighter: &mut L2CFighterCommon, g_mot: u64, a_mot: u64) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        palutena_special_n_momentum_helper(fighter, false.into());
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

unsafe extern "C" fn special_n_end_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, vars::palutena::status::POWER_BOARD_FLUSHED) {
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2, 0);
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1, 0);
        utils::ui::UiManager::change_power_board_color(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
        );
    }
    0.into()
}

// colorless
unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_common(fighter, hash40("special_n"), hash40("special_air_n"));

    fighter.main_shift(special_n_main_loop)
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_loop_common(fighter, hash40("special_n"), hash40("special_air_n"))
}

// red: burn attack
unsafe extern "C" fn special_n_r_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_common(fighter, hash40("special_n_r"), hash40("special_air_n_r"));

    if VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 1 {
        VarModule::on_flag(fighter.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED);
    }
    
    fighter.main_shift(special_n_r_main_loop)
}

unsafe extern "C" fn special_n_r_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_loop_common(fighter, hash40("special_n_r"), hash40("special_air_n_r"))
}

// blue: ice attack
unsafe extern "C" fn special_n_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_common(fighter, hash40("special_n_b"), hash40("special_air_n_b"));

    if VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 2 {
        VarModule::on_flag(fighter.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED);
    }
    
    fighter.main_shift(special_n_b_main_loop)
}

unsafe extern "C" fn special_n_b_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_loop_common(fighter, hash40("special_n_b"), hash40("special_air_n_b"))
}

// yellow: paralyze attack
unsafe extern "C" fn special_n_y_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_common(fighter, hash40("special_n_y"), hash40("special_air_n_y"));

    if VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 3 {
        VarModule::on_flag(fighter.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED);
    }
    
    fighter.main_shift(special_n_y_main_loop)
}

unsafe extern "C" fn special_n_y_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_main_loop_common(fighter, hash40("special_n_y"), hash40("special_air_n_y"))
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
    special_n_end_common(fighter)
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
    special_n_end_common(fighter)
}

unsafe extern "C" fn special_n_g_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    STOP_SE(fighter, Hash40::new("se_item_club_wind"));
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, palutena_special_n_init_common);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_end_common);

    agent.status(Pre, statuses::palutena::SPECIAL_N_R, special_n_color_pre);
    agent.status(Init, statuses::palutena::SPECIAL_N_R, palutena_special_n_init_common);
    agent.status(Main, statuses::palutena::SPECIAL_N_R, special_n_r_main);
    agent.status(End, statuses::palutena::SPECIAL_N_R, special_n_end_common);

    agent.status(Pre, statuses::palutena::SPECIAL_N_B, special_n_color_pre);
    agent.status(Init, statuses::palutena::SPECIAL_N_B, palutena_special_n_init_common);
    agent.status(Main, statuses::palutena::SPECIAL_N_B, special_n_b_main);
    agent.status(End, statuses::palutena::SPECIAL_N_B, special_n_end_common);

    agent.status(Pre, statuses::palutena::SPECIAL_N_Y, special_n_color_pre);
    agent.status(Init, statuses::palutena::SPECIAL_N_Y, palutena_special_n_init_common);
    agent.status(Main, statuses::palutena::SPECIAL_N_Y, special_n_y_main);
    agent.status(End, statuses::palutena::SPECIAL_N_Y, special_n_end_common);

    agent.status(Pre, statuses::palutena::SPECIAL_N_P, special_n_color_pre);
    agent.status(Init, statuses::palutena::SPECIAL_N_P, palutena_special_n_init_common);
    agent.status(Main, statuses::palutena::SPECIAL_N_P, special_n_p_main);
    agent.status(End, statuses::palutena::SPECIAL_N_P, special_n_p_end);

    agent.status(Pre, statuses::palutena::SPECIAL_N_O, special_n_color_pre);
    agent.status(Init, statuses::palutena::SPECIAL_N_O, palutena_special_n_init_common);
    agent.status(Main, statuses::palutena::SPECIAL_N_O, special_n_o_main);
    agent.status(End, statuses::palutena::SPECIAL_N_O, special_n_end_common);

    agent.status(Pre, statuses::palutena::SPECIAL_N_G, special_n_color_pre);
    agent.status(Init, statuses::palutena::SPECIAL_N_G, palutena_special_n_init_common);
    agent.status(Main, statuses::palutena::SPECIAL_N_G, special_n_g_main);
    agent.status(End, statuses::palutena::SPECIAL_N_G, special_n_g_end);
    agent.status(Exit, statuses::palutena::SPECIAL_N_G, special_n_g_exit);
}