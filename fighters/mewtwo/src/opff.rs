// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn actionable_teleport_air(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
    && boma.status_frame() == 1 {
        VarModule::off_flag(boma.object(), vars::mewtwo::instance::GROUNDED_TELEPORT);
        if situation_kind == *SITUATION_KIND_GROUND {
            VarModule::on_flag(boma.object(), vars::mewtwo::instance::GROUNDED_TELEPORT);
        }
    }
    // Allows M2 to turnaround based on stick position when reappearing
    if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2 && MotionModule::is_end(boma) {
        if boma.get_num_used_jumps() < boma.get_jump_count_max() {
            PostureModule::set_stick_lr(boma, 0.0);
            PostureModule::update_rot_y_lr(boma);
        }
    }
    // Actionability when double jump isn't burned
    if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 && situation_kind == *SITUATION_KIND_AIR && frame > 9.0 {
        if boma.get_num_used_jumps() < boma.get_jump_count_max() {
            VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
            CancelModule::enable_cancel(boma);
            // Consume double jump, except when Teleport is initiated on ground
            if !VarModule::is_flag(boma.object(), vars::mewtwo::instance::GROUNDED_TELEPORT) {
                fighter.set_int(2, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
        }
    }
     //takes away float after 5 frames of jump
    if boma.get_num_used_jumps() == 2 && (fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) == VarModule::get_int(boma.object(), vars::common::instance::FLOAT_DURATION)) {
        if !(status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL && boma.status_frame() <= 5) {
            fighter.set_int(0, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
        }
    }
}

unsafe fn dj_upB_jump_refresh(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) {
        // If first 3 frames of dj
        if fighter.status_frame() <= 3 {
            VarModule::on_flag(fighter.battle_object, vars::mewtwo::instance::UP_SPECIAL_JUMP_REFRESH);
        }
        else {
            VarModule::off_flag(fighter.battle_object, vars::mewtwo::instance::UP_SPECIAL_JUMP_REFRESH);
        }
    }
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL)
    && VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::UP_SPECIAL_JUMP_REFRESH) {
        // Grants 1 extra jump if all jumps used up
        if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        VarModule::off_flag(fighter.battle_object, vars::mewtwo::instance::UP_SPECIAL_JUMP_REFRESH);
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            if MotionModule::is_end(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
            }
        }
    }
}

unsafe fn unfloat_confusion(fighter: &mut smash::lua2cpp::L2CFighterCommon, status_kind: i32) {
    if status_kind == FIGHTER_STATUS_KIND_SPECIAL_S
    && StatusModule::is_changing(fighter.module_accessor)
    && fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) > 0
    && fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) < VarModule::get_int(fighter.battle_object, vars::common::instance::FLOAT_DURATION) {
        fighter.on_flag(*FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_BUOYANCY);
    }
}

pub unsafe fn mewtwo_teleport_wall_ride(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, id: usize) {
    // Wall Ride momentum fixes
    let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
    let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);

    if boma.is_status(*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2) {
        let touch_normal_y_left = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
        let touch_normal_y_right = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
        if (touch_right && touch_normal_y_right != 0.0)
        || (touch_left && touch_normal_y_left != 0.0)
        {
            let init_speed_y = VarModule::get_float(boma.object(), vars::common::status::TELEPORT_INITIAL_SPEED_Y);

            if init_speed_y > 0.0 {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, init_speed_y);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            }
        }
    }
    else if boma.is_status(*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3) {
        if touch_right || touch_left {
            if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
                let wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_STOP);
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
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX,
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_S_THROW,
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    actionable_teleport_air(fighter, boma, id, status_kind, situation_kind, frame);
    nspecial_cancels(boma, status_kind, situation_kind);
    unfloat_confusion(fighter, status_kind);
    mewtwo_teleport_wall_ride(fighter, boma, status_kind, id);
    dj_upB_jump_refresh(fighter);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_MEWTWO )]
pub fn mewtwo_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		mewtwo_frame(fighter)
    }
}

pub unsafe fn mewtwo_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}