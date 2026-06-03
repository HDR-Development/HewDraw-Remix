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
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        ControlModule::clear_command(fighter.module_accessor, true);
        return true.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("furafura"), 0.0, 1.0, false, 0.0, false, false);
    }

    let lerp_start = 25.0_f64;
    let lerp_end = 125.0_f64;
    let lerp_min = 1.0_f64;
    let lerp_max = 5.0_f64 / 3.0_f64;
    let damage = DamageModule::damage(fighter.module_accessor, 0) as f64;
    let lerp_scalar = (damage - lerp_start) / (lerp_end - lerp_start);
    let end_mul = lerp_min.lerp(&lerp_max, &lerp_scalar).clamp(lerp_min, lerp_max);
    let end_frame = fighter.get_param_float("common", "furafura_frame") as f64;
    let motion_rate = 1.5 / end_mul;
    MotionModule::set_rate(fighter.module_accessor, motion_rate as f32);
    if fighter.status_frame() as f64 >= end_frame * end_mul {
        fighter.change_status(FIGHTER_STATUS_KIND_FURAFURA_END.into(), false.into());
        return false.into();
    }

    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_status_FuraFuraEnd)]
unsafe fn status_FuraFuraEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut rate = 1.0;
    // Separate Disable/Bind Wakeup Framedata
    if StatusModule::prev_status_kind(fighter.module_accessor, 2) != *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY {
        let disable_frame = ParamModule::get_float(fighter.battle_object, ParamType::Common, "bind_end_frame") - 1.0; // FAF not lag
        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("furafura_end"), true);
        let end_frame = MotionModule::end_frame(fighter.module_accessor);
        rate = if cancel_frame > 0.0 {cancel_frame/disable_frame} else {end_frame/disable_frame};
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("furafura_end"), 0.0, rate, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_FuraFuraEnd_Main as *const () as _))
}

pub fn install() {
    skyline::install_hooks!(
        status_FuraFura,
        status_FuraFura_Main,
        status_FuraFuraEnd_Main
    );
}
