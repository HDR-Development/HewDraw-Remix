// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK_TURN
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
    magic_series(fighter, boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
    extra_special_cancels(fighter, boma, status_kind, situation_kind, motion_kind, frame);
    metered_cancels(fighter, boma, frame);
    target_combos(boma);
    rotate_forward_bair(boma);
    turn_run_back_status(fighter, boma, status_kind);
    ryu_ex_shoryu(fighter, boma, cat, status_kind, situation_kind, motion_kind, frame);
    ryu_ex_hado(fighter, boma, frame);
    ryu_ex_tatsu(fighter, boma, frame);
    fastfall_specials(fighter);
}

// symbol-based call for the shotos' common opff
extern "Rust" {
    fn shotos_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}

#[fighter_frame_callback]
pub fn ryu_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        if fighter.kind() != FIGHTER_KIND_RYU {
            return;
        }
        MeterModule::update(fighter.battle_object, false);
        utils::ui::UiManager::set_ex_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_ex_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            MeterModule::meter(fighter.object()),
            (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object())),
            MeterModule::meter_per_level(fighter.object())
        );
    }
}

#[utils::macros::opff(FIGHTER_KIND_RYU )]
pub fn ryu_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        shotos_common(fighter);
		ryu_frame(fighter);
    }
}

pub unsafe fn ryu_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

/// enables shield during transition from dashback to run
unsafe fn turn_run_back_status(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind != *FIGHTER_RYU_STATUS_KIND_TURN_RUN_BACK {
        return;
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
}

// boma: its a boma
// start_frame: frame to start interpolating the body rotation
// bend_frame: frame to interpolate to the intended angle amount until
// return_frame: frame to start interpolating back to regular angle
// straight_frame: frame the body should be at the regular angle again
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
        /*
        let calc_body_rotate = max_rotation *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let body_rotation = calc_body_rotate.clamp(0.0, max_rotation);
        */
        let calc_body_rotate = 180.0 *((frame - return_frame) / (straight_frame - return_frame)) + 180.0;
        let body_rotation = calc_body_rotate.clamp(180.0, 360.0);
        rotation = Vector3f{x: 0.0, y: body_rotation, z: 0.0};
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
}

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

unsafe fn ryu_ex_shoryu(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, frame: f32) {
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
    && boma.is_button_on(Buttons::AttackAll | Buttons::Catch | Buttons::AppealAll)
    && boma.is_button_on(Buttons::SpecialAll)
    && frame <= 4.0 {
        // change into different motions depending on current motion
        // MeterModule and VarModule calls are repeated so that I know
        // for 100% fact they can only be called if we change motion
        if boma.is_motion(Hash40::new("special_hi"))
        && MeterModule::drain(boma.object(), 1) {
            MotionModule::change_motion(boma, Hash40::new("special_hi_ex"), frame, 1.0, false, 0.0, false, false);
            VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);

        } else if boma.is_motion(Hash40::new("special_hi_command"))
        && MeterModule::drain(boma.object(), 1) {
            MotionModule::change_motion(boma, Hash40::new("special_hi_command_ex"), frame, 1.0, false, 0.0, false, false);
            VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);

        } else if boma.is_motion(Hash40::new("special_air_hi"))
        && MeterModule::drain(boma.object(), 1) {
            MotionModule::change_motion(boma, Hash40::new("special_air_hi_ex"), frame, 1.0, false, 0.0, false, false);
            VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);

        } else if boma.is_motion(Hash40::new("special_air_hi_command"))
        && MeterModule::drain(boma.object(), 1) {
            MotionModule::change_motion(boma, Hash40::new("special_air_hi_command_ex"), frame, 1.0, false, 0.0, false, false);
            VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);

        }
    }
}

unsafe fn ryu_ex_hado(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if !boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND,
    ]) {
        return;
    }

    // EX Hado
    if !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL)
    && !ArticleModule::is_exist(boma, *FIGHTER_RYU_GENERATE_ARTICLE_HADOKEN)
    && boma.is_button_on(Buttons::AttackAll | Buttons::Catch | Buttons::AppealAll)
    && boma.is_button_on(Buttons::SpecialAll)
    && frame > 1.0 && frame <= 4.0
    && MeterModule::drain(boma.object(), 1) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
    }

}

