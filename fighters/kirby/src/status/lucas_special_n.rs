use super::*;

// SPECIAL N //

unsafe extern "C" fn lucas_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_FIRE.into(), false.into());
        return 0.into();
    }
    else {
        smashline::original_status(Pre, fighter, *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N)(fighter)
    }
    
}

// SPECIAL N HOLD //

unsafe extern "C" fn lucas_special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //
    // OLD SPECIAL N STATUS MAIN CODE //
    //
    // let time = fighter.get_param_int("param_special_n", "time");
    // let nobang_time = fighter.get_param_int("param_special_n", "nobang_time");
    // fighter.set_int(time, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_TIME);
    // fighter.set_int(nobang_time, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_NOBANG_TIME);
    // fighter.off_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_MOT_CHANGE);

    // if !StopModule::is_stop(fighter.module_accessor) {
    //     lucas_special_n_hold_main_sub_status(fighter, false.into());
    // }
    // fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr( lucas_special_n_hold_main_sub_status as *const () as _));
    // fighter.main_shift(lucas_special_n_hold_main_loop)
    if !StopModule::is_stop(fighter.module_accessor) {
        lucas_special_n_hold_main_sub_status(fighter, false.into());
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("lucas_special_n_hold"), 0.0, 1.0, false, 0.0, false, false);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr( lucas_special_n_hold_main_sub_status as *const () as _));
    fighter.main_shift(lucas_special_n_hold_main_loop)
    
}

unsafe fn lucas_special_n_check_explosion(fighter: &mut L2CFighterCommon) {
    //
    // OLD PK FREEZE EXPLOSION CODE //
    //
    // if fighter.is_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE) 
    //     && !fighter.is_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_ALREADY_GENERATED)
    // {
    //     fighter.off_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    //     fighter.on_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_ALREADY_GENERATED);
    //     ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE, false, -1);
    // }

    // if !fighter.is_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_ALREADY_GENERATED) {
    //     return;
    // }
    // WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_TIME, 0);
    // WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_NOBANG_TIME, 0);
    // if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE) {
    //     return;
    // }
    // if fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_NOBANG_TIME) <= 0 {
    //     if fighter.is_button_off(Buttons::Special) {
    //         ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), fighter.is_button_off(Buttons::Special));
    //         return;
    //     }
    // }
    // if dbg!(fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_TIME)) <= 0 {
    //     ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), fighter.is_button_off(Buttons::Special));
    //     return;
    // }
}

unsafe extern "C" fn lucas_special_n_hold_main_sub_status(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    // 
    // OLD SPECIAL N SUBSTATUS CODE //
    // 
    // if arg.get_bool() {
    //     lucas_special_n_check_explosion(fighter);
    // }
    // if !fighter.is_situation(*SITUATION_KIND_GROUND) {
    //     if !arg.get_bool() {
    //         WorkModule::dec_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME);
    //     }
    //     if fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME) > 0 {
    //         KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    //     } else {
    //         KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    //     }
    // }
    // 0.into()

    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        if !arg.get_bool() {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME);
        }
        if fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME) > 0 {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        } else {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    0.into()
}

unsafe extern "C" fn lucas_special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //
    // OLD SPECIAL N MAIN LOOP STATUS SCRIPT CODE //
    //
    // let nobang_time = fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_NOBANG_TIME);
    // if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE) 
    //     && nobang_time <= 0
    // {
    //     fighter.change_status(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE.into(), false.into());
    //     return 1.into();
    // }
    // let wait_mtrans_kind = fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_WAIT_MTRANS_KIND);
    // if StatusModule::is_changing(fighter.module_accessor) || fighter.is_situation(wait_mtrans_kind) {
    //     // else block
    //     lucas_special_n_hold_transition_g2a_kind(fighter, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_WAIT_MTRANS_KIND, *FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_MOT_CHANGE, 
    //         *FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_LUCAS_AIR_STOP_SPECIAL_N, Hash40::new("special_n_hold"), 
    //         Hash40::new("special_air_n_hold"), *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    // }
    // 1.into()

    /// let charge = "attack_up_charge_time"
    /// <is_ready_go>
    /// set_charge(charge)
    /// <once per frame>
    ///     dec_charge()
    ///     if charge == 0 then IS_ATTACK_UP = true && set_charge(charge)

    if !VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) && fighter.is_button_on(Buttons::Special) {
        if VarModule::countdown_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0) {
            //let charge_time = ParamModule::get_int(fighter.object(), ParamType::Agent, "attack_up_charge_time");
            VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, LUCAS_CHARGE_TIME);
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_FIRE.into(), false.into());
            return 1.into();
        }
    } else {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_END.into(), false.into());
        return 1.into();
    }
    let wait_mtrans_kind = fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_WAIT_MTRANS_KIND);
    if StatusModule::is_changing(fighter.module_accessor) || fighter.is_situation(wait_mtrans_kind) {
        // else block
        lucas_special_n_hold_transition_g2a_kind(fighter, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_WAIT_MTRANS_KIND, *FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_MOT_CHANGE, 
            *FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_LUCAS_AIR_STOP_SPECIAL_N, Hash40::new("lucas_special_n_hold"), 
            Hash40::new("lucas_special_air_n_hold"), *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    1.into()
}

unsafe extern "C" fn lucas_special_n_hold_transition_g2a_kind(
    fighter: &mut L2CFighterCommon,
    mtrans_kind_work_id: i32,
    flag_work_id: i32,
    ground_kinetic_type: i32,
    air_kinetic_type: i32,
    ground_motion_kind: Hash40,
    aerial_motion_kind: Hash40,
    ground_correct_kind: i32
) {
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, air_kinetic_type);
        if !fighter.is_flag(flag_work_id) {
            MotionModule::change_motion(fighter.module_accessor, aerial_motion_kind, 0.0, 1.0, false, 0.0, false, false);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, aerial_motion_kind, -1.0, 1.0, 0.0, false, false);
        }
        fighter.set_int(*SITUATION_KIND_GROUND, mtrans_kind_work_id);
    } else {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(ground_correct_kind));
        KineticModule::change_kinetic(fighter.module_accessor, ground_kinetic_type);
        if !fighter.is_flag(flag_work_id) {
            MotionModule::change_motion(fighter.module_accessor, ground_motion_kind, 0.0, 1.0, false, 0.0, false, false);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, ground_motion_kind, -1.0, 1.0, 0.0, false, false);
        }
        fighter.set_int(*SITUATION_KIND_AIR, mtrans_kind_work_id);
    }
    fighter.on_flag(flag_work_id);
}

pub fn install() {
    smashline::Agent::new("kirby")
        .status(
            Pre,
            *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N,
            lucas_special_n_pre,
        )
        .status(
            Main,
            *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_HOLD,
            lucas_special_n_hold_main,
        )
        .install();
}
