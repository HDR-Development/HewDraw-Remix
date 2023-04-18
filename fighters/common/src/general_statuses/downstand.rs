use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_end_DownStandFb,
            bind_address_call_status_end_DownStandFb
        );
    }
}

// This runs at the end of getup rolls
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_DownStandFb)]
unsafe fn status_end_DownStandFb(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
    app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_bind_address_call_status_end_DownStandFb)]
unsafe fn bind_address_call_status_end_DownStandFb(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_DownStandFb();
    0.into()
}