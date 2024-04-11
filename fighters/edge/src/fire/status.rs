use super::*;
use globals::*;

unsafe extern "C" fn fly_s_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_fire"), hash40("life_s"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_n1"), 0.0, 1.0, false, 0.0, false, false);
    fly_set_physics(weapon, 0, true, false, 0.0, 0.0);
    weapon.fastshift(L2CValue::Ptr(fly_s_main_loop as *const () as _))
}

unsafe extern "C" fn fly_s_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly(weapon, 0, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_S.into());
    return 0.into()
}

unsafe extern "C" fn fly_m_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_fire"), hash40("life_m"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_n2"), 0.0, 1.0, false, 0.0, false, false);
    fly_set_physics(weapon, 1, true, false, 0.0, 0.0);
    weapon.fastshift(L2CValue::Ptr(fly_m_main_loop as *const () as _))
}

unsafe extern "C" fn fly_m_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly(weapon, 1, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_M.into());
    return 0.into()
}

unsafe extern "C" fn sub_fly(weapon: &mut L2CWeaponCommon, flare_type: i32, status: L2CValue) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 || (WorkModule::is_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_HIT_WALL) && weapon.status_frame() <= 2) {
        weapon.change_status(status, false.into());
        return 1.into()
    }
    else {
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let edge = utils::util::get_battle_object_from_id(owner_id);
        VarModule::set_float(edge, vars::edge::instance::FIRE_POS_X, PostureModule::pos_x(weapon.module_accessor));
        VarModule::set_float(edge, vars::edge::instance::FIRE_POS_Y, PostureModule::pos_y(weapon.module_accessor));
        if VarModule::is_flag(edge, vars::edge::instance::FLASH_REFLECT) {
            EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_reflection"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
            SoundModule::play_se(weapon.module_accessor, Hash40::new("se_item_badge_reflection"), true, false, false, false, app::enSEType(0));
            let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            PostureModule::reverse_lr(weapon.module_accessor);
            fly_set_physics(weapon, flare_type, false, true, speed_x, speed_y);
            VarModule::off_flag(edge, vars::edge::instance::FLASH_REFLECT);
            return 0.into()
        }
        if L2CWeaponCommon::sub_ground_module_is_touch_all_consider_speed(weapon).get_bool() {
            if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
                let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                let facing = PostureModule::lr(weapon.module_accessor);
                let accel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_m")) * facing;
                sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, 1.0);
                sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y * -1.0);
                let stick_y = VarModule::get_float(weapon.battle_object, vars::edge_fire::status::STICK_Y);
                VarModule::set_float(weapon.battle_object, vars::edge_fire::status::STICK_Y, stick_y.abs());
                return 0.into()
            }
            WorkModule::on_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_HIT_WALL);
            if weapon.status_frame() > 1 {
                weapon.change_status(status, false.into());
                return 1.into()
            }
            StopModule::set_other_stop(weapon.module_accessor, 2, StopOtherKind(0));
        }
    }
    if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_ATTACK) {
        return 0.into()
    }

    weapon.change_status(status, false.into());
    return 1.into()
}

unsafe extern "C" fn fly_set_physics(weapon: &mut L2CWeaponCommon, flare_type: i32, is_init: bool, is_reflect: bool, reflect_speed_x: f32, reflect_speed_y: f32) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let edge = utils::util::get_battle_object_from_id(owner_id);
    let kirb = (&mut *(*edge).module_accessor).kind() == *FIGHTER_KIND_KIRBY;
    let facing = PostureModule::lr(weapon.module_accessor);
    let stick_y = if is_init { (&mut *(*edge).module_accessor).stick_y() } else { VarModule::get_float(weapon.battle_object, vars::edge_fire::status::STICK_Y) };
    if is_init { VarModule::set_float(weapon.battle_object, vars::edge_fire::status::STICK_Y, stick_y); }
    let speed_x;
    let speed_y;

    // Flare
    if flare_type == 0 {
        if is_reflect {
            speed_x = reflect_speed_x;
            speed_y = reflect_speed_y;
        }
        else {
            speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("speed_x_s")) * facing;
            speed_y = if kirb { 0.01 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.speed_y_s") } * stick_y;
        }
        let speed_x_stick_y_sub = if kirb { 0.3 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.speed_x_s_stick_y_sub") * stick_y.abs() };
        let accel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_s")) * facing;
        let max_speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("max_speed_x_s")) - speed_x_stick_y_sub;
        let accel_y = if kirb { 1.0 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.accel_y_s") } * stick_y;
        let max_speed_y = if kirb { 0.6 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.max_speed_y_s") };
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, if is_reflect { max_speed_x * -1.0 } else { speed_x }, speed_y);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, accel_y);
        sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x, max_speed_y);
    }
    // Megaflare
    else if flare_type == 1 {
        if is_reflect {
            speed_x = reflect_speed_x;
            speed_y = reflect_speed_y;
        }
        else {
            speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("speed_x_m")) * facing;
            speed_y = if kirb { 0.01 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.speed_y_m") } * stick_y;
        }
        let speed_x_stick_y_sub = if kirb { 0.3 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.speed_x_m_stick_y_sub") } * stick_y.abs();
        let accel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_m")) * facing;
        let max_speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("max_speed_x_m")) - speed_x_stick_y_sub;
        let accel_y = if kirb { 1.0 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.accel_y_m") } * stick_y;
        let max_speed_y = if kirb { 0.5 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.max_speed_y_m") };
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, if is_reflect { max_speed_x * -1.0 } else { speed_x }, speed_y);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, accel_y);
        sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x, max_speed_y);
    }
    // Gigaflare
    else {

    }
}

unsafe extern "C" fn fly_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let edge = utils::util::get_battle_object_from_id(owner_id);
    VarModule::set_int(edge, vars::edge::instance::FIRE_ID, -1);
    VarModule::off_flag(edge, vars::edge::instance::FLASH_REFLECT);
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, fly_s_main);
    agent.status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, fly_end);

    agent.status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M, fly_m_main);
    agent.status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M, fly_end);

    //agent.status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L, fly_l_main);
    agent.status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L, fly_end);
}