// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    metered_cancels(fighter, boma, frame);
    target_combos(boma);
    rotate_forward_bair(boma);
    turn_run_back_status(fighter, boma, status_kind);
    heat_rush(boma, frame);
    air_hado_distinguish(fighter, boma, frame);
    ken_ex_flag_reset(boma);
    ken_ex_shoryu(fighter, boma, cat, status_kind, situation_kind, motion_kind, frame);
    tatsu_behavior_and_ex(fighter, boma, frame);
}

// symbol-based call for the shotos' common opff
extern "Rust" {
    fn shotos_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}

#[fighter_frame( agent = FIGHTER_KIND_KEN )]
pub fn ken_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        MeterModule::update(fighter.battle_object, false);
        utils::ui::UiManager::set_ex_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_ex_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            MeterModule::meter(fighter.object()),
            ParamModule::get_float(fighter.object(), ParamType::Common, "meter_max_damage"),
            MeterModule::meter_per_level(fighter.object())
        );
    }
}

#[utils::macros::opff(FIGHTER_KIND_KEN)]
pub fn ken_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        shotos_common(fighter);
		ken_frame(fighter);
    }
}

pub unsafe fn ken_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

unsafe fn ken_ex_flag_reset(boma: &mut BattleObjectModuleAccessor) {
    if !boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_STATUS_KIND_SPECIAL_S, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP
    ]) {
        VarModule::off_flag(boma.object(), vars::shotos::instance::IS_USE_EX_SPECIAL);
    }
}

unsafe fn ken_ex_shoryu(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, frame: f32) {
    if !boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP,
    ])
    || VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        return;
    }
    // only check EX if this is a heavy shoryu with A+B on f4
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_S
    && boma.is_button_on(Buttons::Attack)
    && boma.is_button_on(Buttons::Special)
    && frame == 4.0 {
        // change into different motions depending on current motion
        // MeterModule and VarModule calls are repeated so that I know
        // for 100% fact they can only be called if we change motion
        if boma.is_motion(Hash40::new("special_hi"))
        && MeterModule::drain(boma.object(), 4) {
            MotionModule::change_motion(boma, Hash40::new("special_hi_ex"), frame, 1.0, false, 0.0, false, false);
            VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);

        } else if boma.is_motion(Hash40::new("special_hi_command"))
        && MeterModule::drain(boma.object(), 4) {
            MotionModule::change_motion(boma, Hash40::new("special_hi_command_ex"), frame, 1.0, false, 0.0, false, false);
            VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);

        } else if boma.is_motion(Hash40::new("special_air_hi"))
        && MeterModule::drain(boma.object(), 4) {
            MotionModule::change_motion(boma, Hash40::new("special_air_hi_ex"), frame, 1.0, false, 0.0, false, false);
            VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);

        } else if boma.is_motion(Hash40::new("special_air_hi_command"))
        && MeterModule::drain(boma.object(), 4) {
            MotionModule::change_motion(boma, Hash40::new("special_air_hi_command_ex"), frame, 1.0, false, 0.0, false, false);
            VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);

        }
    }
}

unsafe fn air_hado_distinguish(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if !boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND,
    ]) {
        return;
    }

    // EX Hado
    if !boma.is_status_one_of(&[*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND])
    && !ArticleModule::is_exist(boma, *FIGHTER_RYU_GENERATE_ARTICLE_HADOKEN)
    && boma.is_button_on(Buttons::Attack)
    && boma.is_button_on(Buttons::Special)
    && frame <= 4.0
    && MeterModule::drain(boma.object(), 1) {
        boma.change_status_req(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, true);
    }

    // set VarModule flag on f12 - this flag changes hado properties
    if frame == 12.0 && fighter.is_motion_one_of(&[
        Hash40::new("special_air_n"), 
    ]) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_CURRENT_HADOKEN_AIR);
    }
    // after frame 13, disallow changing from aerial to grounded hadoken
    // instead, we enter a landing animation
    if (frame > 13.0 || fighter.is_motion_one_of(&[
        Hash40::new("special_air_n_empty"), 
        Hash40::new("special_n_empty"), 
    ]))
    && boma.is_situation(*SITUATION_KIND_GROUND) 
    && boma.is_prev_situation(*SITUATION_KIND_AIR) {
        if frame < 70.0 { // the autocancel frame
            WorkModule::set_float(boma, 11.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            boma.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        } else {
            boma.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
        }
    }
}

