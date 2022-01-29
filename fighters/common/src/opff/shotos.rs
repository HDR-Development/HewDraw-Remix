use super::*;
use smash::app::BattleObjectModuleAccessor;

// Dtilt and Utilt repeat increment
unsafe fn dtilt_utilt_repeat_increment(boma: &mut BattleObjectModuleAccessor) {
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
    && !VarModule::is_flag(boma.object(), vars::shotos::REPEAT_INCREMENTED)
    {
        if boma.is_motion(Hash40::new("attack_hi3_w")) {
            VarModule::inc_int(boma.object(), vars::shotos::REPEAT_COUNT_HI);
            VarModule::on_flag(boma.object(), vars::shotos::REPEAT_INCREMENTED);
        } else if boma.is_motion(Hash40::new("attack_lw3_w")) {
            VarModule::inc_int(boma.object(), vars::shotos::REPEAT_COUNT_LW);
            VarModule::on_flag(boma.object(), vars::shotos::REPEAT_INCREMENTED);
        }
    }
}

// Shotos Tatsumaki Land Cancel, hover, and EX momentum handling
unsafe fn tatsumaki_ex_land_cancel_hover(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    let jump_rising = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let stop_rise = Vector3f{x: 1.0, y: 0.0, z: 1.0};
	let ex_momentum = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let prev_situation_kind = StatusModule::prev_situation_kind(boma);

    if !boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END
    ])
    {
        return;
    }

    if boma.is_situation(*SITUATION_KIND_GROUND) && boma.is_prev_situation(*SITUATION_KIND_AIR) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
    }

    if VarModule::is_flag(boma.object(), vars::shotos::IS_USE_EX_SPECIAL) {
        KineticModule::mul_speed(boma, &Vector3f::zero(), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    if !boma.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END)
    && boma.is_situation(*SITUATION_KIND_AIR)
    && boma.is_button_on(Buttons::Special | Buttons::Attack)
    && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) < 0.0
    {
        KineticModule::mul_speed(boma, &Vector3f::new(1.0, 0.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

// Shotos EX Shoryuken
unsafe fn ex_shoryuken(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    if !VarModule::is_flag(boma.object(), vars::shotos::IS_USE_EX_SPECIAL) {
        return;
    }

    if !boma.is_motion_one_of(&[Hash40::new("attack_11_w"), Hash40::new("attack_11_s"), Hash40::new("attack_11_near_s")]) {
        return;
    }

    ControlModule::clear_command(boma, true);
    WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK);
    WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
    WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER);
    WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
    WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);

    if boma.is_motion_one_of(&[Hash40::new("attack_11_w"), Hash40::new("attack_11_s")]) {
        MotionModule::change_motion_kind(boma, Hash40::new("attack_11_near_s"));
    }

    if boma.is_status(*FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR) {
        DamageModule::add_damage(boma, 10.0, 0);
    }
}

// The actual super fs cancel code since it's used on both ryu and ken w/ separate inputs
unsafe fn super_fs_cancel(boma: &mut BattleObjectModuleAccessor) -> bool {
    if MeterModule::drain(boma.object(), 10) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
        true
    } else {
        false
    }
}

// Shotos Hadoken FADC and Super (FS) cancels
unsafe fn hadoken_fadc_sfs_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, cat: [i32; 4], frame: f32) {

    let mut agent_base = fighter.fighter_base.agent_base;
    let cat1 = cat[0];
    let cat4 = cat[3];
    let fighter_kind = boma.kind();

    let frame = MotionModule::frame(boma);

    if !boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND
    ])
    || frame <= 5.0 {
        return;
    }


    if boma.kind() == *FIGHTER_KIND_RYU
    && boma.is_cat_flag(Cat4::SpecialNCommand | Cat4::SpecialN2Command | Cat4::SpecialHiCommand)
    && super_fs_cancel(boma) {
        return;
    }

    if boma.kind() == *FIGHTER_KIND_KEN
    && boma.is_cat_flag(Cat4::SpecialSCommand | Cat4::SpecialHiCommand)
    && super_fs_cancel(boma) {
        return;
    }

    if frame > 15.0
    && boma.is_cat_flag(Cat1::SpecialLw)
    && MeterModule::drain(boma.object(), 1)
    {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
    }
}

