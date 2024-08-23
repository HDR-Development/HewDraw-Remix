use super::*;


pub unsafe extern "C" fn regular_pre(weapon: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}

pub unsafe extern "C" fn regular_main(weapon: &mut L2CFighterCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_firebreath"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let packun = utils::util::get_battle_object_from_id(owner_id);
        if (&mut *(*packun).module_accessor).kind() == *FIGHTER_KIND_PACKUN {
            VarModule::set_float(packun, vars::packun::instance::FIRE_POS_X, PostureModule::pos_x(weapon.module_accessor));
            VarModule::set_float(packun, vars::packun::instance::FIRE_POS_Y, PostureModule::pos_y(weapon.module_accessor));
        }
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(regular_main_loop as *const () as _))
}

pub unsafe extern "C" fn regular_main_loop(weapon: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) > 0 {
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let packun = utils::util::get_battle_object_from_id(owner_id);
        if (&mut *(*packun).module_accessor).kind() == *FIGHTER_KIND_PACKUN {
            VarModule::set_float(packun, vars::packun::instance::FIRE_POS_X, PostureModule::pos_x(weapon.module_accessor));
            VarModule::set_float(packun, vars::packun::instance::FIRE_POS_Y, PostureModule::pos_y(weapon.module_accessor));
        }
        if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
            app::lua_bind::MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_EFFECT, Hash40::new("effect_bound"), -1);
            if !weapon.pop_lua_stack(1).get_bool() {
                notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            }
        }
    }
    else {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }

    return 0.into();
}

pub unsafe extern "C" fn regular_exec(weapon: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

pub unsafe extern "C" fn regular_end(weapon: &mut L2CFighterCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let packun = utils::util::get_battle_object_from_id(owner_id);
    if (&mut *(*packun).module_accessor).kind() == *FIGHTER_KIND_PACKUN {
        VarModule::set_float(packun, vars::packun::instance::FIRE_POS_X, 0.0);
        VarModule::set_float(packun, vars::packun::instance::FIRE_POS_Y, 0.0);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, WEAPON_PACKUN_FIREBREATH_STATUS_KIND_REGULAR, regular_pre);
    agent.status(Main, WEAPON_PACKUN_FIREBREATH_STATUS_KIND_REGULAR, regular_main);
    agent.status(Exec, WEAPON_PACKUN_FIREBREATH_STATUS_KIND_REGULAR, regular_exec);
    agent.status(End, WEAPON_PACKUN_FIREBREATH_STATUS_KIND_REGULAR, regular_end);
}