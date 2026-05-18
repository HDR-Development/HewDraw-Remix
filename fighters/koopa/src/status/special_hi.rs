use super::*;

unsafe extern "C" fn special_hi_a_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_SOUND as i32
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into()
}

pub unsafe extern "C" fn special_hi_a_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG_MOT_RESTART) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0, false, false);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    fighter.set_float(0.0, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_RESTART_FRAME);

    fighter.main_shift(special_hi_a_main_loop)
}

pub unsafe extern "C" fn special_hi_a_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        return 0.into();
    }
    if fighter.status_frame() >= 1 {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            let jump_restart_start_frame = fighter.get_param_float("param_special_hi", "special_hi_jump_restart_start_frame");
            let jump_restart_end_frame = fighter.get_param_float("param_special_hi", "special_hi_jump_restart_end_frame");
            if (jump_restart_start_frame..=jump_restart_end_frame).contains(&(fighter.status_frame() as f32)) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    let restart_frame = fighter.get_float(*FIGHTER_KOOPA_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_RESTART_FRAME);
                    let jump_restart_prohibition_frame = fighter.get_param_float("param_special_hi", "special_hi_jump_restart_prohibition_frame");
                    let frame = fighter.status_frame() as f32;
                    if jump_restart_prohibition_frame < frame - restart_frame {
                        fighter.set_float(frame, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_RESTART_FRAME);
                        let jump_speed_y = fighter.get_param_float("param_special_hi", "special_hi_jump_speed_y");
                        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, jump_speed_y);
                    }
                }
            }
        }
    }
    
    return 0.into();
}

pub unsafe extern "C" fn special_hi_a_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_prev_status(*FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1SET) {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_WORK_INT_F);
            return 0.into()
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1SET);
        }
    }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_WORK_INT_F);
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, special_hi_a_pre);
    agent.status(Main, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, special_hi_a_main);
    agent.status(Exec, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, special_hi_a_exec);
}