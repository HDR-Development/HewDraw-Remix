use super::*;

// FIGHTER_STATUS_KIND_ATTACK_LW4

pub unsafe extern "C" fn attack_lw4_main(agent: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    agent.attack_lw4_mtrans();
    WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
    if !StopModule::is_stop(agent.module_accessor) {
        agent.status_ThrowKirby_Uniq(L2CValue::Bool(false));
    }
    agent.global_table[SUB_STATUS].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_status_ThrowKirby_Uniq as *const () as _));
    agent.sub_shift_status_main(L2CValue::Ptr(attack_lw4_main_loop as *const () as _))
}

pub unsafe extern "C" fn attack_lw4_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(agent.module_accessor)
    && !WorkModule::is_enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND)
    && !MotionModule::is_end(agent.module_accessor) {
        agent.sub_status_uniq_process_ThrowKirby_execFixPos();
        return 0.into()
    }
    agent.status_AttackLw4_Main()
}

pub unsafe extern "C" fn attack_lw4_map_correction(agent: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(agent.module_accessor);
    let prev_frame = MotionModule::prev_frame(agent.module_accessor);
    let start_air_frame = 2.0;
    let fall_start_frame = 19.0;
    let fall_stop_frame = 20.0;
    let landing_frame = 21.0;

    if frame <= fall_start_frame {
        return 0.into()
    }
    if prev_frame < start_air_frame && frame >= start_air_frame {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    if agent.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        if prev_frame < fall_stop_frame && frame >= fall_stop_frame {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            agent.clear_lua_stack();
            lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -12.0);
            app::sv_kinetic_energy::set_speed(agent.lua_state_agent);
            agent.clear_lua_stack();
            lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            app::sv_kinetic_energy::set_accel_x_mul(agent.lua_state_agent);
            agent.clear_lua_stack();
            lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            app::sv_kinetic_energy::set_accel_x_add(agent.lua_state_agent);
            MotionModule::set_frame(agent.module_accessor, fall_stop_frame, true);
            MotionModule::set_rate(agent.module_accessor, 0.0);
        }
    }
    else {
        if frame < landing_frame {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
            MotionModule::set_frame(agent.module_accessor, landing_frame, true);
            MotionModule::set_rate(agent.module_accessor, 1.0);
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, attack_lw4_main);
    agent.status(MapCorrection, *FIGHTER_STATUS_KIND_ATTACK_LW4, attack_lw4_map_correction);
}