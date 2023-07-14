// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
pub unsafe fn missile_land_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// Shinespark charge
unsafe fn shinespark_charge(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if [*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_RUN].contains(&status_kind) && frame > 31.0 {
        if  !VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_READY) {
            VarModule::on_flag(boma.object(), vars::samus::instance::SHINESPARK_READY);
            gimmick_flash(boma);
        }
    }
}

// Shinespark Reset
unsafe fn shinespark_reset(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    if ![*FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_TURN_RUN,
        *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status_kind) {
            VarModule::off_flag(boma.object(), vars::samus::instance::SHINESPARK_READY);
            VarModule::off_flag(boma.object(), vars::samus::instance::SHINESPARK_USED);
    }
}

// Morph Ball Crawl
// PUBLIC
pub unsafe fn morphball_crawl(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW].contains(&status_kind) {
        if frame >= 32.0 {
            if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
                && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 12.0, 1.0, 1.0);
            }
        }
    }
}

pub unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "Rust" fn common_samus(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        missile_land_cancel(fighter, &mut *info.boma, info.id, info.status_kind, info.situation_kind);
        morphball_crawl(&mut *info.boma, info.status_kind, info.frame);
        nspecial_cancels(&mut *info.boma, info.status_kind, info.situation_kind);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A,
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    shinespark_charge(boma, id, status_kind, frame);
    shinespark_reset(boma, id, status_kind);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_SAMUS )]
pub fn samus_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		samus_frame(fighter);
        common_samus(fighter);
    }
}

pub unsafe fn samus_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}