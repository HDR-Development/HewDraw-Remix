use super::*;
use globals::*;

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    }
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
}

unsafe extern "C" fn special_n_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    }
    smashline::original_status(Main, fighter, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE)(fighter)
}

unsafe extern "C" fn special_n_cancel_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_n_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.main_shift(special_n_cancel_main_loop)
}

unsafe extern "C" fn special_n_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_cancel"), -1.0, 1.0, 0.0, false, false);
        VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_NONE);
    }
    else if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_AIR && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_cancel"), -1.0, 1.0, 0.0, false, false);
        VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_NONE);
    }
    if VarModule::get_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE) == vars::diddy::SPECIAL_N_CANCEL_TYPE_NONE && MotionModule::is_end(fighter.module_accessor) {
        fighter.fastshift(L2CValue::Ptr(special_n_cancel_main_loop_electric_boogaloo as *const () as _))
    }
    else if VarModule::get_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE) != vars::diddy::SPECIAL_N_CANCEL_TYPE_NONE
        && (MotionModule::is_end(fighter.module_accessor) || (!MotionModule::is_end(fighter.module_accessor) && CancelModule::is_enable_cancel(fighter.module_accessor))) {
        fighter.fastshift(L2CValue::Ptr(special_n_cancel_main_loop_electric_boogaloo as *const () as _))
    }
    else {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) || (!fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
            return 0.into()
        }
        else {
            return 1.into()
        }
    }
}

unsafe extern "C" fn special_n_cancel_main_loop_electric_boogaloo(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        match VarModule::get_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE) {
            vars::diddy::SPECIAL_N_CANCEL_TYPE_ESCAPE => fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), true.into()),
            vars::diddy::SPECIAL_N_CANCEL_TYPE_ESCAPE_B => fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into()),
            vars::diddy::SPECIAL_N_CANCEL_TYPE_ESCAPE_F => fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into()),
            vars::diddy::SPECIAL_N_CANCEL_TYPE_GUARD => fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into()),
            vars::diddy::SPECIAL_N_CANCEL_TYPE_GROUND_JUMP => fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), false.into()),
            vars::diddy::SPECIAL_N_CANCEL_TYPE_NONE => fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into()),
            _ => {},
        }
    }
    else {
        match VarModule::get_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE) {
            vars::diddy::SPECIAL_N_CANCEL_TYPE_ESCAPE_AIR => fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into()),
            vars::diddy::SPECIAL_N_CANCEL_TYPE_NONE => fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into()),
            _ => {},
        }
    }
    return 1.into()
}

unsafe extern "C" fn special_n_cancel_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, ArticleOperationTarget(0));
    return 0.into()
}

unsafe extern "C" fn special_n_jump_cancel_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    // println!("pre");
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_n_jump_cancel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_cancel"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.main_shift(special_n_jump_cancel_main_loop)
}

unsafe extern "C" fn special_n_jump_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_AIR && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let cancel_type = VarModule::get_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE);
        if cancel_type == vars::diddy::SPECIAL_N_CANCEL_TYPE_JUMP_AERIAL {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    return 0.into()
}

unsafe extern "C" fn special_n_jump_cancel_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_GUN, ArticleOperationTarget(0));
    return 0.into()
}

// FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_SHOOT

pub unsafe extern "C" fn special_n_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    app::sv_kinetic_energy::enable(fighter.lua_state_agent);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    special_n_shoot_motion_helper(fighter);
    fighter.main_shift(special_n_shoot_main_loop)
}

unsafe extern "C" fn special_n_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_n_shoot_motion_helper(fighter);
        }
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (!fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
                return 0.into();
            }
            else {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
                return 0.into();
            }
        }
    }
    0.into()
}

unsafe extern "C" fn special_n_shoot_motion_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {    
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_shoot"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_shoot"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {    
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_shoot"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_shoot"), -1.0, 1.0, 0.0, false, false);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(Main, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE, special_n_charge_main);
    agent.status(Main, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_main);
    agent.status(Pre, statuses::diddy::SPECIAL_N_CANCEL, special_n_cancel_pre);
    agent.status(Main, statuses::diddy::SPECIAL_N_CANCEL, special_n_cancel_main);
    agent.status(End, statuses::diddy::SPECIAL_N_CANCEL, special_n_cancel_end);
    agent.status(Pre, statuses::diddy::SPECIAL_N_CANCEL_JUMP, special_n_jump_cancel_pre);
    agent.status(Main, statuses::diddy::SPECIAL_N_CANCEL_JUMP, special_n_jump_cancel_main);
    agent.status(End, statuses::diddy::SPECIAL_N_CANCEL_JUMP, special_n_jump_cancel_end);
}