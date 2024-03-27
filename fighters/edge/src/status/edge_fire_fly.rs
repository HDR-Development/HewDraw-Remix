use super::*;
use globals::*;

unsafe extern "C" fn fly_s_main(fighter: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(fighter.module_accessor, hash40("param_fire"), hash40("life_s"));
    WorkModule::set_int(fighter.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n1"), 0.0, 1.0, false, 0.0, false, false);
    let owner_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let edge = utils::util::get_battle_object_from_id(owner_id);
    let stick_y = (&mut *(*edge).module_accessor).stick_y();
    let kirb = (&mut *(*edge).module_accessor).kind() == *FIGHTER_KIND_KIRBY;
    let facing = PostureModule::lr(fighter.module_accessor);
    let speed_x_stick_y_sub = if kirb { 0.3 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.speed_x_s_stick_y_sub") * stick_y.abs() };
    let speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_fire"), hash40("speed_x_s")) * facing;
    let accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_fire"), hash40("accel_x_s")) * facing;
    let max_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_fire"), hash40("max_speed_x_s")) - speed_x_stick_y_sub;
    let speed_y = if kirb { 0.01 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.speed_y_s") } * stick_y;
    let accel_y = if kirb { 1.0 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.accel_y_s") } * stick_y;
    let max_speed_y = if kirb { 0.6 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.max_speed_y_s") };
    sv_kinetic_energy!(set_speed, fighter, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy!(set_accel, fighter, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, accel_y);
    sv_kinetic_energy!(set_limit_speed, fighter, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x, max_speed_y);
    fighter.fastshift(L2CValue::Ptr(fly_s_main_loop as *const () as _))
}

unsafe extern "C" fn fly_s_main_loop(fighter: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly(fighter, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_S.into());
    return 0.into()
}

unsafe extern "C" fn fly_m_main(fighter: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(fighter.module_accessor, hash40("param_fire"), hash40("life_m"));
    WorkModule::set_int(fighter.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n2"), 0.0, 1.0, false, 0.0, false, false);
    let owner_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let edge = utils::util::get_battle_object_from_id(owner_id);
    let stick_y = (&mut *(*edge).module_accessor).stick_y();
    let kirb = (&mut *(*edge).module_accessor).kind() == *FIGHTER_KIND_KIRBY;
    let facing = PostureModule::lr(fighter.module_accessor);
    let speed_x_stick_y_sub = if kirb { 0.3 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.speed_x_m_stick_y_sub") } * stick_y.abs();
    let speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_fire"), hash40("speed_x_m")) * facing;
    let accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_fire"), hash40("accel_x_m")) * facing;
    let max_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_fire"), hash40("max_speed_x_m")) - speed_x_stick_y_sub;
    let speed_y = if kirb { 0.01 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.speed_y_m") } * stick_y;
    let accel_y = if kirb { 1.0 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.accel_y_m") } * stick_y;
    let max_speed_y = if kirb { 0.5 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.max_speed_y_m") };
    sv_kinetic_energy!(set_speed, fighter, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy!(set_accel, fighter, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, accel_y);
    sv_kinetic_energy!(set_limit_speed, fighter, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x, max_speed_y);
    fighter.fastshift(L2CValue::Ptr(fly_m_main_loop as *const () as _))
}

unsafe extern "C" fn fly_m_main_loop(fighter: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly(fighter, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_M.into());
    return 0.into()
}

unsafe extern "C" fn sub_fly(fighter: &mut L2CWeaponCommon, status: L2CValue) -> L2CValue {
    let life = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 || (WorkModule::is_flag(fighter.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_HIT_WALL) && fighter.status_frame() <= 2) {
        fighter.change_status(status, false.into());
        return 1.into()
    }
    else {
        if L2CWeaponCommon::sub_ground_module_is_touch_all_consider_speed(fighter).get_bool() {
            if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
                let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                let facing = PostureModule::lr(fighter.module_accessor);
                let accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_fire"), hash40("accel_x_m")) * facing;
                sv_kinetic_energy!(set_accel, fighter, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, 1.0);
                sv_kinetic_energy!(set_speed, fighter, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y * -1.0);
                let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                return 0.into()
            }
            WorkModule::on_flag(fighter.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_HIT_WALL);
            if fighter.status_frame() > 1 {
                fighter.change_status(status, false.into());
                return 1.into()
            }
            StopModule::set_other_stop(fighter.module_accessor, 2, StopOtherKind(0));
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_ATTACK) {
        return 0.into()
    }

    fighter.change_status(status, false.into());
    return 1.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, fly_s_main);
    agent.status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M, fly_m_main);
}