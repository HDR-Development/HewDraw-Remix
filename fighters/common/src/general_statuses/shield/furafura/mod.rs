use interpolation::Lerp;

use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_FuraFura)]
unsafe fn status_FuraFura(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("furafura"), 0.0, 1.0, false, 0.0, false, false);
    ControlModule::end_clatter_motion_rate(fighter.module_accessor);
    ControlModule::end_clatter(fighter.module_accessor, 0);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_FuraFura_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_FuraFura_Main)]
unsafe fn status_FuraFura_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("furafura"), 0.0, 1.0, false, 0.0, false, false);
    }

    let lerp_start = 25.0_f64;
    let lerp_end = 125.0_f64;
    let lerp_min = 1.0_f64;
    let lerp_max = 2.0_f64;
    let damage = DamageModule::damage(fighter.module_accessor, 0) as f64;
    let lerp_scalar = (damage - lerp_start) / (lerp_end - lerp_start);
    let end_mul = dbg!(lerp_min.lerp(&lerp_max, &lerp_scalar).clamp(lerp_min, lerp_max));
    let end_frame = fighter.get_param_float("common", "furafura_frame") as f64;
    if fighter.status_frame() as f64 >= end_frame * end_mul {
        fighter.change_status(FIGHTER_STATUS_KIND_FURAFURA_END.into(), false.into());
        return false.into();
    }

    return false.into();
}

pub fn install() {
    skyline::install_hooks!(
        status_FuraFura,
        status_FuraFura_Main
    );
}
