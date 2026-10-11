use super::*;

unsafe extern "C" fn cancel_step_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        0,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn cancel_step_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_WORK_INT_CANCEL_STATUS);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("cancel_step"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.75
    );

    fighter.main_shift(cancel_step_main_loop)
}

unsafe extern "C" fn cancel_step_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_WORK_INT_CANCEL_STATUS) == -1 {
        let mut status = -1;
        let cat4 = fighter.global_table[CMD_CAT4].get_i32();
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_1 != 0 {
            status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_3;
        }
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_3 != 0 {
            status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_1;
        }
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_7 != 0 {
            status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_6;
        }
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_9 != 0 {
            status = *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_2;
        }
        if status != -1 {
            WorkModule::set_int(fighter.module_accessor, status, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_WORK_INT_CANCEL_STATUS);
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let mut status = *FIGHTER_STATUS_KIND_SQUAT_WAIT;
        let mut clear_cmd = false;
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_WORK_INT_CANCEL_STATUS) != -1 {
            status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_LW_3_WORK_INT_CANCEL_STATUS);
            clear_cmd = true;
        }
        fighter.change_status(status.into(), clear_cmd.into());
    }
    0.into()
}

unsafe extern "C" fn cancel_step_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    JostleModule::set_overlap_rate_mul(fighter.module_accessor, 1.0);

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::demon::CANCEL_STEP, cancel_step_pre);
    agent.status(Main, statuses::demon::CANCEL_STEP, cancel_step_main);
    agent.status(End, statuses::demon::CANCEL_STEP, cancel_step_end);
}
