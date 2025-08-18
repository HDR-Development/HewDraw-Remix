use super::*;

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let move_time = fighter.get_param_int("param_special_hi", "special_hi_move_time");
    fighter.set_int(move_time, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_WORK_INT_MOVE_XLU);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);

    fighter.main_shift(special_hi_main_loop)
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            if fighter.is_flag(*FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_start"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PALUTENA_SPECIAL_HI_AIR);
            if fighter.is_flag(*FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi_start"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
            }
        }
        // set ground start
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            VarModule::on_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_TELEPORT_GROUND_START);
        }
        else {
            VarModule::off_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_TELEPORT_GROUND_START);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2.into(), false.into());
    }
    // double jump leniency
    if fighter.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL)
    && VarModule::is_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_JUMP_REFRESH) {
        // Grants 1 extra jump if all jumps used up
        if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        VarModule::off_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_JUMP_REFRESH);
    }

    return 0.into();
}

unsafe extern "C" fn special_hi2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK as i32
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP) as u64,
        (*FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    VisibilityModule::set_whole(fighter.module_accessor, false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
    fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    let cliff_check = fighter.get_int(*FIGHTER_PALUTENA_STATUS_SPECIAL_HI_WORK_INT_CLIFF_CHECK);
    fighter.sub_fighter_cliff_check(cliff_check.into());
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_special_hi2(fighter, false.into());
    }

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_hi2 as *const () as _));
    fighter.main_shift(special_hi2_main_loop)
}

unsafe extern "C" fn sub_special_hi2(fighter: &mut L2CFighterCommon, param: L2CValue) -> L2CValue {
    if param.get_bool() {
        fighter.inc_int(*FIGHTER_PALUTENA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        let frame = fighter.get_int(*FIGHTER_PALUTENA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        if frame >= 2 {
            fighter.on_flag(*FIGHTER_PALUTENA_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND);
        }
    }
    else {
        let frame = fighter.get_int(*FIGHTER_PALUTENA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        let move_xlu = fighter.get_param_int("param_special_hi", "special_hi_move_xlu");
        if frame == move_xlu {
            GroundModule::set_passable_check(fighter.module_accessor, false);
        }
        let cliff_check = fighter.get_param_int("param_special_hi", "special_hi_move_cliff_check");
        if frame == cliff_check {
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_hi2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    let frame = fighter.get_int(*FIGHTER_PALUTENA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    let move_time = fighter.get_param_int("param_special_hi", "special_hi_move_time");
    if frame >= move_time {
        fighter.change_status(FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3.into(), false.into());
        return 0.into();
    }
    else {
        if StatusModule::is_changing(fighter.module_accessor)
        || StatusModule::is_situation_changed(fighter.module_accessor) {
            fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
        }
    }
    if StatusModule::is_changing(fighter.module_accessor) {
        if !VarModule::is_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_TELEPORT_GROUND_START)
        && !(fighter.get_num_used_jumps() < fighter.get_jump_count_max()) {
            VarModule::on_flag(fighter.battle_object, vars::palutena::status::SPECIAL_HI_TELEPORT_AIR_START);
        }
    }
    // wallride behavior
    let init_speed_x = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_X);
    let init_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_Y);
    if GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
        || (!GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_ID_NONE as u32) && init_speed_x.abs() <= 0.01) {
        if !VarModule::is_flag(fighter.battle_object, vars::common::status::IS_TELEPORT_WALL_RIDE) {
            VarModule::on_flag(fighter.battle_object, vars::common::status::IS_TELEPORT_WALL_RIDE);
        }
    }
    if GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
        if init_speed_y > 0.0 {
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, init_speed_y);
        }
    } else if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_TELEPORT_WALL_RIDE) {
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, init_speed_x, init_speed_y);
    }
    // Allow turnaround based on stick position when reappearing
    if MotionModule::is_end(fighter.module_accessor) {
        PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    // Prevent actionability toggle when touching ground during the travel
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    && !VarModule::is_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_ENABLE_FREEFALL)
    && VarModule::is_flag(fighter.battle_object, vars::palutena::status::SPECIAL_HI_TELEPORT_AIR_START) {
        VarModule::on_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_ENABLE_FREEFALL);
    }

    return 0.into();
}

unsafe extern "C" fn special_hi3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK as i32
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);

    agent.status(Pre, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2, special_hi2_pre);
    agent.status(Main, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2, special_hi2_main);

    agent.status(Pre, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3, special_hi3_pre);
}