unsafe fn ryu_ex_tatsu(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
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
    && boma.is_button_on(Buttons::AttackAll | Buttons::Catch | Buttons::AppealAll)
    && boma.is_button_on(Buttons::SpecialAll)
    && frame <= 7.0
    && MeterModule::drain(boma.object(), 1) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
    }

    if VarModule::is_flag(boma.object(), vars::shotos::instance::IS_USE_EX_SPECIAL) {
        KineticModule::mul_speed(boma, &Vector3f::zero(), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    } 
    // Tatsu gravity
    // if holding special in the air, we float
    // params have been modified to make us fall otherwise
    else if !boma.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END)
    && boma.is_situation(*SITUATION_KIND_AIR)
    && boma.is_button_on(Buttons::SpecialAll)
    && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) < 0.0
    {
        KineticModule::mul_speed(boma, &Vector3f::new(1.0, 0.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

/// determines what cancels can be done out of specials
unsafe fn metered_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {

    let is_other_special_cancel = (boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1,
        *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2,
        ]) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    );

    let is_nspecial_cancel = (boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND
        ]) && frame > 13.0
    );

    if !is_nspecial_cancel && !is_other_special_cancel {
        return;
    }

    // check final smashes
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    let is_special = fighter.is_cat_flag(Cat1::SpecialAny);
    // the shinku hadouken
    if is_special
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0
    && MeterModule::level(fighter.object()) >= 6 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_FINAL2.into(), true.into());
        AttackModule::clear_all(fighter.module_accessor);
        return;
    }
    // the shin shoryuken
    if is_special
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0
    && MeterModule::level(fighter.object()) >= 6 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
        fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(), true.into());
        AttackModule::clear_all(fighter.module_accessor);
        return;
    }

    // DSpecial cancels
    // costs more meter on shield
    if boma.is_cat_flag(Cat1::SpecialLw)
    && MeterModule::drain(boma.object(), 1) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        boma.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        return;
    }
}

// Target combos
unsafe fn target_combos(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_in_hitlag() 
    || CancelModule::is_enable_cancel(boma)
    || !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        return;
    }

    if boma.is_motion_one_of(&[Hash40::new("attack_11_s"), Hash40::new("attack_11_near_s")])
    && boma.is_cat_flag(Cat1::AttackN) 
    && !boma.is_cat_flag(Cat1::AttackHi3)
    && !boma.is_cat_flag(Cat1::AttackS3)
    && !boma.is_cat_flag(Cat1::AttackLw3)
    && !StopModule::is_stop(boma) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        MotionModule::change_motion(boma, Hash40::new("attack_12_s"), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe fn magic_series(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // Dont use magic series if we're already in cancel frames, if we're in hitlag, or if we didn't connect
    if !VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL)
    || CancelModule::is_enable_cancel(boma) 
    || boma.is_in_hitlag() 
    || !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        return;
    }

    if [
        *FIGHTER_STATUS_KIND_ATTACK_HI4, 
    ].contains(&status_kind) 
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
    && boma.check_jump_cancel(false, true) {
        return;
    }

    // Smash cancels
    if [
        *FIGHTER_STATUS_KIND_ATTACK, 
        // *FIGHTER_STATUS_KIND_ATTACK_DASH, 
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
    ].contains(&status_kind) {
        if boma.is_cat_flag(Cat1::AttackS4) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
        }
        if boma.is_cat_flag(Cat1::AttackHi4) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
        }
        if boma.is_cat_flag(Cat1::AttackLw4) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
        }
    }

    // Special cancels
    if [
        *FIGHTER_STATUS_KIND_ATTACK, 
        // *FIGHTER_STATUS_KIND_ATTACK_DASH, 
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status_kind) {
        check_special_cancels(fighter, boma, status_kind, situation_kind, motion_kind, frame);
    }

    // Special cancels
    if [
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status_kind) {
        aerial_cancels(boma);
    }
}

unsafe fn extra_special_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64, frame: f32) {
    if WorkModule::is_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) 
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_S4
    ]) {
        check_special_cancels(fighter, boma, status_kind, situation_kind, motion_kind, frame);
    }
}

unsafe fn check_special_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64, frame: f32) {
    // Dont use cancels if we're already in cancel frames, if we're in hitlag, or if we didn't connect
    if CancelModule::is_enable_cancel(boma) 
    || boma.is_in_hitlag() {
        return;
    }
    
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1,
    ];
    let mut enableds = [false; 10];
    for x in 0..terms.len() {
        enableds[x] = WorkModule::is_enable_transition_term(fighter.module_accessor, terms[x]);
    }
    for val in terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *val);
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        fighter.sub_transition_group_check_air_special()
    } else {
        fighter.sub_transition_group_check_ground_special()
    };
    for x in 0..terms.len() {
        if !enableds[x] {
            WorkModule::unable_transition_term(fighter.module_accessor, terms[x]);
        }
    }
}

unsafe fn aerial_cancels(boma: &mut BattleObjectModuleAccessor) {
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
    && boma.check_airdodge_cancel() {
        return;
    }

    let dir = boma.get_aerial();
    if dir == None {
        return;
    }
    match MotionModule::motion_kind(boma) {
        super::hash40!("attack_air_n")  if matches!(dir, Some(AerialKind::Nair)) => return,
        super::hash40!("attack_air_hi") if matches!(dir, Some(AerialKind::Nair) | Some(AerialKind::Uair)) => return,
        super::hash40!("attack_air_f")  if matches!(dir, Some(AerialKind::Nair) | Some(AerialKind::Uair) | Some(AerialKind::Fair)) => return,
        super::hash40!("attack_air_b") => return,
        super::hash40!("attack_air_lw") => return,
        _ => {
            boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_AIR, false);
        }
    }
}
