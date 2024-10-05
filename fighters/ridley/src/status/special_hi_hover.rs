use super::*;

unsafe extern "C" fn special_hi_hover_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_hover_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_hi_hover"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let hover_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("hover_frame")) as f32;
    let rate = MotionModule::end_frame(fighter.module_accessor) / hover_frame;
    MotionModule::set_rate(fighter.module_accessor, rate);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);

    fighter.main_shift(special_hi_hover_main_loop)
}

unsafe extern "C" fn special_hi_hover_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI.into(), false.into());
    }

    0.into()
}

unsafe extern "C" fn special_hi_hover_fix_camera(fighter: &mut L2CFighterCommon) -> L2CValue {
    let end_frame = MotionModule::end_frame(fighter.module_accessor);

    if MotionModule::frame(fighter.module_accessor) + 0.0001 >= end_frame - 1.0
    && MotionModule::prev_frame(fighter.module_accessor) + 0.0001 < end_frame - 1.0 {
        if !VarModule::is_flag(fighter.battle_object, vars::ridley::status::SPECIAL_HI_HOVER_DECIDE_STICK) {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            VarModule::set_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_HOVER_DECIDE_STICK_X, stick_x);
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            VarModule::set_float(fighter.battle_object, vars::ridley::status::SPECIAL_HI_HOVER_DECIDE_STICK_Y, stick_y);

            if stick_x.abs() > 0.125 {
                PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }

            VarModule::on_flag(fighter.battle_object, vars::ridley::status::SPECIAL_HI_HOVER_DECIDE_STICK);
        }
    }

    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_SLOPE_SLOPE, MA_MSC_CMD_SLOEP_SLOPE_KIND_NONE);
    sv_module_access::slope(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER, special_hi_hover_pre);
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER, special_hi_hover_main);
    agent.status(FixCamera, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER, special_hi_hover_fix_camera);
}