use super::*;

// FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FAILURE

unsafe extern "C" fn special_s_failure_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cancel_frame = (FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_s_start"), false) - MotionModule::frame(fighter.module_accessor)) + WorkModule::get_param_int(fighter.module_accessor, hash40("landing_heavy_frame"), 0) as f32 + 5.0;
    if cancel_frame < 1.0 {
        VarModule::set_float(fighter.battle_object, vars::ridley::instance::SPECIAL_S_FAILURE_CANCEL_FRAME, 1.0);
    }
    else {
        VarModule::set_float(fighter.battle_object, vars::ridley::instance::SPECIAL_S_FAILURE_CANCEL_FRAME, cancel_frame);
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_failure"), 0.0, 1.0, false, 0.0, false, false);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)*WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("failure_speed_x_mul"));
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    fighter.sub_shift_status_main(L2CValue::Ptr(side_special_failure_main_loop as *const () as _))
}

pub unsafe fn side_special_failure_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    else if VarModule::get_float(fighter.battle_object, vars::ridley::instance::SPECIAL_S_FAILURE_CANCEL_FRAME) <= MotionModule::frame(fighter.module_accessor) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }

    false.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FAILURE, special_s_failure_main);
}