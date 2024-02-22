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

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn mechakoopa_cooldown(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let cooldown_timer = VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER);
    let item_exists = ArticleModule::is_exist(boma, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA);
    let koopa_is_disabled = VarModule::is_flag(boma.object(), vars::koopajr::instance::DISABLE_MECHAKOOPA);
    let in_cooldown = VarModule::is_flag(boma.object(), vars::koopajr::instance::MECHAKOOPA_IS_COOLDOWN);

    // make sure disable flag is set if the koopa exists
    if item_exists && !koopa_is_disabled {
        VarModule::on_flag(boma.object(), vars::koopajr::instance::DISABLE_MECHAKOOPA);
    }
    // initiate cooldown once the koopa stops existing
    if !item_exists && !in_cooldown && koopa_is_disabled {
        VarModule::on_flag(boma.object(), vars::koopajr::instance::MECHAKOOPA_IS_COOLDOWN);
        VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 120);
    }
    // decrement cooldown timer when active
    if cooldown_timer > 0 {
        VarModule::dec_int(boma.object(), vars::common::instance::GIMMICK_TIMER);
    }
    // enable the koopa once the timer is over
    if cooldown_timer <= 0 && in_cooldown {
        VarModule::off_flag(boma.object(), vars::koopajr::instance::MECHAKOOPA_IS_COOLDOWN);
        VarModule::off_flag(boma.object(), vars::koopajr::instance::DISABLE_MECHAKOOPA);
        gimmick_flash(boma);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP,
        *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_HIT_WALL,
        *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_SPIN_TURN,
        *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT,
        *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_FALL,
        *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ATTACK
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

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    clown_cannon_shield_cancel(boma, status_kind, situation_kind, frame);
    // clown_cannon_dash_cancel(boma, status_kind, situation_kind, cat[0], frame);
    kart_jump_waveland(boma, status_kind, situation_kind, cat[0]);
    upB_kart_respawn(fighter, boma, status_kind);
    fastfall_specials(fighter);
    mechakoopa_cooldown(fighter, boma);
}

pub extern "C" fn koopajr_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
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

pub extern "C" fn koopajr_weapon_remainclown_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        if StatusModule::status_kind(boma) == *WEAPON_KOOPAJR_REMAINCLOWN_STATUS_KIND_FALL
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            StatusModule::change_status_request_from_script(boma, *WEAPON_KOOPAJR_REMAINCLOWN_STATUS_KIND_BURST, true);
        }
    }
}

pub extern "C" fn koopajr_weapon_frame_wrapper(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        koopajr_weapon_frame(weapon)
    }
}

pub unsafe fn koopajr_weapon_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    //if let Some(info) = WeaponFrameInfo::weapon_update_and_get(weapon) {
    //    
    //}
}

pub fn install() {
    smashline::Agent::new("koopajr")
        .on_line(Main, koopajr_frame_wrapper)
        .install();
    smashline::Agent::new("koopajr_remainclown")
        .on_line(Main, koopajr_weapon_remainclown_frame)
        .install();
    smashline::Agent::new("koopajr_cannonball")
        .on_line(Main, koopajr_weapon_frame_wrapper)
        .install();
}
