// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// Dtaunt Counter
// unsafe fn dtaunt_counter(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
//     if [hash40("appeal_lw_l"), hash40("appeal_lw_r")].contains(&motion_kind)
//         && frame >= 29.0 && frame <= 59.0 {
//         if FighterStopModuleImpl::is_damage_stop(boma) {
//             if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
//                 DamageModule::add_damage(boma, 100.0, 0);
//                 WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
//                 StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
//             }
//         }
//     }
// }

// Repeated Warlock Punch turnaround
// unsafe fn repeated_warlock_punch_turnaround(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
//     if status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN {
//         if frame > 30.0 && frame < 45.0 {
//             if stick_x * facing < 0.0 {
//                 StatusModule::change_status_request_from_script(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, true);
//             }
//         }
//     }
// }

unsafe fn fastfall_specials(agent: &mut L2CFighterCommon) {
    if !agent.is_in_hitlag()
    && !StatusModule::is_changing(agent.module_accessor)
    && agent.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END
        ]) 
    && agent.is_situation(*SITUATION_KIND_AIR) {
        agent.sub_air_check_dive();
        if agent.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(agent.module_accessor)) {
                agent.clear_lua_stack();
                lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(agent.lua_state_agent);

                agent.clear_lua_stack();
                lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(agent.lua_state_agent);
                
                agent.clear_lua_stack();
                lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(agent.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, agent.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(agent: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // dtaunt_counter(boma, motion_kind, frame);
    // repeated_warlock_punch_turnaround(boma, status_kind, stick_x, facing, frame);
    fastfall_specials(agent);
}

pub extern "C" fn ganon_frame_wrapper(agent: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(agent);
		ganon_frame(agent)
    }
}

pub unsafe fn ganon_frame(agent: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(agent) {
        moveset(agent, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
pub fn install(agent: &mut Agent) {
    agent.on_line(Main, ganon_frame_wrapper);
}
