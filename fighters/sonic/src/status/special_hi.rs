use super::*;
use globals::*;
use smashline::*;




pub unsafe extern "C" fn pre_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
	StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_HI_COMP,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_HI_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_HI_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_HI_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP
        ) as u64,
        (
            *FIGHTER_STATUS_ATTR_START_TURN |
            *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_WARP
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn exec_special_hi_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.boma();
    let min_speed_y = 1.0;
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);

    if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_DOWN as u32) 
    && speed_y <= min_speed_y {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
    }
    return 0.into();
}


unsafe extern "C" fn exit_special_hi_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.boma();

    let landing_lag = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_landing_frame")) as f32;
    WorkModule::set_float(fighter.module_accessor, landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        WorkModule::off_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_HI_FALL);
    }
    0.into()
}

pub fn install() {
    smashline::Agent::new("sonic")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, pre_special_hi)
        .status(
            Exec,
            *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP,
            exec_special_hi_jump,
        )
        .status(
            Exit,
            *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP,
            exit_special_hi_jump,
        )
        .install();
}
