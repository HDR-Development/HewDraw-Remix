use super::*;
use globals::*;

unsafe extern "C" fn fly_s_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly_main(weapon, 0);
    weapon.fastshift(L2CValue::Ptr(fly_s_main_loop as *const () as _))
}

unsafe extern "C" fn fly_s_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly_main_loop(weapon, 0, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_S.into());
    return 0.into()
}

unsafe extern "C" fn fly_m_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly_main(weapon, 1);
    weapon.fastshift(L2CValue::Ptr(fly_m_main_loop as *const () as _))
}

unsafe extern "C" fn fly_m_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly_main_loop(weapon, 1, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_M.into());
    return 0.into()
}

unsafe extern "C" fn fly_l_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly_main(weapon, 2);
    weapon.fastshift(L2CValue::Ptr(fly_l_main_loop as *const () as _))
}

unsafe extern "C" fn fly_l_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly_main_loop(weapon, 0, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_L.into());
    return 0.into()
}

unsafe extern "C" fn sub_fly_main(weapon: &mut L2CWeaponCommon, flare_type: i32) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let edge = utils::util::get_battle_object_from_id(owner_id);
    let life = match flare_type {
        0 => WorkModule::get_param_int(weapon.module_accessor, hash40("param_fire"), hash40("life_s")),
        1 => WorkModule::get_param_int(weapon.module_accessor, hash40("param_fire"), hash40("life_m")),
        2 => WorkModule::get_param_int(weapon.module_accessor, hash40("param_fire"), hash40("life_l")),
        _ => WorkModule::get_param_int(weapon.module_accessor, hash40("param_fire"), hash40("life_s"))
    };
    let motion = match flare_type {
        0 => Hash40::new("special_n1"),
        1 => Hash40::new("special_n2"),
        2 => Hash40::new("special_n3"),
        _ => Hash40::new("special_n1")
    };
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    if (&mut *(*edge).module_accessor).kind() == *FIGHTER_KIND_EDGE {
        VarModule::set_int(edge, vars::edge::instance::FIRE_ID, 0);
        fly_set_physics(weapon, flare_type);
        VarModule::off_flag(edge, vars::edge::instance::FLASH_REFINE);
        VarModule::off_flag(weapon.battle_object, vars::edge_fire::instance::REFLECT);
    }
    else {
        fly_set_physics(weapon, flare_type);
    }
}

unsafe extern "C" fn sub_fly_main_loop(weapon: &mut L2CWeaponCommon, flare_type: i32, status: L2CValue) -> L2CValue {
    if (WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0)
    || (WorkModule::is_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_HIT_WALL)
    && weapon.status_frame() <= 2) {
        println!("1");
        weapon.change_status(status, false.into());
        return 1.into()
    }
    else {
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let edge = utils::util::get_battle_object_from_id(owner_id);
        VarModule::set_float(edge, vars::edge::instance::FIRE_POS_X, PostureModule::pos_x(weapon.module_accessor));
        VarModule::set_float(edge, vars::edge::instance::FIRE_POS_Y, PostureModule::pos_y(weapon.module_accessor));
        if VarModule::is_flag(edge, vars::edge::instance::FLASH_REFINE) {
            // let stick_x = ControlModule::get_stick_x(&mut *(*edge).module_accessor);
            // if stick_x.abs() > 0.2 && stick_x.signum() != PostureModule::lr(weapon.module_accessor).signum() {
            //     EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_reflection"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.7, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
            //     VarModule::on_flag(weapon.battle_object, vars::edge_fire::instance::REFLECT);
            //     PostureModule::reverse_lr(weapon.module_accessor);
            // }
            EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_counteract_mark"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.7, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
            EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_just_shield_hit"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
            SoundModule::play_se(weapon.module_accessor, Hash40::new("se_item_badge_reflection"), true, false, false, false, app::enSEType(0));
            if weapon.is_status(*WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S) {
                SoundModule::play_se(weapon.module_accessor, Hash40::new("se_item_crossbomb_blink"), true, false, false, false, app::enSEType(0));
                weapon.change_status(WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M.into(), false.into());
                return 1.into()
            }
            else if weapon.is_status(*WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M) || weapon.is_status(*WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L) {
                SoundModule::play_se(weapon.module_accessor, Hash40::new("se_gohoubi_bounus_add"), true, false, false, false, app::enSEType(0));
                weapon.change_status(WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L.into(), false.into());
                return 1.into()
            }
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
                println!("2");
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

unsafe extern "C" fn fly_set_physics(weapon: &mut L2CWeaponCommon, flare_type: i32) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let edge = utils::util::get_battle_object_from_id(owner_id);
    let kirb = (&mut *(*edge).module_accessor).kind() != *FIGHTER_KIND_EDGE;
    let facing = PostureModule::lr(weapon.module_accessor);
    let is_init = if kirb { true } else { !VarModule::is_flag(edge, vars::edge::instance::FLASH_REFINE) };
    let stick_y = if is_init { (&mut *(*edge).module_accessor).stick_y() } else { VarModule::get_float(weapon.battle_object, vars::edge_fire::status::STICK_Y) };
    if is_init { VarModule::set_float(weapon.battle_object, vars::edge_fire::status::STICK_Y, stick_y); }
    let is_reflect = if !kirb { VarModule::is_flag(weapon.battle_object, vars::edge_fire::instance::REFLECT) } else { false };
    let speed_x;
    let speed_y;

    // Flare
    if flare_type == 0 {
        if is_reflect {
            speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
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
            speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
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
        if is_reflect {
            speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
        else {
            speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("speed_x_l")) * facing;
            speed_y = if kirb { 0.01 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.speed_y_l") } * stick_y;
        }
        let speed_x_stick_y_sub = if kirb { 0.3 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.speed_x_l_stick_y_sub") } * stick_y.abs();
        let accel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_l")) * facing;
        let max_speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("max_speed_x_l")) - speed_x_stick_y_sub;
        let accel_y = if kirb { 1.0 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.accel_y_l") } * stick_y;
        let max_speed_y = if kirb { 0.5 } else { ParamModule::get_float(edge, ParamType::Agent, "param_fire.max_speed_y_l") };
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, if is_reflect { max_speed_x * -1.0 } else { speed_x }, speed_y);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, accel_y);
        sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x, max_speed_y);
    }
}

unsafe extern "C" fn fly_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let edge = utils::util::get_battle_object_from_id(owner_id);
    VarModule::set_int(edge, vars::edge::instance::FIRE_ID, -1);
    VarModule::off_flag(edge, vars::edge::instance::FLASH_REFINE);
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, fly_s_main);
    agent.status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, fly_end);

    agent.status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M, fly_m_main);
    agent.status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M, fly_end);

    agent.status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L, fly_l_main);
    agent.status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L, fly_end);
}