unsafe fn tatsu_behavior_and_ex(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if !fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP
    ]) {
        return;
    }

    // EX Tatsu
    // if A+B on f4 and not already EX tatsu, set EX_SPECIAL flag
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, 
    ])
    && !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL)
    && boma.is_button_on(Buttons::Attack)
    && boma.is_button_on(Buttons::Special)
    && frame <= 4.0
    && MeterModule::drain(boma.object(), 2) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        // burst of speed specifically for EX tatsu
        KineticModule::add_speed(boma, &Vector3f::new(2.0 , 0.0, 0.0));
    }

    // Tatsu gravity
    // if holding special in the air, we float
    // params have been modified to make us fall otherwise
    if !boma.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END)
    && boma.is_situation(*SITUATION_KIND_AIR)
    && boma.is_button_on(Buttons::Special)
    && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) < 0.0
    {
        KineticModule::mul_speed(boma, &Vector3f::new(1.0, 0.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

/// enables shield during transition from dashback to run
unsafe fn turn_run_back_status(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind != *FIGHTER_RYU_STATUS_KIND_TURN_RUN_BACK {
        return;
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
}

/// rotates bair forward for the held version
/// start_frame: frame to start interpolating the body rotation
/// bend_frame: frame to interpolate to the intended angle amount until
/// return_frame: frame to start interpolating back to regular angle
/// straight_frame: frame the body should be at the regular angle again
unsafe fn forward_bair_rotation(boma: &mut BattleObjectModuleAccessor, start_frame: f32, bend_frame: f32, return_frame: f32, straight_frame: f32) {
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let max_rotation = 180.0;
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        
    if frame >= start_frame && frame < return_frame {
        // this has to be called every frame, or you snap back to the normal joint angle
        // interpolate to the respective body rotation angle
        let calc_body_rotate = max_rotation * ((frame - start_frame) / (bend_frame - start_frame));
        let body_rotation = calc_body_rotate.clamp(0.0, max_rotation);
        rotation = Vector3f{x: 0.0, y: body_rotation, z: 0.0};
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    } else if frame >= return_frame && frame < straight_frame {
        // linear interpolate back to normal
        let calc_body_rotate = 180.0 *((frame - return_frame) / (straight_frame - return_frame)) + 180.0;
        let body_rotation = calc_body_rotate.clamp(180.0, 360.0);
        rotation = Vector3f{x: 0.0, y: body_rotation, z: 0.0};
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
}

/// logic behind rotate forward bair activation
unsafe fn rotate_forward_bair(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_air_b")){
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            forward_bair_rotation(boma, 6.0, 8.5, 11.0, 31.0);
        }
    }
    else if boma.is_motion(Hash40::new("landing_air_b")){
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            forward_bair_rotation(boma, 0.0, 0.1, 0.2, 11.0);
        }
    }
}

/// implements heat rush by instantly canceling focus into FADC
unsafe fn heat_rush(boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]) {
        boma.change_status_req(*FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, false);
    }

    // resets DISABLE_SPECIAL_LW on hitting a move
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        VarModule::off_flag(boma.object(), vars::shotos::instance::DISABLE_SPECIAL_LW);
    } 
}

/// determines what cancels can be done out of specials
unsafe fn metered_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {

    // don't do anything during hitlag
    if boma.is_in_hitlag() {
        return;
    }

    // don't do anything unless we're in a special and hit something (special case for NSpecial)
    if !((boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1,
        *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2,
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::ken::ATTACK_COMMAND_4)
        ]) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    ) || (boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND
        ]) && frame > 13.0
    )) {
        return;
    }
    // from this point on, the fighter is in a valid spot to try and cancel
    
    // super cancels
    let cat1 =  fighter.global_table[CMD_CAT1].get_i32();
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    // the shinryuken
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY != 0
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0
    && WorkModule::is_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
    && MeterModule::drain(fighter.object(), 8) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_FINAL2.into(), true.into());
        return;
    }
    // the tatsu super
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY != 0
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0
    && WorkModule::is_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
    && MeterModule::drain(fighter.object(), 10) {
        fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(), true.into());
        return;
    }

    // DSpecial cancels
    if boma.is_cat_flag(Cat1::SpecialLw)
    && MeterModule::drain(boma.object(), 2) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        boma.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        return;
    }
}

/// Target combos
unsafe fn target_combos(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_in_hitlag() {
        return;
    }
    if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        return;
    }
    // light UTilt --> jab
    if boma.is_motion_one_of(&[Hash40::new("attack_hi3_w")]){
        if boma.is_cat_flag(Cat1::AttackN) 
        && !boma.is_cat_flag(Cat1::AttackLw3)
        && !boma.is_cat_flag(Cat1::AttackS3)
        && !boma.is_cat_flag(Cat1::AttackHi3) {
            WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
            boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK, false);
        }
    }

    // far light FTilt --> NSpecial
    else if boma.is_motion(Hash40::new("attack_s3_s_w")) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
        if boma.is_cat_flag(Cat1::SpecialAny)
        && boma.is_cat_flag(Cat4::SpecialNCommand) {
            WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
            boma.change_status_req(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND, true);
        }
        else if boma.is_cat_flag(Cat1::SpecialN) {
            WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
            boma.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_N, true);
        }
    }

    // close FTilt --> DSmash
    else if boma.is_motion(Hash40::new("attack_near_w"))
    && boma.is_cat_flag(Cat1::AttackLw4) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4);
        boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_LW4, false);
    }
}