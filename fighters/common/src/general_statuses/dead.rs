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
    original!()(fighter)
}