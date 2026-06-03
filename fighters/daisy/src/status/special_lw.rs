use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_LW

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.set_status_kind_interrupt(statuses::daisy::SPECIAL_LW_THROW);
        return 1.into()
    }
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.set_status_kind_interrupt(statuses::daisy::SPECIAL_AIR_LW);
        return 1.into()
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND as u32,
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // turn around
    let turn_stick_x = fighter.get_param_float("common", "turn_stick_x") * fighter.lr();
    let direc = if fighter.left_stick_x() <= turn_stick_x {-1.0} else {1.0};
    PostureModule::set_lr(fighter.module_accessor, direc);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_ITEM_NO_COUNT);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_04 as i32 - 1 );
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _));
    0.into()
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
            fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_FALL);
            if !fighter.is_situation(*SITUATION_KIND_GROUND) {
                EFFECT_DETACH_KIND(fighter, Hash40::new("daisy_hikkonuki"), -1);
                // cancel into air pull if visually coherent, most edge cancels happen first _ frames anyway
                if MotionModule::frame(fighter.module_accessor) <= 10.0 {
                    fighter.change_status(statuses::daisy::SPECIAL_AIR_LW.into(), false.into());
                    return 1.into();
                }
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        let item_kind = fighter.get_int(*FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND);
        if item_kind != *ITEM_KIND_BEAMSWORD {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2508b59a2b), FIGHTER_ITEM_HOLD_KIND_HAVE);//tells her to use large item run ig, pull anim uses grip instead to position the turnip
        } else {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2508b59a2b), FIGHTER_ITEM_HOLD_KIND_GRIP);//medium item anim
        }
        if fighter.global_table[globals::STATUS_KIND] != FIGHTER_STATUS_KIND_FALL
        && fighter.global_table[globals::STATUS_KIND] != statuses::daisy::SPECIAL_AIR_LW
        && MotionModule::frame(fighter.module_accessor) <= 35.0 {
            ItemModule::drop_item(fighter.module_accessor, 90.0, 0.0, 0);
        }
    }
    0.into()
}

// statuses::daisy::SPECIAL_AIR_LW

unsafe extern "C" fn special_air_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let turn = if !ItemModule::is_have_item(fighter.module_accessor, 0) {*FIGHTER_STATUS_ATTR_START_TURN as u32} else {0};
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        turn,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_air_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    if speed_y < -air_speed_y_stable {
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0, -air_speed_y_stable);
    }
    // should work as both ledge cancel and standalone?
    if !ItemModule::is_have_item(fighter.module_accessor, 0) {
        let start_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.air_start_x_mul");
        sv_kinetic_energy!(mul_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_x, 0.0);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
    }
    fighter.main_shift(special_air_lw_main_loop)
}

unsafe extern "C" fn special_air_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        // empty pull land cancels (won't transition cleanly into any anim)
        if fighter.is_motion(Hash40::new("special_air_lw")) {
            fighter.check_land_cancel(None);
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn special_air_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

// statuses::daisy::SPECIAL_LW_THROW

unsafe extern "C" fn special_lw_throw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_THROW_ITEM as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_ITEM_SHOOT as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_lw_throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_motion_by_situation("special_lw_throw", "special_air_lw_throw", 0.0, 1.0, false, 0.0, false, false);
    fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
    fighter.main_shift(special_lw_throw_main_loop)
}

unsafe extern "C" fn special_lw_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_dive();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_motion_inherit_frame_by_situation("special_lw_throw", "special_air_lw_throw", -1.0, 1.0, 0.0, false, false);
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
    }
    0.into()
}

unsafe extern "C" fn special_lw_throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end);

    agent.status(Pre, statuses::daisy::SPECIAL_AIR_LW, special_air_lw_pre);
    agent.status(Main, statuses::daisy::SPECIAL_AIR_LW, special_air_lw_main);
    agent.status(End, statuses::daisy::SPECIAL_AIR_LW, special_air_lw_end);

    agent.status(Pre, statuses::daisy::SPECIAL_LW_THROW, special_lw_throw_pre);
    agent.status(Main, statuses::daisy::SPECIAL_LW_THROW, special_lw_throw_main);
    agent.status(End, statuses::daisy::SPECIAL_LW_THROW, special_lw_throw_end);
}
