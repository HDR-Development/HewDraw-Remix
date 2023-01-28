use super::*;
use smash::app::BattleObjectModuleAccessor;
use globals::*;

// Dtilt and Utilt repeat increment
// unsafe fn dtilt_utilt_repeat_increment(boma: &mut BattleObjectModuleAccessor) {
//     if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
//     && !VarModule::is_flag(boma.object(), vars::shotos::status::REPEAT_INCREMENTED)
//     {
//         if boma.is_motion(Hash40::new("attack_hi3_w")) {
//             VarModule::inc_int(boma.object(), vars::shotos::instance::REPEAT_COUNT_HI);
//             VarModule::on_flag(boma.object(), vars::shotos::status::REPEAT_INCREMENTED);
//         } else if boma.is_motion(Hash40::new("attack_lw3_w")) {
//             VarModule::inc_int(boma.object(), vars::shotos::instance::REPEAT_COUNT_LW);
//             VarModule::on_flag(boma.object(), vars::shotos::status::REPEAT_INCREMENTED);
//         }
//     }
// }

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

    if VarModule::is_flag(boma.object(), vars::shotos::instance::IS_USE_EX_SPECIAL) {
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
// unsafe fn ex_shoryuken(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64) {
//     if !VarModule::is_flag(boma.object(), vars::shotos::instance::IS_USE_EX_SPECIAL) {
//         return;
//     }

//     if !boma.is_motion_one_of(&[Hash40::new("attack_11_w"), Hash40::new("attack_11_s"), Hash40::new("attack_11_near_s")]) {
//         return;
//     }

//     ControlModule::clear_command(boma, true);
//     WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK);
//     WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
//     WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
//     WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER);
//     WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
//     WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);

//     if boma.is_motion_one_of(&[Hash40::new("attack_11_w"), Hash40::new("attack_11_s")]) {
//         MotionModule::change_motion_kind(boma, Hash40::new("attack_11_near_s"));
//     }

//     if boma.is_status(*FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR) {
//         DamageModule::add_damage(boma, 10.0, 0);
//     }
// }

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
    && MeterModule::drain(boma.object(), 2)
    {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
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
    MeterModule::update(fighter.battle_object, false);
    if boma.kind() != *FIGHTER_KIND_DOLLY {
        utils::ui::UiManager::set_ex_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_ex_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            MeterModule::meter(fighter.object()),
            ParamModule::get_float(fighter.object(), ParamType::Common, "meter_max_damage"),
            MeterModule::meter_per_level(fighter.object())
        );
    }
    //dtilt_utilt_repeat_increment(boma, id, motion_kind); // UNUSED
    tatsumaki_ex_land_cancel_hover(boma, status_kind, situation_kind);
	//ex_shoryuken(boma, status_kind, situation_kind, motion_kind);
    hadoken_fadc_sfs_cancels(fighter, boma, id, status_kind, cat, frame);
    training_mode_full_meter(fighter, boma, status_kind);
}