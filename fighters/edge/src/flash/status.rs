use super::*;
use globals::*;

unsafe extern "C" fn wait_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_direction"), true, true);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let edge = utils::util::get_battle_object_from_id(owner_id);
    GroundModule::set_collidable(weapon.module_accessor, false);
    if (&mut *(*edge).module_accessor).kind() != *FIGHTER_KIND_EDGE {
        StatusModule::change_status_force(weapon.module_accessor, statuses::edge_flash::BURST, false);
        return 1.into()
    }
    let life = ParamModule::get_int(edge, ParamType::Agent, "param_flash.life");
    VarModule::set_int(weapon.battle_object, vars::edge_flash::status::LIFE, life);
    if VarModule::is_flag(edge, vars::edge::status::SPECIAL_LW_HOLD) {
        let pos_x = PostureModule::pos_x(weapon.module_accessor);
        let pos_y = PostureModule::pos_y(weapon.module_accessor);
        let offset_x = ParamModule::get_float(edge, ParamType::Agent, "param_flash.hold_offset_x");
        PostureModule::set_pos(weapon.module_accessor, &Vector3f::new(pos_x + (offset_x * PostureModule::lr(weapon.module_accessor)), pos_y, 0.0));
    }

    weapon.fastshift(L2CValue::Ptr(wait_main_loop as *const () as _))
}

unsafe extern "C" fn wait_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let edge = utils::util::get_battle_object_from_id(owner_id);
    let edge_boma = &mut *(*edge).module_accessor;
    if VarModule::get_int(weapon.battle_object, vars::edge_flash::status::LIFE) <= 0 {
        StatusModule::change_status_force(weapon.module_accessor, statuses::edge_flash::VANISH, false);
        return 1.into()
    }

    return 0.into();
}

unsafe extern "C" fn wait_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    VarModule::dec_int(weapon.battle_object, vars::edge_flash::status::LIFE);

    return 0.into();
}

unsafe extern "C" fn burst_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("edge_senkou_shield"), false, false);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_direction"), false, false);
    smashline::original_status(Pre, weapon, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK)(weapon)
}

unsafe extern "C" fn burst_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("edge_senkou_shield_break"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.6, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
    smashline::original_status(Main, weapon, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK)(weapon)
}

unsafe extern "C" fn burst_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    smashline::original_status(Exec, weapon, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK)(weapon)
}

unsafe extern "C" fn burst_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    smashline::original_status(End, weapon, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK)(weapon)
}

unsafe extern "C" fn vanish_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("edge_senkou_shield"), false, false);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_direction"), false, false);
    EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_erace_smoke"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        0,
        app::GroundCliffCheckKind(*GROUND_CORRECT_KIND_AIR),
        false,
        0,
        0,
        0,
        0
    );
    
    return 0.into();
}

unsafe extern "C" fn vanish_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("vanish"), 0.0, 1.0, false, 0.0, false, false);
    return 0.into();
}

unsafe extern "C" fn vanish_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK, wait_main);
    agent.status(Exec, *WEAPON_EDGE_FLASH_STATUS_KIND_ATTACK, wait_exec);

    agent.status(Pre, statuses::edge_flash::BURST, burst_pre);
    agent.status(Main, statuses::edge_flash::BURST, burst_main);
    agent.status(Exec, statuses::edge_flash::BURST, burst_exec);
    agent.status(End, statuses::edge_flash::BURST, burst_end);

    agent.status(Pre, statuses::edge_flash::VANISH, vanish_pre);
    agent.status(Main, statuses::edge_flash::VANISH, vanish_main);
    agent.status(End, statuses::edge_flash::VANISH, vanish_end);
}