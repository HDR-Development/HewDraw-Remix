use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_status_end_EscaleFB_hook,
        );
    }
}

// This runs at the end of rolls
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_status_end_EscaleFB)]
pub unsafe fn sub_status_end_EscaleFB_hook(fighter: &mut L2CFighterCommon) {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
}