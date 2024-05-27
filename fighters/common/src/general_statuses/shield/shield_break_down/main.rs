// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_ShieldBreakFly_Main)]
unsafe fn shield_break_down_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let vec = Vector3f{x: 0.0, y: 0.03, z: 0.0};
    KineticModule::add_speed(fighter.module_accessor, &vec);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.15);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.15);

    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.0
    );

    HitModule::set_whole(fighter.module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);

    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
        else {
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL.into(), true.into());
    } 

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hooks!(
        shield_break_down_main
    );
}