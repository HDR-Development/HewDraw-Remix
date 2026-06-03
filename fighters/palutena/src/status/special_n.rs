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
    fighter.sub_status_pre_SpecialNCommon();
    
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
    fighter.sub_air_check_dive();
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

    return 0.into();
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
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    fighter.sub_air_check_dive();
    if fighter.motion_frame() >= 18.0 {
        fighter.check_land_cancel(Some(14.0));
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        palutena_special_n_momentum_helper(fighter, false.into());
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_p"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_p"), -1.0, 1.0, 0.0, false, false);
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
    
    return 0.into();
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

unsafe extern "C" fn special_n_g_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
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

unsafe extern "C" fn special_n_g_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    // force airborne
    fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());
    VarModule::on_flag(fighter.battle_object, vars::common::status::DISABLE_ECB_SHIFT);
    // start speed mul
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y * 0.7, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, speed_x * 0.7, 0.0, 0.0, 0.0, 0.0);
    // tweak until it feels good idk
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.7);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.7, 0.0);

    return 0.into();
}

unsafe extern "C" fn special_n_g_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.battle_object, vars::palutena::status::SPECIAL_N_GREEN_LOOP, 1);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_g_start"), 0.0, 1.0, false, 0.0, false, false);
    fighter.main_shift(special_n_g_main_loop)
}

unsafe extern "C" fn special_n_g_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // end portion
    if fighter.is_motion_one_of(&[Hash40::new("special_n_g_end"), Hash40::new("special_air_n_g_end")]) {
        return special_n_g_end_main_loop(fighter).into();
    }
    // rise portion
    if fighter.is_motion(Hash40::new("special_n_g_loop")) {
        return special_n_g_rise_main_loop(fighter).into();
    }
    // start portion
    if fighter.is_motion(Hash40::new("special_n_g_start")) {
        return special_n_g_start_main_loop(fighter).into();
    }
    return 0.into();
}

unsafe extern "C" fn special_n_g_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_g_loop"), 0.0, 1.0, false, 0.0, false, false);
        VarModule::on_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_N_FLUSH_BOARD);
    }
    return 1.into();
}

unsafe extern "C" fn special_n_g_rise_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let timer = VarModule::get_int(fighter.battle_object, vars::palutena::status::SPECIAL_N_GREEN_BUTTON_TIMER);
    let loop_count = VarModule::get_int(fighter.battle_object, vars::palutena::status::SPECIAL_N_GREEN_LOOP);
    // repeat loop if hop initiated in last 12 frames
    if MotionModule::is_end(fighter.module_accessor) {
        VarModule::inc_int(fighter.battle_object, vars::palutena::status::SPECIAL_N_GREEN_LOOP);
        if timer >= -7
        && loop_count < 2 {
            // loop
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_g_loop"), 0.0, 1.0, false, 0.0, false, false);
        } else {
            //VarModule::off_flag(fighter.battle_object, vars::common::status::DISABLE_ECB_SHIFT);
            VarModule::set_int(fighter.battle_object, vars::palutena::status::SPECIAL_N_GREEN_BUTTON_TIMER, 0);
            fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());
            let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            // check if gr, kinetics
            let ground = if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) && speed_y < 0.001 {true} else {false};
            if ground {
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
                fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
                // gr momentum transfer
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
                sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, speed_x*1.5, 0.0, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
                sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, -1.0);
                let ground_brake = fighter.get_param_float("ground_brake", "");
                sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ground_brake / 2.0, 0.0);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            } else {
                // pop up finisher
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, (speed_y + 0.5).clamp(-0.5, 1.0));
            }
            fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
            fighter.change_motion_by_situation("special_n_g_end", "special_air_n_g_end", 0.0, 1.0, false, 0.0, false, false);
            return 1.into();
        }
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        // during rising spin, press special to rise
        if timer <= 0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                VarModule::set_int(fighter.battle_object, vars::palutena::status::SPECIAL_N_GREEN_BUTTON_TIMER, 5);
                let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                let ground = if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) && speed_y < 0.001 {true} else {false};
                let add_y = if ground {1.5} else {0.9}; // rise off ground easier
                let max_y = if ground {1.5} else {1.2 - 0.0875*(loop_count as f32)}; // slower rise the longer the move goes on, 1.1125-0.85
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, (speed_y + add_y).min(max_y));
            }
        }
        VarModule::dec_int(fighter.battle_object, vars::palutena::status::SPECIAL_N_GREEN_BUTTON_TIMER);
    }
    
    return 0.into();
}

unsafe extern "C" fn special_n_g_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.sub_transition_group_check_air_cliff();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        // only fastfall when actionable
        fighter.sub_air_check_dive();
    }
    // cut speed on hitbox clear
    if VarModule::get_int(fighter.battle_object, vars::palutena::status::SPECIAL_N_GREEN_BUTTON_TIMER) == 1 {
        VarModule::set_int(fighter.battle_object, vars::palutena::status::SPECIAL_N_GREEN_BUTTON_TIMER, -1);
        if !fighter.is_situation(*SITUATION_KIND_GROUND) {
            let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y * 0.4, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, speed_x * 0.4, 0.0, 0.0, 0.0, 0.0);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_MOTION_FALL.into());
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
        fighter.change_motion_inherit_frame_by_situation("special_n_g_end", "special_air_n_g_end", -1.0, 1.0, 0.0, false, false);
    }
    return 0.into();
}


unsafe extern "C" fn special_n_g_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_club_tornado"), true, true);
    STOP_SE(fighter, Hash40::new("se_item_club_wind"));
    special_n_end_common(fighter)
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

    agent.status(Pre, statuses::palutena::SPECIAL_N_G, special_n_g_pre);
    agent.status(Init, statuses::palutena::SPECIAL_N_G, special_n_g_init);
    agent.status(Main, statuses::palutena::SPECIAL_N_G, special_n_g_main);
    agent.status(End, statuses::palutena::SPECIAL_N_G, special_n_g_end);
}