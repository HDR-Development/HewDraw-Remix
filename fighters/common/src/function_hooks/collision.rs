use super::*;
use globals::*;

// This prevents projectiles from dying on platforms/ground
#[skyline::hook(replace=GroundModule::is_touch)]
unsafe fn is_touch_hook(boma: &mut BattleObjectModuleAccessor, ground_touch_flags: u32) -> bool {
    let mut ground_touch_flags = ground_touch_flags;
    let normal_y = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_DOWN as u32);
    let y_vel = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);

    if boma.is_weapon()
    && (normal_y >= 0.99 && normal_y <= 1.01)  // if touching a near-flat platform/ground
    && y_vel >= 0.0  // if not moving downwards
    {
        if ground_touch_flags == *GROUND_TOUCH_FLAG_ALL as u32
        && !original!()(boma, *GROUND_TOUCH_FLAG_LEFT as u32 | *GROUND_TOUCH_FLAG_RIGHT as u32)  // if not touching a wall
        {
            // Ignore ground collision
            return false;
        }
        if ground_touch_flags & *GROUND_TOUCH_FLAG_DOWN as u32 != 0 {
            // Ignore ground collision
            ground_touch_flags -= *GROUND_TOUCH_FLAG_DOWN as u32;
        }
    }
    original!()(boma, ground_touch_flags)
}

pub fn install() {
    skyline::install_hooks!(
        is_touch_hook
    );
}