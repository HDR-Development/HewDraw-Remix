use super::*;
use globals::*;
// status script import

unsafe extern "C" fn fly_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = weapon.get_owner_boma();
    if VarModule::is_flag(owner_boma.object(), vars::miiswordsman::status::SPECIAL_LW2_CHANGE_ARTICLE) {
        StatusModule::set_status_kind_interrupt(weapon.module_accessor, statuses::miiswordsman_tornadoshot::SHOCK_SPELL);
        return 1.into();
    }

    return smashline::original_status(Pre, weapon, *WEAPON_MIISWORDSMAN_TORNADOSHOT_STATUS_KIND_FLY)(weapon);
}

unsafe extern "C" fn shock_spell_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    
    return 0.into();
}

unsafe extern "C" fn shock_spell_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.set_int(50, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    weapon.set_int(50, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shock_spell"), 0.0, 1.0, false, 0.0, false, false);
    let owner_boma = weapon.get_owner_boma();
    let offset_x;
    if VarModule::is_flag(owner_boma.object(), vars::miiswordsman::status::SPECIAL_LW2_HOLD) {
        VarModule::on_flag(weapon.battle_object, vars::miiswordsman_shockspell::status::SHOCK_SPELL_HOLD);
        offset_x = 60.0;
    }
    else {
        offset_x = 20.0;
    }
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f::new(pos_x + (offset_x * PostureModule::lr(weapon.module_accessor)), pos_y, 0.0));

    weapon.fastshift(L2CValue::Ptr(shock_spell_main_loop as *const () as _))
}

unsafe extern "C" fn shock_spell_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // This is the most cursed thing ever, but it somehow ignores everything else so bear with it
    SET_SPEED_EX(weapon, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if !StatusModule::is_changing(weapon.module_accessor) {
        weapon.dec_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    if weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }

    return 0.into();
}

unsafe extern "C" fn shock_spell_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *WEAPON_MIISWORDSMAN_TORNADOSHOT_STATUS_KIND_FLY, fly_pre);

    agent.status(Pre, statuses::miiswordsman_tornadoshot::SHOCK_SPELL, shock_spell_pre);
    agent.status(Main, statuses::miiswordsman_tornadoshot::SHOCK_SPELL, shock_spell_main);
    agent.status(End, statuses::miiswordsman_tornadoshot::SHOCK_SPELL, shock_spell_end);
}