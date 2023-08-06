// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn chef_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if StatusModule::is_changing(boma) {
            let nspec_halt = Vector3f{x: 0.9, y: 1.0, z: 1.0};
            KineticModule::mul_speed(boma, &nspec_halt, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
}

// Game & Watch Parachute Double Jump
unsafe fn parachute_dj(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if [*FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_FALL,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_CLOSE].contains(&status_kind) {
        boma.check_jump_cancel(false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_CATCH,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_SHOOT,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_REFLECT,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT_START
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

    chef_land_cancel(boma, status_kind, situation_kind, cat[1], stick_y);
    parachute_dj(boma, status_kind, situation_kind, cat[0]);
    //jc_oil_panic_reflect(boma, status_kind, situation_kind); 
    jc_judge_four(boma, motion_kind, situation_kind);
    dthrow_reverse(boma, motion_kind);
    fastfall_specials(fighter);

    // Frame Data
    frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if frame <= 19.0 {
            MotionModule::set_rate(boma, 2.0);
        }
        if frame > 19.0 {
            MotionModule::set_rate(boma, 1.0);
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_GAMEWATCH )]
pub fn gamewatch_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		gamewatch_frame(fighter)
    }
}

pub unsafe fn gamewatch_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

// Jump cancel Judge 4
unsafe fn jc_judge_four(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, situation_kind: i32) {
    if motion_kind == hash40("special_s_4") || motion_kind == hash40("special_air_s_4") {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag() {
            boma.check_jump_cancel(false);
        }
    }
}

// down throw mirror
unsafe fn dthrow_reverse(boma: & mut BattleObjectModuleAccessor, motion_kind: u64) {
    if boma.is_motion(Hash40::new("throw_lw")) {
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &Vector3f{x: 0.0, y: 180.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
}

// Jump cancel bucket reflect
// unsafe fn jc_oil_panic_reflect(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
//     if status_kind == *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_REFLECT {
//         if boma.is_input_jump() && !boma.is_in_hitlag() {
//             if situation_kind == *SITUATION_KIND_GROUND {
//                 StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
//             }
//             else if situation_kind == *SITUATION_KIND_AIR {
//                 if boma.get_num_used_jumps() < boma.get_jump_count_max() {
//                     StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
//                 }
//             }
//         }
//     }
// }

#[smashline::weapon_frame_callback(main)]
pub fn box_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.kind() != WEAPON_KIND_GAMEWATCH_BOMB {
            return
        }
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let gnw = utils::util::get_battle_object_from_id(owner_id);
        let gnw_boma = &mut *(*gnw).module_accessor;
        if gnw_boma.is_motion(Hash40::new("attack_air_f")) {
            let gnw_fighter = utils::util::get_fighter_common_from_accessor(gnw_boma);
            if let Some(info) = FrameInfo::update_and_get(gnw_fighter) {
                if info.frame < 11.0 {
                    ModelModule::set_scale(weapon.module_accessor, 0.75);
                }
                else {
                    ModelModule::set_scale(weapon.module_accessor, 1.1);
                }
            }
        }
    }
}