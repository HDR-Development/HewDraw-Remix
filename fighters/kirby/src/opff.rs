// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

mod copy;

// symbol-based call for the pikachu/pichu characters' common opff
extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn final_cutter_landing_bugfix(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2)
    && MotionModule::frame(fighter.module_accessor) <= 2.0 {
        fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
    }
}

unsafe fn hammer_swing_drift_landcancel(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK) {
        let landing_lag = 19.0;
        if fighter.check_land_cancel(Some(landing_lag)) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }

    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK]) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

unsafe fn inhale_forced_end(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_FALL) {
        if fighter.is_prev_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SWALLOW) {
            // inhaled in midair
            if fighter.status_frame() >= 20 {
                fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_DRINK.into(), false.into());
            }
        }
        else {
            // inhaled then walked offstage
            if fighter.status_frame() >= 40 {
                fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_DRINK.into(), false.into());
            }
        }
    }
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_JUMP2) && fighter.status_frame() >= 80 {
        // inhaled then jumped offstage
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_DRINK.into(), false.into());
    }
}

// Adds landing detection during the Stone reappearance animation
// (missing from vanilla script)
unsafe fn down_special_reappear_proper_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_STONE_END)
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    let copystatus = StatusModule::status_kind(fighter.module_accessor);
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_DRINK,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_END,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SWALLOW,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_FALL,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_JUMP1,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_JUMP2,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_TURN_AIR,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_WAIT_FALL,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_WAIT_JUMP,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK,
            *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N3//because for some reason it doesn't work if its in the lua_consts range below
            ])
        || (0x206..0x377).contains(&copystatus) {
            fighter.sub_air_check_dive();
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon) {
    final_cutter_landing_bugfix(fighter);
    hammer_swing_drift_landcancel(fighter);
    inhale_forced_end(fighter);
    down_special_reappear_proper_landing(fighter);
    fastfall_specials(fighter);

    copy::kirby_copy_handler(fighter);
}

pub extern "C" fn kirby_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        kirby_frame(fighter)
    }
}

pub unsafe fn kirby_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    copy::install(agent);
    agent.on_line(Main, kirby_frame_wrapper);
}