use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_HI

pub unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.0
    );
    ret
}

// FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END

unsafe extern "C" fn special_hi_rush_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn special_hi_rush_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END)(fighter);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let fall_x_mul = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("param_special_hi"),
            hash40("fire_fall_x_mul")
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * fall_x_mul,
            0.0
        );
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_FALCO_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    }
    ret
}

// FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_BOUND

pub unsafe extern "C" fn special_hi_bound_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(End, fighter, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_BOUND)(fighter);

    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_frame"), 0);
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(Pre, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_rush_end_pre);
    agent.status(Main, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_rush_end_main);
    agent.status(End, *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_BOUND, special_hi_bound_end);
}