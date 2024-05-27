// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_ShieldBreakFly_Main)]
unsafe fn shield_break_fly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    HitModule::set_whole(fighter.module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, -0.05);

    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hooks!(
        shield_break_fly_main
    );
}