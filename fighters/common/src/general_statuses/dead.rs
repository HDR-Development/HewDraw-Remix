// status imports
use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_dead_uniq_process_init_hook
        );
    }
}

// this runs as you are KO'd
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_dead_uniq_process_init)]
pub unsafe fn sub_dead_uniq_process_init_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Kill rage smoke gfx on star/screen KO
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_steam1"), true, true);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_steam2"), true, true);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_steam3"), true, true);

    if utils::game_modes::check_custom_mode(CustomMode::WarMode) {
        war_mode_award_stock(fighter);
    }

    original!()(fighter)
}

unsafe fn war_mode_award_stock(fighter: &mut L2CFighterCommon) {
    let dead_boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);

    if !VarModule::has_var_module((*dead_boma).object()) {
        return;
    }

    let attacker_entry_id = VarModule::get_int((*dead_boma).object(), vars::common::instance::LAST_ATTACKER_ENTRY_ID);
    if attacker_entry_id < 0 {
        return; // self-destruct or no attacker tracked
    }

    let dead_entry_id = WorkModule::get_int(dead_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    if attacker_entry_id == dead_entry_id {
        return; // don't award for self-KOs
    }

    let fighter_manager = crate::singletons::FighterManager();
    let attacker_info = app::lua_bind::FighterManager::get_fighter_information(
        fighter_manager,
        app::FighterEntryID(attacker_entry_id)
    );
    if attacker_info.is_null() {
        return;
    }

    // stock_count is at *(*(fighter_info + 8) + 0xd8)
    let inner = *((attacker_info as u64 + 8) as *const u64);
    let stock_ptr = (inner + 0xd8) as *mut u32;
    *stock_ptr += 1;
}
