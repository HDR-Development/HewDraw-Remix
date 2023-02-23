// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn ff_chef_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
        if MotionModule::frame(boma) <= 1.0 {
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

// Game & Watch Fair cake box position readjustment
unsafe fn fair_repositioning(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && motion_kind == hash40("attack_air_f") {
        if frame < 9.0 || (frame >= 9.0 && frame < 10.0 && boma.is_in_hitlag()) {
            ModelModule::set_joint_translate(boma, Hash40::new("havel"), &Vector3f{ x: -3.5, y: 6.5, z: 0.0 }, false, false);
            ModelModule::set_joint_rotate(boma, Hash40::new("havel"), &Vector3f{ x: -15.0, y: 0.0, z: 0.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(boma, Hash40::new("shoulderl"), &Vector3f{ x: 0.0, y: 0.0, z: -40.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(boma, Hash40::new("shoulderr"), &Vector3f{ x: 0.0, y: 0.0, z: 40.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else {
            // -2.0/9.0/0.0; in relation to the havel bone, x is down right, y is down left
            ModelModule::set_joint_translate(boma, Hash40::new("havel"), &Vector3f{x:-2.5, y: 9.0, z: 0.0 }, false, false);
            ModelModule::set_joint_rotate(boma, Hash40::new("havel"), &Vector3f{ x: 5.0, y: 0.0, z: 0.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    ff_chef_land_cancel(boma, status_kind, situation_kind, cat[1], stick_y);
    parachute_dj(boma, status_kind, situation_kind, cat[0]);
    fair_repositioning(boma, status_kind, motion_kind, frame);
    //jc_oil_panic_reflect(boma, status_kind, situation_kind); 
    jc_judge_four(boma, motion_kind, situation_kind);
    dthrow_reverse(boma, motion_kind);

    // Frame Data
    frame_data(boma, status_kind, motion_kind, frame);
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if frame <= 18.0 {
            MotionModule::set_rate(boma, 2.0);
        }
        if frame > 18.0 {
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
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
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

#[smashline::weapon_frame_callback]
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
                if info.frame < 10.0 {
                    ModelModule::set_scale(weapon.module_accessor, 0.75);
                }
                else {
                    ModelModule::set_scale(weapon.module_accessor, 1.1);
                }
            }
        }
    }
}