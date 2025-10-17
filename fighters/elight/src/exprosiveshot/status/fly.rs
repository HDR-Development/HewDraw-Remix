use super::*;

unsafe extern "C" fn fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = weapon.get_param_int("param_exprosiveshot", "life");
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    let speed = weapon.get_param_float("param_exprosiveshot", "speed");
    let angle = weapon.get_param_float("param_exprosiveshot", "angle");

    let rad = angle.to_radians();
    let cos = rad.cos();
    let sin = rad.sin();

    let lr = PostureModule::lr(weapon.module_accessor);

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed * cos * lr,
        speed * sin
    );

    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("fly"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    weapon.fastshift(L2CValue::Ptr(fly_fastshift as *const () as _))
}

unsafe extern "C" fn fly_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    weapon.set_float(speed_x, *WEAPON_ELIGHT_EXPROSIVESHOT_INSTANCE_WORK_ID_FLOAT_SPEED_X);
    weapon.set_float(speed_y, *WEAPON_ELIGHT_EXPROSIVESHOT_INSTANCE_WORK_ID_FLOAT_SPEED_Y);

    // if WorkModule::is_flag(weapon.module_accessor, *WEAPON_ELIGHT_EXPROSIVESHOT_INSTANCE_WORK_ID_FLAG_ATTACK) {
    //     weapon.change_status(WEAPON_ELIGHT_EXPROSIVESHOT_STATUS_KIND_BURST.into(), false.into());
    //     return 0.into();
    // }

    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        // notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.change_status(WEAPON_ELIGHT_EXPROSIVESHOT_STATUS_KIND_BURST.into(), false.into());
        return 0.into();
    }

    if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_ELIGHT_EXPROSIVESHOT_INSTANCE_WORK_ID_FLAG_HIT_WALL) {
        if weapon.sub_ground_module_is_touch_all_consider_speed().get_bool() {
            WorkModule::on_flag(weapon.module_accessor, *WEAPON_ELIGHT_EXPROSIVESHOT_INSTANCE_WORK_ID_FLAG_HIT_WALL);
            let status_frame = weapon.global_table[globals::CURRENT_FRAME].get_f32();
            if status_frame <= 1.0 {
                StopModule::set_other_stop(weapon.module_accessor, 2, StopOtherKind(0));
            }
            else {
                weapon.change_status(WEAPON_ELIGHT_EXPROSIVESHOT_STATUS_KIND_BURST.into(), false.into());
            }
        }
    }
    else {
        let status_frame = weapon.global_table[globals::CURRENT_FRAME].get_f32();
        if 2.0 <= status_frame {
            weapon.change_status(WEAPON_ELIGHT_EXPROSIVESHOT_STATUS_KIND_BURST.into(), false.into());
        }
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_ELIGHT_EXPROSIVESHOT_STATUS_KIND_FLY, fly_main);
}