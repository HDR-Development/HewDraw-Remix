use super::*;
use globals::*;

utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn dspecial_cancels(agent: &mut L2CFighterCommon) {
    if agent.is_status(*FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END)
    && agent.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_INT_CANCEL_STATUS);
    }
}

// Fixes bug where touching ground during upB puts you into special fall for 1f before landing
unsafe fn up_special_proper_landing(Agent: &mut L2CFighterCommon) {
    if Agent.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && Agent.is_situation(*SITUATION_KIND_GROUND)
    && WorkModule::is_flag(Agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL) {
        Agent.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        WorkModule::off_flag(Agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL);
    }
}

unsafe fn fastfall_specials(Agent: &mut L2CFighterCommon) {
    if !Agent.is_in_hitlag()
    && !StatusModule::is_changing(Agent.module_accessor)
    && Agent.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2_FALL,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END
        ]) 
    && Agent.is_situation(*SITUATION_KIND_AIR) {
        Agent.sub_air_check_dive();
        if Agent.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(Agent.module_accessor)) {
                Agent.clear_lua_stack();
                lua_args!(Agent, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(Agent.lua_state_agent);

                Agent.clear_lua_stack();
                lua_args!(Agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(Agent.lua_state_agent);
                
                Agent.clear_lua_stack();
                lua_args!(Agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(Agent.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, Agent.module_accessor);
            }
        }
    }
}

pub unsafe extern "C" fn cloud_frame_wrapper(Agent: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(Agent);

    dspecial_cancels(Agent);
    up_special_proper_landing(Agent);
    fastfall_specials(Agent);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, cloud_frame_wrapper);
}