use super::*;
use globals::*;

unsafe extern "C" fn fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_flare1"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_EDGE_FLARE1_INSTANCE_WORK_ID_INT_LIFE);
    let mut vec = Vector2f::zero();
    let mut speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_flare1"), hash40("speed_x"));
    vec.x = speed_x;
    let speed_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_flare1"), hash40("speed_mul"));
    let ratio = WorkModule::get_float(weapon.module_accessor, *WEAPON_EDGE_FLARE1_INSTANCE_WORK_ID_FLOAT_RATIO);
    use interpolation::Lerp;
    let speed_lerp = Lerp::lerp(&1.0, &speed_mul, &ratio);
    vec.x *= speed_lerp;
    //let angle = WorkModule::get_float(weapon.module_accessor, *WEAPON_EDGE_FLARE1_INSTANCE_WORK_ID_FLOAT_ANGLE);
    //let speed_y = app::sv_math::vec2_rot(vec.x, vec.y, angle);
    // some sort of bitshift?
    let facing = PostureModule::lr(weapon.module_accessor);
    vec.x *= facing;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, vec.x, 0.0);
    sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, vec.x, 0.0);
    AttackModule::set_base_offset(weapon.module_accessor, &Vector2f::new(-vec.x, 0.0));
    weapon.fastshift(L2CValue::Ptr(fly_main_loop as *const () as _))
}

unsafe extern "C" fn fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if WorkModule::get_int(weapon.module_accessor, *WEAPON_EDGE_FLARE1_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        weapon.change_status(WEAPON_EDGE_FLARE1_STATUS_KIND_END.into(), false.into())
    }
    if weapon.sub_ground_module_is_touch_all_consider_speed().get_bool() {
        weapon.change_status(WEAPON_EDGE_FLARE1_STATUS_KIND_END.into(), false.into())
    }
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let edge = utils::util::get_battle_object_from_id(owner_id);
    VarModule::set_float(edge, vars::edge::instance::FLARE1_POS_X, PostureModule::pos_x(weapon.module_accessor));
    VarModule::set_float(edge, vars::edge::instance::FLARE2_POS_Y, PostureModule::pos_y(weapon.module_accessor));
    if VarModule::is_flag(edge, vars::edge::instance::FLASH_REFRACT)
    && !VarModule::is_flag(weapon.battle_object, vars::edge_flare1::status::REFRACTED) {
        EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_counteract_mark"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.7, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_muzzleflash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_item_badge_reflection"), true, false, false, false, app::enSEType(0));
        let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_EDGE_FLARE1_INSTANCE_WORK_ID_INT_LIFE);
        let max_life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_flare1"), hash40("life"));
        WorkModule::set_int(weapon.module_accessor, life + max_life, *WEAPON_EDGE_FLARE1_INSTANCE_WORK_ID_INT_LIFE);
        VarModule::on_flag(weapon.battle_object, vars::edge_flare1::status::REFRACTED);
        VarModule::off_flag(edge, vars::edge::instance::FLASH_REFRACT);
    }

    return 0.into()
}

unsafe extern "C" fn fly_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let edge = utils::util::get_battle_object_from_id(owner_id);
    if !VarModule::is_flag(edge, vars::edge::instance::FLASH_REFRACT) {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_EDGE_FLARE1_INSTANCE_WORK_ID_INT_LIFE);
    }
    return 0.into()
}

unsafe extern "C" fn fly_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let edge = utils::util::get_battle_object_from_id(owner_id);
    VarModule::set_int(edge, vars::edge::instance::FLARE1_ID, -1);
    VarModule::off_flag(edge, vars::edge::instance::FLASH_REFRACT);
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_EDGE_FLARE1_STATUS_KIND_FLY, fly_main);
    agent.status(Exec, *WEAPON_EDGE_FLARE1_STATUS_KIND_FLY, fly_exec);
    agent.status(End, *WEAPON_EDGE_FLARE1_STATUS_KIND_FLY, fly_end);
}