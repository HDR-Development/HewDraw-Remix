// opff import
utils::import_noreturn!(common::opff::{fighter_common_opff});
use super::*;
use globals::*;

 
unsafe fn slaughter_high_kick_devastator(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_HI3) && boma.is_motion(Hash40::new("attack_hi3")) {
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_CHECK_STEP) {
            if boma.is_cat_flag(Cat1::AttackS3 | Cat1::AttackS4) && !boma.is_in_hitlag() {
               if boma.is_stick_backward() {
                    VarModule::on_flag(boma.object(), vars::demon::instance::ATTACK_HI3_SLAUGHTER_HIGH_KICK);
                    boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_5, false);
               }
               if boma.is_stick_forward() {
                    VarModule::on_flag(boma.object(), vars::demon::instance::ATTACK_HI3_DEVASTATOR);
                    boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK, false);
               }
            }
        }
    }
    // shouldn't need these anymore as they get turned on/off when needed
    // if ![*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_5].contains(&status_kind) {
    //     VarModule::off_flag(boma.object(), vars::demon::instance::ATTACK_HI3_SLAUGHTER_HIGH_KICK);
    // }
    // if ![*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) {
    //     VarModule::off_flag(boma.object(), vars::demon::instance::ATTACK_HI3_DEVASTATOR);
    // }
}

unsafe fn lightning_screw_uppercut(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_stand_21")) {
        if boma.status_frame() < 19 {
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) && !VarModule::is_flag(boma.object(), vars::demon::instance::ATTACK_STEP2S_SPINNING_DEMON) {
                VarModule::on_flag(boma.object(), vars::demon::instance::ATTACK_STAND2_LIGHTNING_SCREW_UPPERCUT);
            }
        }
        else {
            if VarModule::is_flag(boma.object(), vars::demon::instance::ATTACK_STAND2_LIGHTNING_SCREW_UPPERCUT) {
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("attack_stand_22"), 0.0, 1.2, 0.0);
            }
        }
    }
    if boma.is_motion(Hash40::new("attack_stand_22")) && boma.status_frame() > 16 {
        if VarModule::is_flag(boma.object(), vars::demon::instance::ATTACK_STAND2_LIGHTNING_SCREW_UPPERCUT) {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("attack_stand_23"), 0.0, 1.15, 0.0);
        }
    }
    if boma.is_motion(Hash40::new("attack_stand_23")) && boma.status_frame() > 16 {
        if VarModule::is_flag(boma.object(), vars::demon::instance::ATTACK_STAND2_LIGHTNING_SCREW_UPPERCUT) {
            boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L, false);
        }
    }
    if !boma.is_motion_one_of(&[Hash40::new("attack_stand_21"), Hash40::new("attack_stand_22"), Hash40::new("attack_stand_23"), Hash40::new("attack_step_2l")]) {
        VarModule::off_flag(boma.object(), vars::demon::instance::ATTACK_STAND2_LIGHTNING_SCREW_UPPERCUT);
    }
}

unsafe fn spinning_demon(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_step_2s")) {
        if boma.status_frame() > 16 && boma.status_frame() < 18 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                VarModule::on_flag(boma.object(), vars::demon::instance::ATTACK_STEP2S_SPINNING_DEMON);
            }
        }
        else if boma.status_frame() >= 18 {
            if VarModule::is_flag(boma.object(), vars::demon::instance::ATTACK_STEP2S_SPINNING_DEMON) {
                boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_2, false);
            }
        }
    }
    if boma.is_status(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_2)
    && VarModule::is_flag(boma.object(), vars::demon::instance::ATTACK_STEP2S_SPINNING_DEMON)
    && boma.is_motion(Hash40::new("attack_stand_21")) {
        MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("attack_stand_24"), 0.0, 1.0, 0.0);
    }
    if !boma.is_motion_one_of(&[Hash40::new("attack_stand_21"), Hash40::new("attack_stand_24"), Hash40::new("attack_step_2s")]) {
        VarModule::off_flag(boma.object(), vars::demon::instance::ATTACK_STEP2S_SPINNING_DEMON);
    }
}

