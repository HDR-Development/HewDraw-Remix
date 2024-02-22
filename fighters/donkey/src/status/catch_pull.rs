use super::*;

unsafe extern "C" fn catch_pull_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        *SITUATION_KIND_NONE
    }
    else {
        *SITUATION_KIND_GROUND
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(situation),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        true,
        false,
        0,
        (
            *FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT |
            *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE
        ) as u32,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn catch_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION
        );
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
    }
    fighter.status_CatchPull_common(hash40("catch_wait").into());
    fighter.main_shift(catch_pull_main_loop)
}

unsafe extern "C" fn catch_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_LW
    && CancelModule::is_enable_cancel(fighter.module_accessor) {
        let status = fighter.global_table[0x45].get_i32();
        fighter.change_status(status.into(), false.into());
        1.into()
    }
    else {
        fighter.status_CatchPull_Main()
    }
}

pub fn install() {
    smashline::Agent::new("donkey")
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_PULL, catch_pull_main)
        .install();
}
