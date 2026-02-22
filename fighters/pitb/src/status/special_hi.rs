use super::*;


// FIGHTER_STATUS_KIND_SPECIAL_HI

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("pitb_fly_miracle_start"), true, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("pitb_ikaros_wing_flare"), true, true);
    }

    return 0.into();
}

// FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH

unsafe extern "C" fn special_hi_rush_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
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

    return 0.into();
}

unsafe extern "C" fn special_hi_rush_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Init, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH)(fighter);

    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let mut rad = 90.0_f32.to_radians();
    let rush_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_speed"));
    let lr = PostureModule::lr(fighter.module_accessor);

    let stick_added = stick_x.abs() + stick_y.abs();

    if stick_added >= 0.5 {
        let atan = (stick_y).atan2(stick_x * lr);
        rad = atan;
    }
    let rush_angle = fighter.get_param_float("param_special_hi", "rush_angle");
    rad = rad.clamp((90.0 - 0.5 * rush_angle).to_radians(), (90.0 + 0.5 * rush_angle).to_radians());
    // dbg!(rad);
    // dbg!(rad.to_degrees());
    let rush_speed_x = rush_speed * rad.sin() * lr;
    let rush_speed_y = rush_speed * rad.cos();

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, rush_speed_x, rush_speed_y);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    fighter.set_joint_rotate("rot", Vector3f{x: rad.to_degrees(), y: 0.0, z: 0.0});

    WorkModule::set_float(fighter.module_accessor, rad, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SDIR);
    WorkModule::set_float(fighter.module_accessor, rad, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SDIR_INIT);

    ret
}

unsafe extern "C" fn special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH)(fighter);

    let rush_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.rush_frame") as f32;
    let rate = MotionModule::end_frame(fighter.module_accessor) / rush_frame;
    MotionModule::set_rate(fighter.module_accessor, rate);

    let dir = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_SDIR);
    let mut rush_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_end_speed_rate_x"));
    let mut rush_brake_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_end_speed_rate_y"));

    rush_brake_x *= dir.cos().abs();
    rush_brake_y *= dir.sin().abs();

    WorkModule::set_float(fighter.module_accessor, rush_brake_x, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_BRAKE_X);
    WorkModule::set_float(fighter.module_accessor, rush_brake_y, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLOAT_BRAKE_Y);

    ret
}

// FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END

pub unsafe extern "C" fn special_hi_rush_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIT_SPECIAL_HI_RUSH_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIT_SPECIAL_HI_RUSH_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIT_SPECIAL_HI_RUSH_END_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    
    0.into()
}

pub unsafe extern "C" fn special_hi_rush_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_hi_end"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let x_max_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * x_max_mul,
        0.0
    );

    fighter.select_cliff_hangdata_from_name("special_hi");
    
    fighter.main_shift(special_hi_rush_end_main_loop)
}

unsafe extern "C" fn special_hi_rush_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end);

    agent.status(Pre, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_pre);
    agent.status(Init, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_init);
    agent.status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_main);

    agent.status(Pre, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_rush_end_pre);
    agent.status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_rush_end_main);
}