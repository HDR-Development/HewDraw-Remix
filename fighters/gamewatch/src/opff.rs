// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn ff_chef_land_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) {
        if boma.is_prev_situation(*SITUATION_KIND_AIR) && boma.is_situation(*SITUATION_KIND_GROUND) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if boma.is_situation(*SITUATION_KIND_AIR) {
            if boma.is_cat_flag(Cat2::FallJump) && ControlModule::get_stick_y(boma) < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
        if StatusModule::is_changing(boma) {
            let nspec_halt = Vector3f{x: 0.9, y: 1.0, z: 1.0};
            KineticModule::mul_speed(boma, &nspec_halt, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
}

// Game & Watch Parachute Double Jump
unsafe fn parachute_dj(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[*FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_CLOSE]) {
        boma.check_jump_cancel(false);
    }
}

// Jump cancel Judge 4
unsafe fn jc_judge_four(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion_one_of(&[Hash40::new("special_s_4"), Hash40::new("special_air_s_4")]) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag() {
            boma.check_jump_cancel(false);
        }
    }
}

// down throw mirror
unsafe fn dthrow_reverse(boma: & mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("throw_lw")) {
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &Vector3f{x: 0.0, y: 180.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    ff_chef_land_cancel(boma);
    parachute_dj(boma);
    jc_judge_four(boma);
    dthrow_reverse(boma);
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