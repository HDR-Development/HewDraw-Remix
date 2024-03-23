use super::*;
use globals::*;

// FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END

pub unsafe extern "C" fn special_hi_rush_end_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        agent.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIT_SPECIAL_HI_RUSH_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIT_SPECIAL_HI_RUSH_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIT_SPECIAL_HI_RUSH_END_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        agent.module_accessor,
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

pub unsafe extern "C" fn special_hi_rush_end_main(agent: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        agent.module_accessor,
        Hash40::new("special_air_hi_end"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let air_speed_x_stable = WorkModule::get_param_float(agent.module_accessor, hash40("air_speed_x_stable"), 0);
    let x_max_mul = WorkModule::get_float(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    sv_kinetic_energy!(
        set_stable_speed,
        agent,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * x_max_mul,
        0.0
    );

    agent.select_cliff_hangdata_from_name("special_hi");
    
    agent.main_shift(special_hi_rush_end_main_loop)
}

unsafe extern "C" fn special_hi_rush_end_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if agent.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        agent.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        return 0.into();
    }
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
        agent.status(
            Pre,
            *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END,
            special_hi_rush_end_pre,
        )
        .status(
            Main,
            *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END,
            special_hi_rush_end_main,
        )
}
