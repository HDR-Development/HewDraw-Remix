use super::*;
use globals::*;

unsafe extern "C" fn check_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let palutena = utils::util::get_battle_object_from_id(owner_id);
    if (&mut *(*palutena).module_accessor).kind() == *FIGHTER_KIND_KIRBY {
        StatusModule::set_status_kind_interrupt(weapon.module_accessor, statuses::palutena_explosiveflame::CHECK_KIRBY);
        return 1.into();
    }
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

unsafe extern "C" fn check_kirby_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
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

unsafe extern "C" fn check_kirby_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.set_int(21, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("check"), 0.0, 1.0, false, 0.0, false, false);
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_object = utils::util::get_battle_object_from_id(owner_id);
    let offset_x = if VarModule::is_flag(owner_object, vars::kirby::status::PALUTENA_SPECIAL_N_HOLD) { 75.0 } else { 45.0 };
    PostureModule::set_pos(weapon.module_accessor, &Vector3f::new(pos_x + (offset_x * PostureModule::lr(weapon.module_accessor)), pos_y + 7.0, 0.0));
    if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_UP | *GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_UP_LEFT
    | *GROUND_TOUCH_FLAG_UP_RIGHT) as u32) {
        weapon.on_flag(*WEAPON_PALUTENA_EXPLOSIVEFLAME_INSTANCE_WORK_ID_FLAG_RESERVE_MISS);
    }
    // if weapon.is_flag(*WEAPON_PALUTENA_EXPLOSIVEFLAME_INSTANCE_WORK_ID_FLAG_RESERVE_MISS) {
    //     app::WeaponSpecializer_ExplosiveFlame::is_touch_down()
    // }

    weapon.shift(L2CValue::Ptr(check_kirby_main_loop as *const () as _))
}

unsafe extern "C" fn check_kirby_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("palutena_bullet"), true, true);
    weapon.dec_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if weapon.is_flag(*WEAPON_PALUTENA_EXPLOSIVEFLAME_INSTANCE_WORK_ID_FLAG_RESERVE_MISS) {
        weapon.change_status(WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_MISS.into(), false.into());
    }
    if weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        weapon.change_status(statuses::palutena_explosiveflame::EXPLODE_KIRBY.into(), false.into());
    }

    return 0.into();
}

unsafe extern "C" fn check_kirby_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn explode_kirby_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
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

unsafe extern "C" fn explode_kirby_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.set_int(36, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, false, 0.0, false, false);
    if StopModule::is_stop(weapon.module_accessor) {
        explode_kirby_main_substatus(weapon);
    }
    
    weapon.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(explode_kirby_main_substatus as *const () as _));
    weapon.fastshift(L2CValue::Ptr(explode_kirby_main_loop as *const () as _))
}

unsafe extern "C" fn explode_kirby_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn explode_kirby_main_substatus(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.dec_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    
    return 0.into();
}

unsafe extern "C" fn explode_kirby_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_CHECK, check_pre);

    agent.status(Pre, statuses::palutena_explosiveflame::CHECK_KIRBY, check_kirby_pre);
    agent.status(Main, statuses::palutena_explosiveflame::CHECK_KIRBY, check_kirby_main);
    agent.status(End, statuses::palutena_explosiveflame::CHECK_KIRBY, check_kirby_end);

    agent.status(Pre, statuses::palutena_explosiveflame::EXPLODE_KIRBY, explode_kirby_pre);
    agent.status(Main, statuses::palutena_explosiveflame::EXPLODE_KIRBY, explode_kirby_main);
    agent.status(End, statuses::palutena_explosiveflame::EXPLODE_KIRBY, explode_kirby_end);
}