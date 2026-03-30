// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn clown_cannon_shield_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_HOLD) {
        if boma.status_frame() > 16 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if boma.is_situation(*SITUATION_KIND_GROUND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                }
            }
        }
    }
}

// Bowser Jr. Kart Jump Waveland
unsafe fn kart_jump_waveland(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP) {
        boma.check_airdodge_cancel();
    }
}

unsafe fn upB_kart_respawn(boma: &mut BattleObjectModuleAccessor) {
    // Respawns Clown Kart and allows actionability once hitstun is over
    // after getting hit into non-tumble knockback out of upB
    if boma.is_status(*FIGHTER_STATUS_KIND_DAMAGE_AIR)
    && WorkModule::is_flag(boma, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION)
    && WorkModule::is_flag(boma, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END, false);
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn mechakoopa_cooldown(boma: &mut BattleObjectModuleAccessor) {
    let item_exists = ArticleModule::is_exist(boma, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA);
    let koopa_is_disabled = VarModule::is_flag(boma.object(), vars::koopajr::instance::DISABLE_MECHAKOOPA);

    // make sure disable flag is set if the koopa exists
    if item_exists && !koopa_is_disabled {
        VarModule::on_flag(boma.object(), vars::koopajr::instance::DISABLE_MECHAKOOPA);
    }

    let in_cooldown = VarModule::is_flag(boma.object(), vars::koopajr::instance::MECHAKOOPA_COOLDOWN_ACTIVE);
    // initiate cooldown once the koopa stops existing
    if !item_exists && !in_cooldown && koopa_is_disabled {
        VarModule::on_flag(boma.object(), vars::koopajr::instance::MECHAKOOPA_COOLDOWN_ACTIVE);
        VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 120);
    }

    let cooldown_timer = VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER);
    // decrement cooldown timer when active
    if cooldown_timer > 0 {
        VarModule::dec_int(boma.object(), vars::common::instance::GIMMICK_TIMER);
    }
    // enable the koopa once the timer is over
    if cooldown_timer <= 0 && in_cooldown {
        VarModule::off_flag(boma.object(), vars::koopajr::instance::MECHAKOOPA_COOLDOWN_ACTIVE);
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
        *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_FALL,
        *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ATTACK
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    clown_cannon_shield_cancel(boma);
    kart_jump_waveland(boma);
    upB_kart_respawn(boma);
    fastfall_specials(fighter);
    mechakoopa_cooldown(boma);
}

pub extern "C" fn koopajr_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		koopajr_frame(fighter)
    }
}

pub unsafe fn koopajr_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, koopajr_frame_wrapper);
}