// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_ShieldBreakFall_Main)]
unsafe fn shield_break_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.18);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0);

    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.sub_wait_ground_check_common(false.into());
        fighter.change_status(FIGHTER_STATUS_KIND_SLEEP_END.into(), true.into());
    } else {
        fighter.sub_air_check_fall_common();
    }
    L2CValue::I32(0)

}

pub fn install() {
    skyline::install_hooks!(
        shield_break_fall_main
    );
}