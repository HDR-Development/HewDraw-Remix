use super::*;

// FIGHTER_STATUS_KIND_ATTACK_S3

unsafe extern "C" fn attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS3Common();
    fighter.main_shift(attack_s3_main_loop)
}
// cheated by editing wuboy's dtilt rewrite (idk how to read dat_xxx in ghidra)
unsafe extern "C" fn attack_s3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        fighter.attack_s3_mtrans();
        check_stage(fighter);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        if VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::ATTACK_INVALID_COMBO_INPUT) {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::ATTACK_S3_COUNT, 0);
        }
        return 1.into();
    }
    if fighter.global_table[globals::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into())
    }
    let jump_attack_frame = fighter.get_int(*FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if 0 < jump_attack_frame {
        if !StopModule::is_stop(fighter.module_accessor)
        && fighter.sub_check_button_jump().get_bool() {
            let log = fighter.status_attack();
            let info = log[0x10f40d7b92u64].get_i64();
            let mot = MotionModule::motion_kind(fighter.module_accessor);
            MotionAnimcmdModule::call_script_single(
                fighter.module_accessor,
                *FIGHTER_ANIMCMD_EXPRESSION,
                Hash40::new_raw(mot),
                -1
            );
            fighter.set_int64(info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if 1 == jump_attack_frame {
        if !fighter.global_table[globals::IS_STOPPING].get_bool()
        && fighter.get_int64(*FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
            let log = fighter.get_int64(*FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
            fighter.set_int64(0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    // combo input
    if check_input(fighter) {
        if fighter.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        && (fighter.is_cat_flag(Cat1::AttackS3 | Cat1::AttackN)
        || (!fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) 
        && fighter.is_button_on(Buttons::Attack)
        && !fighter.is_button_trigger(Buttons::Attack)
        && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::ATTACK_INVALID_COMBO_INPUT)))
        {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S3.into(), true.into())
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn check_input(fighter: &mut L2CFighterCommon) -> bool {
    let special_stick_y = fighter.get_param_float("common", "special_stick_y");
    let lr = fighter.lr();
    // f1 of input try to filter non jab/ftilt inputs
    if fighter.is_button_trigger(Buttons::Attack) {
        if fighter.is_stick_backward()
        || fighter.stick_y().abs() > special_stick_y {
            fighter.clear_commands(Cat1::AttackN); 
            fighter.clear_commands(Cat1::AttackS3);
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::ATTACK_INVALID_COMBO_INPUT);
            return false.into();
        }
        VarModule::off_flag(fighter.battle_object, vars::bayonetta::status::ATTACK_INVALID_COMBO_INPUT);
    }
    // hold input restriction
    if fighter.is_stick_backward()
    || fighter.left_stick_y().abs() > special_stick_y
    || fighter.right_stick_y().abs() > special_stick_y {
        VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::ATTACK_INVALID_COMBO_INPUT);
        return false.into();
    }
    // again but also filter grab
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH)
    || fighter.global_table[CMD_CAT1].get_i32() & (
        //*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI |
        //*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 |
        //*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 |
        *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH
    ) == 1 {
        return false.into();
    }
    true.into()
}

unsafe extern "C" fn check_stage(fighter: &mut L2CFighterCommon) {
    let prev_status_0 = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    let mut stage = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::ATTACK_S3_COUNT);
    let mut motion = Hash40::new("attack_s3_s");
    // fuck combo module
    if ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&prev_status_0) {
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::ATTACK_S3_COUNT, 0);
        stage = 0;
    }
    match stage {
        0 => VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::ATTACK_S3_COUNT, 1),
        1 => {VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::ATTACK_S3_COUNT, 2);
        motion = Hash40::new("attack_s3_s2");},
        _ => {VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::ATTACK_S3_COUNT, 0);
        motion = Hash40::new("attack_s3_s3");},
    };
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    return;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_S3, attack_s3_main);
}