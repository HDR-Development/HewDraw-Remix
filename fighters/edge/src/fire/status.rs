use super::*;
use globals::*;

unsafe extern "C" fn fly_s_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly_main(weapon, 0);
    weapon.fastshift(L2CValue::Ptr(fly_s_main_loop as *const () as _))
}

unsafe extern "C" fn fly_s_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly_main_loop(weapon, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_S.into());
    return 0.into()
}

unsafe extern "C" fn fly_m_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly_main(weapon, 1);
    weapon.fastshift(L2CValue::Ptr(fly_m_main_loop as *const () as _))
}

unsafe extern "C" fn fly_m_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly_main_loop(weapon, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_M.into());
    return 0.into()
}

unsafe extern "C" fn fly_l_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly_main(weapon, 2);
    weapon.fastshift(L2CValue::Ptr(fly_l_main_loop as *const () as _))
}

unsafe extern "C" fn fly_l_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sub_fly_main_loop(weapon, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_L.into());
    return 0.into()
}

unsafe extern "C" fn sub_fly_main(weapon: &mut L2CWeaponCommon, flare_type: i32) {
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

    fly_set_physics(weapon, flare_type);
    VarModule::off_flag(weapon.battle_object, vars::edge_fire::instance::REFINE);
    VarModule::off_flag(weapon.battle_object, vars::edge_fire::instance::REFLECT);

    if !StopModule::is_stop(weapon.module_accessor) {
        sub_fly_substatus(weapon, false.into());
    }
    weapon.global_table[globals::SUB_STATUS].assign(&L2CValue::Ptr(sub_fly_substatus as *const () as _));
}

unsafe extern "C" fn sub_fly_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        VarModule::countdown_int(weapon.battle_object, vars::edge_fire::instance::REFINE_COOLDOWN, 0);
    }
    0.into()
}

unsafe extern "C" fn sub_fly_main_loop(weapon: &mut L2CWeaponCommon, status: L2CValue) -> L2CValue {
    if (WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0)
    || (WorkModule::is_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_HIT_WALL)
    && weapon.status_frame() <= 2) {
        weapon.change_status(status, false.into());
        return 1.into()
    }
    else {
        if VarModule::is_flag(weapon.battle_object, vars::edge_fire::instance::REFINE) {
            VarModule::off_flag(weapon.battle_object, vars::edge_fire::instance::REFINE);
            if VarModule::get_int(weapon.battle_object, vars::edge_fire::instance::REFINE_COOLDOWN) == 0 {
                VarModule::set_int(weapon.battle_object, vars::edge_fire::instance::REFINE_COOLDOWN, 40);
                // let mut stick_x = weapon.global_table[STICK_X].get_f32();
                // if stick_x == 0.0 {
                //     let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
                //     let owner_object = utils::util::get_battle_object_from_id(owner_id);
                //     stick_x = (&mut *(*owner_object).module_accessor).stick_x();
                // };
                // if stick_x.abs() > 0.2 && stick_x.signum() != PostureModule::lr(weapon.module_accessor).signum() {
                //     EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_reflection"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.7, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
                //     VarModule::on_flag(weapon.battle_object, vars::edge_fire::instance::REFLECT);
                //     PostureModule::reverse_lr(weapon.module_accessor);
                // }
                EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_counteract_mark"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.7, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
                EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_just_shield_hit"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
                SoundModule::play_se(weapon.module_accessor, Hash40::new("se_item_badge_reflection"), true, false, false, false, app::enSEType(0));
                WorkModule::off_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_ATTACK);
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
        }
        if L2CWeaponCommon::sub_ground_module_is_touch_all_consider_speed(weapon).get_bool() {
            if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
                let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                let facing = PostureModule::lr(weapon.module_accessor);
                let accel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_m")) * facing;
                sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, 1.0);
                sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y * -1.0);
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
    if !weapon.is_status(*WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S)
    && WorkModule::is_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_ATTACK) {
        weapon.change_status(status, false.into());
        return 1.into();
    }
    
    return 0.into()
}

unsafe extern "C" fn fly_set_physics(weapon: &mut L2CWeaponCommon, flare_type: i32) {
    let facing = PostureModule::lr(weapon.module_accessor);
    let mut stick_y = weapon.global_table[STICK_Y].get_f32();
    // println!("weapon stick_y: {}", stick_y);
    if stick_y == 0.0 {
        // println!("fighter fallback");
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let owner_object = utils::util::get_battle_object_from_id(owner_id);
        stick_y = (&mut *(*owner_object).module_accessor).stick_y();
    };
    // println!("final stick_y: {}", stick_y);
    let is_reflect = VarModule::is_flag(weapon.battle_object, vars::edge_fire::instance::REFLECT);
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
            speed_y = 0.01 * stick_y;
        }
        let speed_x_stick_y_sub = 0.3 * stick_y.abs();
        let accel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_s")) * facing;
        let max_speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("max_speed_x_s")) - speed_x_stick_y_sub;
        let accel_y = 1.0 * stick_y;
        let max_speed_y = 0.6;
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
            speed_y = 0.01 * stick_y;
        }
        let speed_x_stick_y_sub = 0.3 * stick_y.abs();
        let accel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_m")) * facing;
        let max_speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("max_speed_x_m")) - speed_x_stick_y_sub;
        let accel_y = 1.0 * stick_y;
        let max_speed_y = 0.5;
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
            speed_y = 0.01 * stick_y;
        }
        let speed_x_stick_y_sub = 0.3 * stick_y.abs();
        let accel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_l")) * facing;
        let max_speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("max_speed_x_l")) - speed_x_stick_y_sub;
        let accel_y = 1.0 * stick_y;
        let max_speed_y = 0.5;
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, if is_reflect { max_speed_x * -1.0 } else { speed_x }, speed_y);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, accel_y);
        sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x, max_speed_y);
    }
}

unsafe extern "C" fn fly_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let status = weapon.global_table[STATUS_KIND].get_i32();
    if ![
        *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S,
        *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M,
        *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L,
    ].contains(&status) {
        VarModule::set_int(weapon.battle_object, vars::edge_fire::instance::REFINE_COOLDOWN, 0);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, fly_s_main);
    agent.status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, fly_end);

    agent.status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M, fly_m_main);
    agent.status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_M, fly_end);

    agent.status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L, fly_l_main);
    agent.status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_L, fly_end);
}