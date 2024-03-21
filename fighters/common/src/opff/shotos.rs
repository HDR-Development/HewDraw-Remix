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

// TRAINING MODE
// Full Meter Gain via shield during taunt
unsafe fn training_mode_full_meter(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if app::smashball::is_training_mode()
    && boma.is_status(*FIGHTER_STATUS_KIND_APPEAL)
    && boma.is_button_on(Buttons::Guard)
    {
        let meter_max = (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object()));
        MeterModule::add(boma.object(), meter_max);
    }
}

unsafe fn up_special_early_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP) 
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.motion_frame() >= 25.0 {
        fighter.change_status_req(*FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING, false);
    }
}

#[no_mangle]
pub unsafe extern "Rust" fn shotos_common(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        shotos_moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub unsafe fn shotos_moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //MeterModule::update(fighter.battle_object, false);
    //if boma.kind() != *FIGHTER_KIND_DOLLY {
        //utils::ui::UiManager::set_ex_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        //utils::ui::UiManager::set_ex_meter_info(
            //fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            //MeterModule::meter(fighter.object()),
            //(MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object())),
            //MeterModule::meter_per_level(fighter.object())
        //);
    //}
    //dtilt_utilt_repeat_increment(boma, id, motion_kind); // UNUSED
	//ex_shoryuken(boma, status_kind, situation_kind, motion_kind);
    training_mode_full_meter(fighter, boma, status_kind);
    up_special_early_landing(fighter);
}