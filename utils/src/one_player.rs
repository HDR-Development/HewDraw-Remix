use skyline::hooks::InlineCtx;
use smash2::app::FighterManager;
use smash::app::{self, lua_bind::{PostureModule, WorkModule, FighterInformation}};
use smash::lib::lua_const::*;
use std::sync::atomic::{AtomicBool, Ordering};
use utils_dyn::util::MATCH_EXITING;

pub static IS_RULE_TIME: AtomicBool = AtomicBool::new(false);

#[skyline::hook(offset = 0x1b7b814, inline)]
unsafe fn match_load(ctx: &mut InlineCtx) {
    if !one_player_entry() {
        return;
    }

    let result_ptr = ctx.registers[22].x() as *const u64;
    let pane = *result_ptr.add(1);
    if pane == 0 {
        return;
    }

    let internal = *(pane as *const u64);
    if internal == 0 {
        return;
    }

    let parent = *((internal as *const u8).add(0x18) as *const u64);
    if parent == 0 {
        return;
    }

    // hide timer
    *(parent as *mut u8).add(0x58) &= 0xFE;
}

// Set the match timer to 99 minutes every frame in 1P mode so it never expires.
#[skyline::hook(offset = 0x15812b8, inline)]
unsafe fn once_per_frame(ctx: &mut InlineCtx) {
    if !one_player_entry() {
        return;
    }

    let match_state = ctx.registers[0].x() as *mut u8;
    // 99 minutes in frames
    *(match_state.add(0x3d4) as *mut i32) = 99 * 60 * 60;
}

// Prevent the match state from transitioning to the ending sequence in 1P mode,
// unless the player has lost all stocks.
#[skyline::hook(offset = 0x1585a5c, inline)]
unsafe fn prevent_match_end_transition(ctx: &mut InlineCtx) {
    if MATCH_EXITING.load(Ordering::Relaxed) || !one_player_entry() || is_last_stock() {
        return;
    }

    let base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
    let normal_update = base + 0x1581270;
    ctx.registers[8].set_x(normal_update);
}

// Prevent the game-over declaration from executing in 1P mode, unless the
// player has lost all stocks. The game calls this on every death in 1P.
// When stocks remain, block it until stock count hits 0.
#[skyline::hook(offset = 0x1585b20)]
unsafe fn game_over_declare(param_1: u64) {
    if !MATCH_EXITING.load(Ordering::Relaxed) && one_player_entry() && !is_last_stock() {
        return;
    }
    call_original!(param_1);
}

/// Returns true when the player's stock count has reached 0 (last stock lost).
unsafe fn is_last_stock() -> bool {
    if let Some(object) = crate::util::get_battle_object_from_entry_id(0) {
        let entry_id = WorkModule::get_int((*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let info = app::lua_bind::FighterManager::get_fighter_information(crate::singletons::FighterManager(), app::FighterEntryID(entry_id));
        if !info.is_null() {
            return FighterInformation::stock_count(info) == 0;
        }
    }
    false
}

pub fn one_player_entry() -> bool {
    if let Some(fighter_manager) = FighterManager::instance() {
        return fighter_manager.entry_count() == 1;
    }
    false
}

pub fn install() {
    skyline::install_hooks!(
        game_over_declare,
        prevent_match_end_transition,
        once_per_frame,
        match_load,
    );

    // Allow 0 CPUs in Training Mode menu
    // Allow UI to decrement to 0
    skyline::patching::Patch::in_text(0x1bb46a4).data(0xb907fa7fu32).unwrap();
    // Change set-value handler clamp to 0
    skyline::patching::Patch::in_text(0x1bbad14).data(0x7100011fu32).unwrap();
    // Fix clamp logic to clamp underflow to 0 instead of 1
    skyline::patching::Patch::in_text(0x1bbad18).data(0x1a9fa114u32).unwrap();
}