unsafe fn korean_back_dash(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_DEMON_STATUS_KIND_DASH_BACK)
    && boma.left_stick_y() < WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y")) {
        boma.change_status_req(*FIGHTER_STATUS_KIND_SQUAT, false);
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
    ])
    && boma.is_cat_flag(Cat1::TurnDash) && boma.left_stick_y() > WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y")) {
        boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_DASH_BACK, false);
    }
}

unsafe fn enable_both_recovery_specials(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_LW_FALL]) && boma.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
    }
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_AIR_END]) && boma.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    }
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) && !VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
    }
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S) && !VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    }
}

// boma: its a boma
// start_frame: frame to start interpolating the body rotation
// bend_frame: frame to interpolate to the intended angle amount until
// return_frame: frame to start interpolating back to regular angle
// straight_frame: frame the body should be at the regular angle again
unsafe fn forward_bair_rotation(boma: &mut BattleObjectModuleAccessor, start_frame: f32, bend_frame: f32, return_frame: f32, straight_frame: f32) {
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let max_rotation = -180.0;
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        
    if frame >= start_frame && frame < return_frame {
        // this has to be called every frame, or you snap back to the normal joint angle
        // interpolate to the respective body rotation angle
        let calc_body_rotate = max_rotation * ((frame - start_frame) / (bend_frame - start_frame));
        let body_rotation = calc_body_rotate.clamp(-180.0, 0.0);
        rotation = Vector3f{x: 0.0, y: body_rotation, z: 0.0};
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    } else if frame >= return_frame && frame < straight_frame {
        // linear interpolate back to normal
        /*
        let calc_body_rotate = max_rotation *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let body_rotation = calc_body_rotate.clamp(0.0, max_rotation);
        */
        let calc_body_rotate = -180.0 *((frame - return_frame) / (straight_frame - return_frame)) + 180.0;
        let body_rotation = calc_body_rotate.clamp(0.0, 180.0);
        rotation = Vector3f{x: 0.0, y: body_rotation, z: 0.0};
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
}

unsafe fn rotate_forward_bair(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_air_b")){
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            forward_bair_rotation(boma, 6.0, 9.5, 21.0, 41.0);
        }
    }
    else if boma.is_motion(Hash40::new("landing_air_b")){
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            forward_bair_rotation(boma, 0.0, 0.1, 0.2, 9.0);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DEMON_STATUS_KIND_SPECIAL_N_AIR_SHOOT,
        *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_AIR_END,
        *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE,
        *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_FALL,
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

unsafe fn up_special_freefall(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(fighter.module_accessor)
    && (fighter.is_situation(*SITUATION_KIND_GROUND)
        || fighter.is_situation(*SITUATION_KIND_CLIFF)
        || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]))
    {
        VarModule::off_flag(fighter.battle_object, vars::demon::instance::SPECIAL_HI_ENABLE_FREEFALL);
    }

    if fighter.is_prev_status(*FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE) {
        if StatusModule::is_changing(fighter.module_accessor) {
            VarModule::on_flag(fighter.battle_object, vars::demon::instance::SPECIAL_HI_ENABLE_FREEFALL);
        }
    }

    if fighter.is_status(*FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE) {
        if fighter.is_situation(*SITUATION_KIND_AIR)
        && VarModule::is_flag(fighter.battle_object, vars::demon::instance::SPECIAL_HI_ENABLE_FREEFALL) {
            if CancelModule::is_enable_cancel(fighter.module_accessor) {
                let speed_x_max_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_max_speed_x_mul"));
                WorkModule::set_float(fighter.module_accessor, speed_x_max_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
                fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
                *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
            }
        }
    }
}

unsafe fn camera_lockout(fighter: &mut L2CFighterCommon) {
    let lockout = VarModule::get_int(fighter.battle_object, vars::demon::instance::CAMERA_LOCKOUT_TIMER);
    VarModule::set_int(fighter.battle_object, vars::demon::instance::CAMERA_LOCKOUT_TIMER, (lockout - 1).max(0));
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    slaughter_high_kick_devastator(boma);
    korean_back_dash(boma);
    lightning_screw_uppercut(boma);
    spinning_demon(boma);
    enable_both_recovery_specials(boma);
    fastfall_specials(fighter);
    up_special_freefall(fighter, boma);
    camera_lockout(fighter);
}

pub extern "C" fn demon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		demon_frame(fighter)
    }
}

pub unsafe fn demon_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, demon_frame_wrapper);
}
