use super::*;
use globals::*;


// FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END

#[status_script(agent = "pitb", status = FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn special_hi_rush_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
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

#[status_script(agent = "pitb", status = FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_rush_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

pub fn install() {
    install_status_scripts!(
        special_hi_rush_end_pre,
        special_hi_rush_end_main
    );
}