// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn clown_cannon_shield_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_HOLD {
        if frame > 16.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                }
            }
        }
    }
}

// Bowser Jr. Clown Cannon Dash Cancel
// unsafe fn clown_cannon_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
   // if status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_SHOOT {
        // if frame > 13.0 {
            // if situation_kind == *SITUATION_KIND_GROUND {
              //  if boma.is_cat_flag(Cat1::Dash) {
                   // StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, false);
                // }
               // if boma.is_cat_flag(Cat1::TurnDash) {
                   // StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN_DASH, false);
                // }
            // }
        // }
    // }
// }

// Bowser Jr. Kart Jump Waveland
unsafe fn kart_jump_waveland(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP {
        boma.check_airdodge_cancel();
    }
}

unsafe fn upB_kart_respawn(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    // Respawns Clown Kart and allows actionability once hitstun is over
    // after getting hit into non-tumble knockback out of upB
    if *FIGHTER_STATUS_KIND_DAMAGE_AIR == status_kind
    && WorkModule::is_flag(boma, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION)
    && WorkModule::is_flag(boma, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END, false);
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    clown_cannon_shield_cancel(boma, status_kind, situation_kind, frame);
    // clown_cannon_dash_cancel(boma, status_kind, situation_kind, cat[0], frame);
    kart_jump_waveland(boma, status_kind, situation_kind, cat[0]);
    upB_kart_respawn(fighter, boma, status_kind);
}


#[utils::macros::opff(FIGHTER_KIND_KOOPAJR )]
pub fn koopajr_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		koopajr_frame(fighter)
    }
}

pub unsafe fn koopajr_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame( agent = WEAPON_KIND_KOOPAJR_REMAINCLOWN, main)]
pub fn koopajr_weapon_remainclown_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        if StatusModule::status_kind(boma) == *WEAPON_KOOPAJR_REMAINCLOWN_STATUS_KIND_FALL
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            StatusModule::change_status_request_from_script(boma, *WEAPON_KOOPAJR_REMAINCLOWN_STATUS_KIND_BURST, true);
        }
    }
}

pub fn install_remainclown() {
    smashline::install_agent_frame!(koopajr_weapon_remainclown_frame);
}

#[smashline::weapon_frame( agent = WEAPON_KIND_KOOPAJR_CANNONBALL, main )]
pub fn koopajr_weapon_frame_wrapper(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        koopajr_weapon_frame(weapon)
    }
}

pub unsafe fn koopajr_weapon_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    //if let Some(info) = WeaponFrameInfo::weapon_update_and_get(weapon) {
    //    
    //}
}