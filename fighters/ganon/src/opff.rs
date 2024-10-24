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

unsafe fn wizards_foot_jump_refresh(boma: &mut BattleObjectModuleAccessor) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END].contains(&boma.status())
    && boma.is_situation(*SITUATION_KIND_AIR) {
        let jump_count_max = boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        if boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == jump_count_max {
            boma.set_int(jump_count_max - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW,
        *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // dtaunt_counter(boma, motion_kind, frame);
    wizards_foot_jump_refresh(boma);
    fastfall_specials(fighter);
}

pub extern "C" fn ganon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		ganon_frame(fighter)
    }
}

pub unsafe fn ganon_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, ganon_frame_wrapper);
}