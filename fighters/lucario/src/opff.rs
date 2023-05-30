// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

#[fighter_frame( agent = FIGHTER_KIND_LUCARIO )]
pub fn lucario_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        MeterModule::update(fighter.object(), false);
        utils::ui::UiManager::set_ff_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_ff_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            MeterModule::meter(fighter.object()),
            ParamModule::get_float(fighter.object(), ParamType::Common, "meter_max_damage"),
            MeterModule::meter_per_level(fighter.object())
        );
    }
}

unsafe fn training_mode_deplete_meter(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if app::smashball::is_training_mode()
    && boma.is_status(*FIGHTER_STATUS_KIND_APPEAL)
    && boma.is_button_on(Buttons::Guard)
    {
        MeterModule::reset(fighter.battle_object);
        VarModule::on_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT);
    }
}
 
unsafe fn extreme_speed_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END, false);
        }
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32) {
    //PM-like neutral-b canceling
    /***if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD {
        if boma.is_cat_flag(Cat2::CommonGuard) {
            if situation_kind == *SITUATION_KIND_AIR {
                if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                    WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL, true);
                }
            }
            else {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL, true);
            }
        }
    }***/
    if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    extreme_speed_cancel(boma, status_kind);
    nspecial_cancels(boma, status_kind, situation_kind, cat[1]);
    meter_module(fighter, boma);
    // Magic Series
    magic_series(fighter, boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
    training_mode_deplete_meter(fighter, boma, status_kind);
}

unsafe fn meter_module(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    MeterModule::set_damage_gain_mul(fighter.object(), 0.5);
    let level = MeterModule::level(fighter.object()) as f32;
    let meter_per_level = MeterModule::meter_per_level(fighter.object());
    let meter_max = ParamModule::get_float(fighter.object(), ParamType::Common, "meter_max_damage");
    if (level * meter_per_level >= meter_max) {
        VarModule::off_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT);
    }

    if !boma.is_in_hitlag() {
        let mut lockout_frame = VarModule::get_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME);
        if lockout_frame > 0 {
            lockout_frame = (lockout_frame - 1).max(0);
            VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, lockout_frame);
        } else {
            MeterModule::add(boma.object(), VarModule::get_float(fighter.battle_object, vars::lucario::instance::METER_PASSIVE_RATE));
        }
    }
}

unsafe fn magic_series(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    let cat1 = cat[0];
    // Level 1: Jab and Dash Attack Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind) {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
        || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {
            // Check for tilt attack inputs
            if boma.is_cat_flag(Cat1::AttackS3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3,false);
            }
            if boma.is_cat_flag(Cat1::AttackHi3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3,false);
            }
            if boma.is_cat_flag(Cat1::AttackLw3) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3,false);
            }

            // Check for smash attack inputs
            if boma.is_cat_flag(Cat1::AttackS4)
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::drain(boma.object(), 2) {
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
            }
            if boma.is_cat_flag(Cat1::AttackHi4)
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::drain(boma.object(), 2) {
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
            }
            if boma.is_cat_flag(Cat1::AttackLw4)
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::drain(boma.object(), 2) {
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
            }

            // Check for special attack inputs
            if boma.is_cat_flag(Cat1::SpecialN)
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::drain(boma.object(), 2) {
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
            }
            if boma.is_cat_flag(Cat1::SpecialS)
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::drain(boma.object(), 2) {
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
            }
            if boma.is_cat_flag(Cat1::SpecialHi)
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::drain(boma.object(), 2) {
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
            }
            if boma.is_cat_flag(Cat1::SpecialLw)
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::drain(boma.object(), 2) {
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
            }

            // Check for jump inputs during dash attack (on hit)
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
            && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) 
            && !boma.is_in_hitlag() 
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::level(boma.object()) >= 1
            && boma.check_jump_cancel(false) {
                MeterModule::drain(boma.object(), 1);
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
            }
        }
    }

    // Level 2: Tilt Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
        || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {
            // Check for smash attack inputs
            if boma.is_cat_flag(Cat1::AttackS4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
            }
            if boma.is_cat_flag(Cat1::AttackHi4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
            }
            if boma.is_cat_flag(Cat1::AttackLw4) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
            }

            // Check for special attack inputs
            if boma.is_cat_flag(Cat1::SpecialN)
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::drain(boma.object(), 2) {
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
            }
            if boma.is_cat_flag(Cat1::SpecialS)
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::drain(boma.object(), 2) {
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
            }
            if boma.is_cat_flag(Cat1::SpecialHi)
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::drain(boma.object(), 2) {
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
            }
            if boma.is_cat_flag(Cat1::SpecialLw)
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::drain(boma.object(), 2) {
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
            }

            // Check for jump inputs during utilt
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
            && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            && !boma.is_in_hitlag()
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::level(boma.object()) >= 1
            && boma.check_jump_cancel(false) {
                MeterModule::drain(boma.object(), 1);
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
            }
        }
    }

    // Smash Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
        || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {

            // Check for special attack inputs
            if boma.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
            }
            if boma.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
            }
            if boma.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
            }
            if boma.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
            }

            // Check for jump inputs
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
            && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            && !boma.is_in_hitlag()
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::level(boma.object()) >= 1
            && boma.check_jump_cancel(false) {
                MeterModule::drain(boma.object(), 1);
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
            }
        }
    }

    // Aerial Cancels
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
        || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {
            // Check for jump inputs
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) 
            && !boma.is_in_hitlag() 
            && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::level(boma.object()) >= 1
            && boma.check_jump_cancel(false) {
                MeterModule::drain(boma.object(), 1);
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
            }
            // Check for special attack inputs
            if boma.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
            }
            if boma.is_cat_flag(Cat1::SpecialS) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
            }
            if boma.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
            }
            if boma.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
            }
        }
    }

    // Extreme Speed Cancels
    if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END 
    || status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
        || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag())
        {
            if !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) 
            && MeterModule::drain(boma.object(), 2) {
                VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 120);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
            }
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_LUCARIO )]
pub fn lucario_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		lucario_frame(fighter)
    }
}

pub unsafe fn lucario_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
