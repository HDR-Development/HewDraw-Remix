use super::*;

unsafe extern "C" fn dolly_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_special_lw_main_inner(fighter)
}

unsafe extern "C" fn dolly_special_lw_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND);
    dolly_special_lw_main_inner(fighter)
}

unsafe extern "C" fn dolly_special_lw_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::set_int(fighter.module_accessor, situation, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_INT_START_SITUATION);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_DOLLY_STRENGTH_S, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);
    WorkModule::set_customize_no(fighter.module_accessor, 1, 3);
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_lw").into());

    // cap start speed
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * lr;
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_speed_x_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.start_speed_x_max");
    SET_SPEED_EX(fighter, speed_x.min(start_speed_x_max) * lr, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    dolly_special_lw_mot_helper(fighter, true.into());
    let additions = if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_07
    }
    else {
        let command_power_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("command_power_mul"));
        AttackModule::set_power_mul_status(fighter.module_accessor, command_power_mul);
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_08
    };
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, additions - 1);

    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_special_lw_mot_helper(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let mot;
    let reset;
    let correct;
    if situation == *SITUATION_KIND_GROUND {
        mot = Hash40::new("special_lw_start");
        reset = ENERGY_MOTION_RESET_TYPE_GROUND_TRANS;
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
    }
    else {
        mot = Hash40::new("special_air_lw_start");
        reset = ENERGY_MOTION_RESET_TYPE_AIR_TRANS;
        correct = *GROUND_CORRECT_KIND_AIR;
    }
    if param_1.get_bool() {
        MotionModule::change_motion(
            fighter.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        reset,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
}

unsafe extern "C" fn dolly_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_JUMP)
        && !StatusModule::is_changing(fighter.module_accessor)
        && StatusModule::is_situation_changed(fighter.module_accessor) {
            dolly_special_lw_mot_helper(fighter, false.into());
            fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_lw").into());
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_JUMP)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_GUARD_TRIGGER != 0 {
        VarModule::on_flag(fighter.battle_object, vars::dolly::status::SPECIAL_LW_BREAK);
    }

    if !fighter.global_table[IS_STOPPING].get_bool()
    && MotionModule::is_end(fighter.module_accessor) {
        if VarModule::is_flag(fighter.battle_object, vars::dolly::status::SPECIAL_LW_BREAK)
        && MeterModule::level(fighter.battle_object) >= 1 {
            VarModule::off_flag(fighter.battle_object, vars::dolly::status::SPECIAL_LW_BREAK);
            fighter.change_status(statuses::dolly::SPECIAL_LW_BREAKING.into(), true.into());
            return 0.into();
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_JUMP) {
            let frame = MotionModule::frame(fighter.module_accessor);
            WorkModule::set_int(fighter.module_accessor, frame as i32, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_INT_START_FRAME);
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK.into(), false.into());
            return 0.into();
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_JUMP);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_DOLLY_SPECIAL_LW_JUMP);
        let strength = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);
        let mot = if strength == *FIGHTER_DOLLY_STRENGTH_W {
            Hash40::new("special_lw_w")
        }
        else {
            Hash40::new("special_lw")
        };
        MotionModule::change_motion(
            fighter.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    0.into()
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING

pub unsafe extern "C" fn special_lw_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        fighter.sub_pre_landing_kinetic_type().into(),
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        0,
        0
    );
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, dolly_special_lw_main);
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, dolly_special_lw_command_main);

    agent.status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING, special_lw_landing_pre);
}