use super::*;

unsafe extern "C" fn shinkuhadoken_move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("move"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    weapon.fastshift(L2CValue::Ptr(shinkuhadoken_move_main_loop as *const () as _))
}

unsafe extern "C" fn shinkuhadoken_move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE, 0) {
        weapon.change_status(statuses::ryu_shinkuhadoken::FINISH.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn shinkuhadoken_finish_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn shinkuhadoken_finish_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_GAME, Hash40::new("game_finish"), -1);
    MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_EFFECT, Hash40::new("effect_finish"), -1);
    weapon.fastshift(L2CValue::Ptr(shinkuhadoken_finish_main_loop as *const () as _))
}

unsafe extern "C" fn shinkuhadoken_finish_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.global_table[0xE].get_f32() >= 5.0 {
        EffectModule::detach_all(weapon.module_accessor, 5);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_RYU_SHINKUHADOKEN_STATUS_KIND_MOVE, shinkuhadoken_move_main);

    agent.status(Pre, statuses::ryu_shinkuhadoken::FINISH, shinkuhadoken_finish_pre);
    agent.status(Main, statuses::ryu_shinkuhadoken::FINISH, shinkuhadoken_finish_main);
}