// Shotos Special hit cancels
// Only for Ken?
unsafe fn special_hit_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if boma.kind() == *FIGHTER_KIND_KEN
    && boma.is_status_one_of(&[*FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2])
    {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
}

// Shotos Shield Stop and Run Drop
unsafe fn shield_stop_run_drop(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if !boma.is_status(*FIGHTER_RYU_STATUS_KIND_DASH_BACK) {
        return;
    }

    if boma.is_pad_flag(PadFlag::GuardTrigger)
    && boma.is_button_off(Buttons::Catch)
    {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
        ControlModule::clear_command(boma, true);
    }

    let flick_y = ParamModule::get_float(boma.object(), ParamType::Common, "general_flick_y_sens");

    if GroundModule::is_passable_ground(boma) && boma.is_flick_y(flick_y) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, false);
    }
}

// TRAINING MODE
// Full Meter Gain via shield during taunt
unsafe fn training_mode_full_meter(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if app::smashball::is_training_mode()
    && boma.is_status(*FIGHTER_STATUS_KIND_APPEAL)
    && boma.is_button_on(Buttons::Guard)
    {
        let meter_max = ParamModule::get_float(boma.object(), ParamType::Common, "meter_max_damage");
        MeterModule::add(boma.object(), meter_max);
    }
}

#[no_mangle]
pub unsafe extern "Rust" fn shotos_common(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        shotos_moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub unsafe fn shotos_moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    MeterModule::update(fighter.battle_object, true);
    //dtilt_utilt_repeat_increment(boma, id, motion_kind); // UNUSED
    tatsumaki_ex_land_cancel_hover(boma, status_kind, situation_kind);
	//ex_shoryuken(boma, status_kind, situation_kind, motion_kind);
    hadoken_fadc_sfs_cancels(fighter, boma, id, status_kind, cat, frame);
    special_hit_cancels(boma, status_kind);
    shield_stop_run_drop(boma, status_kind);
    training_mode_full_meter(fighter, boma, status_kind);

    // Magic Series
    magic_series(fighter, boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

unsafe fn jab_cancels(boma: &mut BattleObjectModuleAccessor) {
    let new_status = if boma.is_motion(Hash40::new("attack_13")) {
        if boma.is_cat_flag(Cat1::AttackS4) {
            *FIGHTER_STATUS_KIND_ATTACK_S4_START
        } else if boma.is_cat_flag(Cat1::AttackHi4) {
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START
        } else if boma.is_cat_flag(Cat1::AttackLw4) {
            *FIGHTER_STATUS_KIND_ATTACK_LW4_START
        } else {
            return;
        }
    } else {
        if boma.is_cat_flag(Cat1::AttackS3) {
            *FIGHTER_STATUS_KIND_ATTACK_S3
        } else if boma.is_cat_flag(Cat1::AttackHi3) {
            *FIGHTER_STATUS_KIND_ATTACK_HI3
        } else if boma.is_cat_flag(Cat1::AttackLw3) {
            *FIGHTER_STATUS_KIND_ATTACK_LW3
        } else {
            return;
        }
    };

    VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
    StatusModule::change_status_request_from_script(boma, new_status, false);
}

unsafe fn tilt_cancels(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_HI3)
    && boma.is_input_jump()
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
    {
        VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
        boma.change_status_req(*FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        return;
    }

    let new_status = if boma.is_cat_flag(Cat1::AttackS4) {
        *FIGHTER_STATUS_KIND_ATTACK_S4_START
    } else if boma.is_cat_flag(Cat1::AttackHi4) {
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START
    } else if boma.is_cat_flag(Cat1::AttackLw4) {
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START
    } else {
        return;
    };

    VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
    boma.change_status_req(new_status, true);
}

unsafe fn smash_cancels(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);

    if boma.kind() == *FIGHTER_KIND_RYU {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);

        if boma.is_cat_flag(Cat4::SpecialN2Command) {
            boma.change_status_req(*FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, false);
            return;
        }
    } else if boma.kind() == *FIGHTER_KIND_KEN {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);

        if boma.is_cat_flag(Cat4::Command1) {
            boma.change_status_req(*FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1, false);
            return;
        } else if boma.is_cat_flag(Cat4::Command2) {
            boma.change_status_req(*FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2, false);
            return;
        }
    }

    if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_HI4)
    && boma.is_input_jump()
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
    }

    let new_status = if boma.is_cat_flag(Cat4::SpecialNCommand) {
        *FIGHTER_STATUS_KIND_SPECIAL_N
    } else if boma.is_cat_flag(Cat1::SpecialN) {
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND
    } else if boma.is_cat_flag(Cat4::SpecialSCommand) {
        *FIGHTER_STATUS_KIND_SPECIAL_S
    } else if boma.is_cat_flag(Cat1::SpecialS) {
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND
    } else if boma.is_cat_flag(Cat4::SpecialHiCommand) {
        *FIGHTER_STATUS_KIND_SPECIAL_HI
    } else if boma.is_cat_flag(Cat1::SpecialHi) {
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND
    } else if boma.is_cat_flag(Cat1::SpecialLw) {
        *FIGHTER_STATUS_KIND_SPECIAL_LW
    } else {
        return;
    };

    boma.change_status_req(new_status, false);
}

