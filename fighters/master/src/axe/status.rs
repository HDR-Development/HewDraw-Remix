use super::*;
use globals::*;

unsafe extern "C" fn special_lw_yeet_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );

    0.into()
}

unsafe extern "C" fn special_lw_yeet_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_lw_yeet"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(special_lw_yeet_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_yeet_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    VarModule::set_int(weapon.battle_object, vars::master_axe::status::LIFE, 90);
    let facing = PostureModule::lr(weapon.module_accessor);
    sv_kinetic_energy!(reset_energy, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 1.0 * facing, 0.0, 0.0);

    return 0.into();
}

unsafe extern "C" fn special_lw_yeet_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = VarModule::get_int(weapon.battle_object, vars::master_axe::status::LIFE);
    if life <= 0 || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    VarModule::dec_int(weapon.battle_object, vars::master_axe::status::LIFE);

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::master_axe::SPECIAL_LW_YEET, special_lw_yeet_pre);
    agent.status(Main, statuses::master_axe::SPECIAL_LW_YEET, special_lw_yeet_main);
    agent.status(Init, statuses::master_axe::SPECIAL_LW_YEET, special_lw_yeet_init);
}