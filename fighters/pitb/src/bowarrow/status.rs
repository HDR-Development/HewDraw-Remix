use super::*;

unsafe extern "C" fn fly_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.is_flag(*WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
        return false.into();
    }

    let (stick_mag, stick_rad) = weapon.stick_polar();
    let arrow_deg = weapon.get_float(*WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let was_reflected = AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR | *COLLISION_KIND_MASK_PARRY);

    let new_deg = if was_reflected { // when parried, snap the arrow to one of 8 directions
        AttackModule::clear_inflict_kind_status(weapon.module_accessor);
        (arrow_deg / 45.0).round() * 45.0
    }
    else if stick_mag > 1e-5 { // if stick is not neutral, control the arrow
        let mut delta = arrow_deg - stick_rad.to_degrees();
        while delta > 180.0  { delta -= 360.0; }
        while delta < -180.0 { delta += 360.0; }

        let control_angle = weapon.get_param_float("param_bowarrow", "control_angle");
        let max_turn = control_angle * stick_mag;
        arrow_deg - delta.clamp(-max_turn, max_turn)
    } else {
        arrow_deg
    };

    // apply new angle to velocity
    let speed = weapon.get_float(*WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_SPEED);
    let speed_x = speed * new_deg.to_radians().cos();
    let speed_y = speed * new_deg.to_radians().sin();
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    weapon.set_float(speed, *WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_SPEED);
    weapon.set_float(new_deg, *WEAPON_PIT_BOWARROW_INSTANCE_WORK_ID_FLOAT_ANGLE);
    PostureModule::set_rot(weapon.module_accessor, &Vector3f::new(-new_deg, 90.0, 0.0), 0);

    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *WEAPON_PIT_BOWARROW_STATUS_KIND_FLY, fly_exec);
}