unsafe fn aerial_cancels(boma: &mut BattleObjectModuleAccessor) {
    #[derive(PartialEq, Eq)]
    enum AerialDirection {
        Neutral,
        Forward,
        Backward,
        Up,
        Down,
        None
    }

    unsafe fn get_aerial_direction(boma: &mut BattleObjectModuleAccessor) -> AerialDirection {
        if boma.is_cat_flag(Cat1::AttackS3 | Cat1::AttackS4) {
            if boma.stick_x() < 0.0 {
                AerialDirection::Backward
            } else {
                AerialDirection::Forward
            }
        } else if boma.is_cat_flag(Cat1::AttackHi3 | Cat1::AttackHi4) {
            AerialDirection::Up
        } else if boma.is_cat_flag(Cat1::AttackLw3 | Cat1::AttackLw4) {
            AerialDirection::Down
        } else if boma.is_cat_flag(Cat1::AttackAirN | Cat1::AttackN) {
            AerialDirection::Neutral
        } else {
            AerialDirection::None
        }
    }

    if boma.is_input_jump()
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
    && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
        return;
    }

    let dir = get_aerial_direction(boma);
    match MotionModule::motion_kind(boma) {
        super::hash40!("attack_air_n") if matches!(dir, AerialDirection::None | AerialDirection::Neutral) => return,
        super::hash40!("attack_air_f") if matches!(dir, AerialDirection::None | AerialDirection::Neutral | AerialDirection::Forward) => return,
        super::hash40!("attack_air_b") => return,
        super::hash40!("attack_air_hi") if !matches!(dir, AerialDirection::Backward | AerialDirection::Down) => return,
        super::hash40!("attack_air_lw") if !matches!(dir, AerialDirection::Backward) => return,
        _ => {
            boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_AIR, false);
        }
    }
}

unsafe fn special_cancels(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_cat_flag(Cat1::SpecialLw) && MeterModule::drain(boma.object(), 1) {
        boma.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_LW, true);
        return;
    }

    if boma.kind() == *FIGHTER_KIND_RYU
    && boma.is_cat_flag(Cat4::SpecialNCommand | Cat4::SpecialN2Command | Cat4::SpecialHiCommand)
    {
        super_fs_cancel(boma);
        return;
    }

    if boma.kind() == *FIGHTER_KIND_KEN
    && boma.is_cat_flag(Cat4::SpecialSCommand | Cat4::SpecialHiCommand)
    {
        super_fs_cancel(boma);
        return;
    }
}

unsafe fn magic_series(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        return;
    }

    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH]) {
        jab_cancels(boma);
        return;
    }

    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3]) {
        tilt_cancels(boma);
        return;
    }

    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4]) {
        smash_cancels(boma);
        return;
    }

    // Aerial Cancels
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        aerial_cancels(boma);
        return;
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END
    ]) {
        special_cancels(boma);
        return;
    }
}
