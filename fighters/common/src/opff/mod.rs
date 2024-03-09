use super::*;
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CFighterBase;

pub mod ledges;
pub mod physics;
pub mod tech;
pub mod tech_cleanup;
pub mod cancels;
pub mod var_resets;
pub mod gentleman;
pub mod momentum_transfer_line;
pub mod shotos;
//pub mod magic;
pub mod gimmick;
pub mod floats;
pub mod other;
pub mod fe;

use other::*;

/*
pub fn install() {
    // acmd::add_custom_hooks!(sys_line_system_control_fighter_hook);
    smashline::install_agent_frames!(sys_line_system_control_fighter_hook);
    smashline::install_agent_frames!(sys_line_system_control_hook);
    
}
*/

/*
This function runs exactly once per every fighter loaded into a match, every frame. I.E.  5 players in a match = 5 times per frame
Use this instead of get_command_flag_cat
*/

//      This is a special case function (I.E. don't use this as an exmaple for hooking).
//         It doesn't need a hook or return value because all that is handled in the ACMD crate.
// lol, lmao - blujay

// general per-frame fighter-level hooks
#[utils::export(common::opff)]
pub unsafe fn fighter_common_opff(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        let boma = &mut *info.boma;
        if boma.is_fighter() {
            moveset_edits(fighter, &info);
        }
    } else {
        panic!("Could not get the FrameInfo for this fighter! Is this even a fighter?")
    }
}

static mut IS_SALTY_INPUT: bool = false;

/// Performs salty runback check based off of the button input
/// This is to make it WiFi safe
unsafe fn salty_check(fighter: &mut L2CFighterCommon) -> bool {
    if IS_SALTY_INPUT {
        return false;
    }
    if fighter.is_button_on(Buttons::StockShare) {
        if fighter.is_button_on(Buttons::AttackRaw) && !fighter.is_button_on(!(Buttons::AttackRaw | Buttons::StockShare)) {
            app::FighterUtil::flash_eye_info(fighter.module_accessor);
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_assist_out"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.5, true, 0, 0, 0, 0, 0, false, false);
            utils::util::trigger_match_reset();
            utils::game_modes::signal_new_game();
            true
        } else if fighter.is_button_on(Buttons::SpecialRaw) && !fighter.is_button_on(!(Buttons::SpecialRaw | Buttons::StockShare)) {
            app::FighterUtil::flash_eye_info(fighter.module_accessor);
            if !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY]) {
                StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_DEAD, false);
            }
            utils::util::trigger_match_exit();
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub unsafe fn moveset_edits(fighter: &mut L2CFighterCommon, info: &FrameInfo) {
    let boma = &mut *info.boma;

    // allow ledge regrab iframes
    if WorkModule::get_int(fighter.boma(), *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT) < 3 {
        // indicate that your next ledge grab grab is capable of having iframes
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CLIFF_XLU);
    }

    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY])
    || !sv_information::is_ready_go()
    {
        IS_SALTY_INPUT = false;
    }

    // General Engine Edits
    if salty_check(fighter) {
        IS_SALTY_INPUT = true;
        return;
    }

    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    physics::run(fighter, info.lua_state, &mut *info.agent, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    tech::run(fighter, info.lua_state, &mut *info.agent, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing, info.frame);
    tech_cleanup::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing, info.frame);
    cancels::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    ledges::run(fighter, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    var_resets::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    gentleman::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    //magic::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    other::run(fighter, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    momentum_transfer_line::run(fighter, info.lua_state, &mut *info.agent, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    // Function hooks are in src/hooks/function_hooks

    // Character Moveset Changes
    // moveset_changes::run(boma, id, cat, status_kind, situation_kind, motion_kind, fighter_kind, stick_x, stick_y, facing, frame);
    floats::run(fighter, info.status_kind, info.situation_kind);
}

pub fn install() {
    // Reserved for common OPFF to be placed on exec status
    // rather than main status (default behavior)
    Agent::new("fighter")
        .on_line(Main, decrease_knockdown_bounce_heights)
        .on_line(Main, left_stick_flick_counter)
        .install();

}
