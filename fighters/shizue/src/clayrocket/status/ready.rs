use super::*;
use crate::globals::*;

// WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_READY

unsafe extern "C" fn ready_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("ready"), 0.0, 1.0, false, 0.0, false, false);
    VisibilityModule::set_int64(weapon.module_accessor, hash40("body") as i64, hash40("body_off") as i64);
    HitModule::set_status(weapon.module_accessor, 0, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    HitModule::set_status(weapon.module_accessor, 1, app::HitStatus(*HIT_STATUS_OFF), 0);
    weapon.fastshift(L2CValue::Ptr(ready_main_loop as *const () as _))
}

unsafe extern "C" fn ready_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let touch_normal = GroundModule::get_touch_normal(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let mut ground_angle = (touch_normal.y/touch_normal.x).atan().to_degrees();
    ground_angle -= 90.0;
    PostureModule::set_rot(
        weapon.module_accessor,
        &Vector3f{x: 0.0, y: ground_angle, z: 0.0},
        0
    );
    if !StatusModule::is_changing(weapon.module_accessor) {
        if WorkModule::is_flag(weapon.module_accessor, *WEAPON_SHIZUE_CLAYROCKET_STATUS_WORK_ID_FLAG_HIT) {
            LinkModule::send_event_parents(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new_raw(0x2477c8e319));
        }
        let damage = WorkModule::get_float(weapon.module_accessor, *WEAPON_SHIZUE_CLAYROCKET_INSTANCE_WORK_ID_FLOAT_DAMAGE);
        let life = WorkModule::get_param_float(weapon.module_accessor, hash40("param_clayrocket"), hash40("life_1"));
        if damage >= life {
            WorkModule::on_flag(weapon.module_accessor, *WEAPON_SHIZUE_CLAYROCKET_INSTANCE_WORK_ID_FLAG_BURST_DAMAGE);
            weapon.change_status(WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_BURST.into(), false.into());
            return 0.into();
        }
        let erase_time = WorkModule::get_param_int(weapon.module_accessor, hash40("param_clayrocket"), hash40("erase_time"));
        if weapon.global_table[CURRENT_FRAME].get_i32() >= erase_time
        || (weapon.global_table[CURRENT_FRAME].get_i32() > 1 && !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32))
        {
            weapon.change_status(WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_DISAPPEAR.into(), false.into());
            return 0.into();
        }
        let bury_offset_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_clayrocket"), hash40("burry_y"));
        let have_offset = Vector3f{x: 0.0, y: -bury_offset_y, z: 0.0};
        ModelModule::set_joint_translate(weapon.module_accessor, Hash40::new("have"), &have_offset, false, false);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_READY, ready_main);
}
