// opff import
utils::import_noreturn!(common::opff::{fighter_common_opff});
use super::*;
use globals::*;

 
unsafe fn slaughter_high_kick_devastator(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    if [*FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind) && motion_kind == hash40("attack_hi3") {
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_CHECK_STEP){
            if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4){
               if boma.is_stick_backward() && !boma.is_in_hitlag() {
                    VarModule::on_flag(boma.object(), vars::demon::instance::SLAUGHTER_HIGH_KICK);
                    boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_5, false);
               }
               if boma.is_stick_forward() && !boma.is_in_hitlag() {
                    VarModule::on_flag(boma.object(), vars::demon::instance::DEVASTATOR);
                    boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK, false);
               }
            }
        }
    }
    if ![*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_5].contains(&status_kind) {
        VarModule::off_flag(boma.object(), vars::demon::instance::SLAUGHTER_HIGH_KICK);
    }
    if ![*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) {
        VarModule::off_flag(boma.object(), vars::demon::instance::DEVASTATOR);
    }
}

// unsafe fn jaw_breaker(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64, frame: f32) {
//     if [*FIGHTER_STATUS_KIND_ESCAPE].contains(&status_kind)
//         && boma.status_frame() > 17 {
//         if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N){
//             VarModule::on_flag(boma.object(), vars::demon::instance::JAW_BREAKER);
//             boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_HI3, false);
//         }
//     }
//     if ![*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind) {
//         VarModule::off_flag(boma.object(), vars::demon::instance::JAW_BREAKER);
//     }
// }

unsafe fn lightning_screw_uppercut(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("attack_stand_21") {
        if frame < 19.0{
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) && !VarModule::is_flag(boma.object(), vars::demon::instance::SPINNING_DEMON) {
                VarModule::on_flag(boma.object(), vars::demon::instance::LIGHTNING_SCREW_UPPERCUT);
            }
        }
        else{
            if VarModule::is_flag(boma.object(), vars::demon::instance::LIGHTNING_SCREW_UPPERCUT){
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("attack_stand_22"), 0.0, 1.2, 0.0);
            }
        }
    }
    if motion_kind == hash40("attack_stand_22") && frame > 16.0 {
        if VarModule::is_flag(boma.object(), vars::demon::instance::LIGHTNING_SCREW_UPPERCUT){
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("attack_stand_23"), 0.0, 1.15, 0.0);
        }
    }
    if motion_kind == hash40("attack_stand_23") && frame > 16.0 {
        if VarModule::is_flag(boma.object(), vars::demon::instance::LIGHTNING_SCREW_UPPERCUT){
            boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L, false);
        }
    }
    if ![hash40("attack_stand_21"), hash40("attack_stand_22"), hash40("attack_stand_23"), hash40("attack_step_2l")].contains(&motion_kind) {
        VarModule::off_flag(boma.object(), vars::demon::instance::LIGHTNING_SCREW_UPPERCUT);
    }
}

unsafe fn spinning_demon(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if motion_kind == hash40("attack_step_2s") {
        if frame > 16.0 && frame < 18.0{
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                VarModule::on_flag(boma.object(), vars::demon::instance::SPINNING_DEMON);
            }
        }
        else if frame >= 18.0{
            if VarModule::is_flag(boma.object(), vars::demon::instance::SPINNING_DEMON){
                boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_2, false);
            }
        }
    }
    if boma.is_status(*FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_2) && VarModule::is_flag(boma.object(), vars::demon::instance::SPINNING_DEMON) && motion_kind == hash40("attack_stand_21"){
        MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("attack_stand_24"), 0.0, 1.0, 0.0);
    }
    if ![hash40("attack_stand_21"), hash40("attack_stand_24"), hash40("attack_step_2s")].contains(&motion_kind) {
        VarModule::off_flag(boma.object(), vars::demon::instance::SPINNING_DEMON);
    }
}

unsafe fn korean_back_dash(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, stick_y: f32) {
    if boma.is_status(*FIGHTER_DEMON_STATUS_KIND_DASH_BACK)
    && boma.left_stick_y() < WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y"))
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_SQUAT, false);
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
    ])
    && compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH)
    && boma.left_stick_y() > WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y"))
    {
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

unsafe fn up_special_freefall(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(fighter.module_accessor)
    && (fighter.is_situation(*SITUATION_KIND_GROUND)
        || fighter.is_situation(*SITUATION_KIND_CLIFF)
        || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]))
    {
        VarModule::off_flag(fighter.battle_object, vars::demon::instance::UP_SPECIAL_FREEFALL);
    }

    if fighter.is_prev_status(*FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE) {
        if StatusModule::is_changing(fighter.module_accessor) {
            VarModule::on_flag(fighter.battle_object, vars::demon::instance::UP_SPECIAL_FREEFALL);
        }
    }

    if fighter.is_status(*FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE) {
        if fighter.is_situation(*SITUATION_KIND_AIR)
        && !StatusModule::is_changing(fighter.module_accessor)
        && VarModule::is_flag(fighter.battle_object, vars::demon::instance::UP_SPECIAL_FREEFALL) {
            if CancelModule::is_enable_cancel(fighter.module_accessor) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
                *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    slaughter_high_kick_devastator(boma, cat[0], status_kind, situation_kind, motion_kind);
    // jaw_breaker(boma, cat[0], status_kind, situation_kind, motion_kind, frame);
    korean_back_dash(boma, cat[0], status_kind, stick_y);
    lightning_screw_uppercut(boma, cat[0], status_kind, situation_kind, motion_kind, frame);
    spinning_demon(boma, cat[0], status_kind, situation_kind, motion_kind, frame);
    enable_both_recovery_specials(boma);
    rotate_forward_bair(boma);
    fastfall_specials(fighter);
    up_special_freefall(fighter, boma);
}

#[utils::macros::opff(FIGHTER_KIND_DEMON )]
pub fn demon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		demon_frame(fighter)
    }
}

pub unsafe fn demon_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}