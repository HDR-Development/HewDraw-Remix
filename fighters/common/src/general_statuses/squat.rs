use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            fl_get_squat_walk_max_speed_hook,
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FL_get_squat_walk_max_speed)]
pub unsafe fn fl_get_squat_walk_max_speed_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let squat_walk_speed_max_mul = fighter.get_param_float("squat_walk_speed_max_mul", "");
    let squat_walk_speed_max = if fighter.is_status(*FIGHTER_STATUS_KIND_SQUAT_B) {
        fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_SQUAT_WALK_SPEED_BACK_MAX)
    } else if fighter.is_status(*FIGHTER_STATUS_KIND_SQUAT_F) {
        fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_SQUAT_WALK_SPEED_FORWARD_MAX)
    } else {
        0.0
    };

    let unique_speed_max_mul = if fighter.kind() == *FIGHTER_KIND_KOOPA { 0.75 } else { 1.0 };
    let speed_max = squat_walk_speed_max * squat_walk_speed_max_mul * unique_speed_max_mul;
    return speed_max.into();
}