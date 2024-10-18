// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn bouncing_fish_transitions(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK) {
        if MotionModule::is_end(fighter.module_accessor)
        && fighter.is_situation(*SITUATION_KIND_AIR) {
            if !VarModule::is_flag(fighter.object(), vars::sheik::instance::SPECIAL_LW_HIT) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            }
        }
    }
    if fighter.is_status(*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN)
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.object(), vars::sheik::instance::SPECIAL_LW_HIT);
        if fighter.status_frame() > 14 {
            fighter.check_jump_cancel(false, false);
            fighter.check_airdodge_cancel();
        }
    }
}

unsafe fn nspecial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FT_SHEIK_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FT_SHEIK_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
            }
        }
    }
}

// Removes "variable landing lag" from Vanish reappearance
// always lands with flat special fall landing lag
unsafe fn vanish_landing_lag(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END)
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_prev_situation(*SITUATION_KIND_AIR)
    && fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// pub unsafe fn hitfall_aerials(fighter: &mut L2CFighterCommon, frame: f32) {
//     if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) {
//         // only allow the last hit of uair to be hitfalled
//         if fighter.is_motion(Hash40::new("attack_air_hi")) {
//             if frame >= 23.0 && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
//                 fighter.check_hitfall();
//             }
//         }
//         else {
//             fighter.check_hitfall();
//         }
//     }
// }

pub unsafe fn vanish_wall_ride(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    // Wall Ride momentum fixes
    if fighter.is_status(*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE) {
        let init_speed_x = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_X);
        let init_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_Y);
        if !GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_NONE as u32) {
            if !VarModule::is_flag(fighter.battle_object, vars::common::status::IS_TELEPORT_WALL_RIDE) {
                VarModule::on_flag(fighter.battle_object, vars::common::status::IS_TELEPORT_WALL_RIDE);
            }
        }
        if GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
            if init_speed_y > 0.0 {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, init_speed_y);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            }
        } else if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_TELEPORT_WALL_RIDE) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, init_speed_x, init_speed_y);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        }
    }
    else if fighter.is_status(*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END) {
        if GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
            if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
                let wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                KineticModule::mul_speed(fighter.module_accessor, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_HOLD,
        *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_FIRE,
        *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END,
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
    bouncing_fish_transitions(fighter);
    nspecial_cancels(fighter, boma, status_kind, situation_kind);
    vanish_landing_lag(fighter);
    fastfall_specials(fighter);
    vanish_wall_ride(fighter);
}

pub extern "C" fn sheik_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        sheik_frame(fighter)
    }
}

pub unsafe fn sheik_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, sheik_frame_wrapper);
}
