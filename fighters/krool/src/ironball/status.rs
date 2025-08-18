use super::*;
use globals::*;

unsafe extern "C" fn shoot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = weapon.get_param_int("param_ironball", "life");
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    let motion = if weapon.is_flag(*WEAPON_KROOL_IRONBALL_INSTANCE_WORK_ID_FLAG_SPIT) { Hash40::new("spit_shoot") } else { Hash40::new("shoot") };
    MotionModule::change_motion(weapon.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        shoot_substatus(weapon, false);
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(shoot_substatus as *const () as _));
    weapon.off_flag(*WEAPON_KROOL_IRONBALL_INSTANCE_WORK_ID_FLAG_HIT_CEIL);
    HitModule::set_whole(weapon.module_accessor, app::HitStatus(*HIT_STATUS_OFF), 0);
    weapon.fastshift(L2CValue::Ptr(shoot_main_loop as *const () as _))
}

unsafe extern "C" fn shoot_substatus(weapon: &mut L2CWeaponCommon, param_1: bool) -> L2CValue {
    if param_1 {
        weapon.dec_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    else {
        let sum_speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let facing = weapon.lr();
        if sum_speed_x * facing < 0.0 {
            sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        }
    }

    return 0.into();
}

unsafe extern "C" fn shoot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        weapon.change_status(WEAPON_KROOL_IRONBALL_STATUS_KIND_END.into(), false.into());
        return 1.into();
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
        weapon.on_flag(*WEAPON_KROOL_IRONBALL_INSTANCE_WORK_ID_FLAG_HOP);
        weapon.on_flag(*WEAPON_KROOL_IRONBALL_INSTANCE_WORK_ID_FLAG_HIT_CEIL);
        weapon.change_status(WEAPON_KROOL_IRONBALL_STATUS_KIND_HOP.into(), false.into());
        return 1.into();
    }
    else {
        if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            weapon.on_flag(*WEAPON_KROOL_IRONBALL_INSTANCE_WORK_ID_FLAG_HOP);
            weapon.change_status(WEAPON_KROOL_IRONBALL_STATUS_KIND_HOP.into(), false.into());
            return 1.into();
        }
        else {
            if GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
                weapon.on_flag(*WEAPON_KROOL_IRONBALL_INSTANCE_WORK_ID_FLAG_HOP);
                weapon.change_status(WEAPON_KROOL_IRONBALL_STATUS_KIND_HOP.into(), false.into());
                return 1.into();
            }
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_KROOL_IRONBALL_STATUS_KIND_SHOOT, shoot_main);
}