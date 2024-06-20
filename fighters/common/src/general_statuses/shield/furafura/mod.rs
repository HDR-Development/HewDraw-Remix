use interpolation::Lerp;

use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_FuraFura)]
unsafe fn status_FuraFura(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("furafura"), 0.0, 1.0, false, 0.0, false, false);
    ControlModule::end_clatter_motion_rate(fighter.module_accessor);
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

    // let lerp_start = 0.0;
    // let lerp_end = 100.0;
    // let lerp_min = 1.0;
    // let lerp_max = 2.0;
    // let damage = DamageModule::damage(fighter.module_accessor, 0).clamp(lerp_start, lerp_end);
    // let ratio = (damage - lerp_start) / (lerp_end - lerp_start);
    // let end_mul = Lerp::lerp(&lerp_min, &lerp_max, &ratio);
    let end_frame = dbg!(fighter.get_param_float("common", "furafura_frame"));
    if fighter.status_frame() as f32 >= end_frame {
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
