use super::*;

unsafe extern "C" fn special_lw_main_old(agent: &mut L2CFighterCommon) -> L2CValue {
    agent.change_status(statuses::krool::SPECIAL_LW_GUT.into(), true.into());
   
    0.into()
}

unsafe extern "C" fn special_lw_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(agent.module_accessor,
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
        agent.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_lw_main(agent: &mut L2CFighterCommon) -> L2CValue {
    agent.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(agent.module_accessor)
    || StatusModule::is_situation_changed(agent.module_accessor) {
        special_lw_change_motion(agent);
    }
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool()
        || agent.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(agent.module_accessor) {
        let status = if agent.is_situation(*SITUATION_KIND_GROUND) {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        agent.change_status(status.into(), false.into());
        return 0.into();
    }

    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 {
        GroundModule::correct(agent.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    if (6..29).contains(&agent.status_frame()) // gut charge logic
    && !ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && VarModule::is_flag(agent.object(), vars::krool::status::GUT_CHECK_CHARGED) {
        VarModule::off_flag(agent.battle_object, vars::krool::status::GUT_CHECK_CHARGED);
        MotionModule::set_frame_sync_anim_cmd(agent.module_accessor, 30.0, true, true, false);
    }
    if agent.status_frame() > 8  // Allows for jump cancel on frame 10 (35 in animation) if not charged
    && !VarModule::is_flag(agent.battle_object, vars::krool::status::GUT_CHECK_CHARGED)
    && !agent.is_in_hitlag() {
        agent.check_jump_cancel(false, false);
    }
    if VarModule::is_flag(agent.battle_object, vars::krool::status::GUT_CHECK_CHARGED)    // restore armor on full charge hit
        && AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT)
        && !AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        WorkModule::set_float(agent.module_accessor, 4.0, 0x4d);
        VarModule::set_float(agent.battle_object, vars::krool::instance::STORED_DAMAGE, 0.0);
    }

    return 0.into()
}

//FUN_710001bef0
unsafe extern "C" fn special_lw_change_motion(agent: &mut L2CFighterCommon) {
    if !agent.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_FIRST) {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(agent.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0);
        }
    } 
    else {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_FIRST) {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(agent.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0);
        }
    }

}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main_old);
    agent.status(Pre, statuses::krool::SPECIAL_LW_GUT, special_lw_pre);
    agent.status(Main, statuses::krool::SPECIAL_LW_GUT, special_lw_main);
}