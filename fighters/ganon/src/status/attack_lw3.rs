use super::*;

unsafe extern "C" fn attack_lw3_main(agent: &mut L2CFighterCommon) -> L2CValue {
    agent.status_AttackLw3_common();
    agent.main_shift(attack_lw3_main_loop)
}

unsafe extern "C" fn attack_lw3_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {    
    if !StatusModule::is_changing(agent.module_accessor) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        && agent.global_table[globals::CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
            MotionModule::change_motion(
                agent.module_accessor,
                Hash40::new("attack_lw3_2"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            agent.clear_lua_stack();
            sv_kinetic_energy::set_motion_energy_update_flag(agent.lua_state_agent);
            return 0.into();
        }
    }

    if CancelModule::is_enable_cancel(agent.module_accessor)
    && agent.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }

    if agent.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    let jump_attack_frame = WorkModule::get_int(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if 0 < jump_attack_frame {
        if !StopModule::is_stop(agent.module_accessor)
        && agent.sub_check_button_jump().get_bool() {
            let log = agent.status_attack();
            let info = log[0x10f40d7b92u64].get_i64();
            let mot = MotionModule::motion_kind(agent.module_accessor);
            MotionAnimcmdModule::call_script_single(
                agent.module_accessor,
                *FIGHTER_ANIMCMD_EXPRESSION,
                Hash40::new_raw(mot),
                -1
            );
            WorkModule::set_int64(agent.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            agent.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if 1 == jump_attack_frame {
        if !agent.global_table[globals::IS_STOPPING].get_bool()
        && WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
            let log = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            FighterStatusModuleImpl::reset_log_action_info(agent.module_accessor, log);
            WorkModule::set_int64(agent.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn attack_lw3_end(agent: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(
        set_speed,
        agent,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.0,
        0.0
    );
    agent.status_end_AttackLw3()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, attack_lw3_main);
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_LW3, attack_lw3_end);
}
