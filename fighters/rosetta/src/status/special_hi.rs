use super::*;
use globals::*;

// FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_END

pub unsafe extern "C" fn special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ROSETTA_SPECIAL_HI_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ROSETTA_SPECIAL_HI_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ROSETTA_SPECIAL_HI_END_FLOAT,
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

pub unsafe extern "C" fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_END)(fighter);

    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let end_accel_y_mul = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("param_special_hi"),
        hash40("end_accel_y_mul")
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -air_accel_y * end_accel_y_mul
    );

    let end_accel_x_mul = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("param_special_hi"),
        0x18689b3939
    );
    sv_kinetic_energy!(
        set_accel_x_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        end_accel_x_mul
    );

    ret
}

pub fn install() {
    smashline::Agent::new("rosetta")
        .status(
            Pre,
            *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_END,
            special_hi_end_pre,
        )
        .status(
            Main,
            *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_END,
            special_hi_end_main,
        )
        .install();
}
