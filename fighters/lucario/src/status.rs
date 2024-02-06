use super::*;
use globals::*;
// status script import
mod attack_air;
mod attack_hi4;
mod special_hi;
mod special_lw;
mod special_n;
mod special_s;

extern "C" {
    #[link_name = "\u{1}_ZN3app14LinkEventThrow13new_l2c_tableEv"]
    fn new_event_table() -> L2CValue;
    #[link_name = "\u{1}_ZN3app8lua_bind31LinkEvent__store_l2c_table_implEPKNS_9LinkEventE"]
    fn store_event_table(event: *const app::LinkEvent) -> L2CValue;
}




extern "C" fn lucario_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if smash::app::utility::get_kind(&mut *fighter.module_accessor) != *FIGHTER_KIND_LUCARIO {
            return;
        }
        fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
        fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
        MeterModule::reset(fighter.battle_object);
        MeterModule::set_meter_cap(fighter.object(), 2);
        MeterModule::set_meter_per_level(fighter.object(), 100.0);
        MeterModule::add(fighter.battle_object, dbg!(MeterModule::meter_per_level(fighter.battle_object)));
        VarModule::off_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT);
    }
}

unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::lucario::instance::DISABLE_SPECIAL_LW) {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {

    // re-enable DSpecial when landing or hit
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_LANDING])
    {
        VarModule::off_flag(fighter.battle_object, vars::lucario::instance::DISABLE_SPECIAL_LW);
    }

    0.into()
}

// FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY
// go into burnout when shield broken

unsafe extern "C" fn shield_break_fly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MeterModule::reset(fighter.battle_object);
    MeterModule::set_meter_cap(fighter.object(), 2);
    MeterModule::set_meter_per_level(fighter.object(), 100.0);
    VarModule::on_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT);
    PLAY_SE(fighter, Hash40::new("se_common_spirits_critical_l_tail"));
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY)(fighter)
}

// FIGHTER_STATUS_KIND_DEAD
// reset meter to initial state between stocks

unsafe extern "C" fn dead_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MeterModule::reset(fighter.battle_object);
    MeterModule::set_meter_cap(fighter.object(), 2);
    MeterModule::set_meter_per_level(fighter.object(), 100.0);
    MeterModule::add(fighter.battle_object, dbg!(MeterModule::meter_per_level(fighter.battle_object)));
    VarModule::off_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DEAD)(fighter)
}

// FIGHTER_STATUS_KIND_ENTRY
// reset meter to initial state between stocks

unsafe extern "C" fn entry_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MeterModule::reset(fighter.battle_object);
    MeterModule::set_meter_cap(fighter.object(), 2);
    MeterModule::set_meter_per_level(fighter.object(), 100.0);
    MeterModule::add(fighter.battle_object, dbg!(MeterModule::meter_per_level(fighter.battle_object)));
    VarModule::off_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ENTRY)(fighter)
}

// FIGHTER_STATUS_KIND_WALK //


pub unsafe extern "C" fn pre_walk(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Walk()
}

// FIGHTER_STATUS_KIND_DASH //


pub unsafe extern "C" fn pre_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Dash()
}

// FIGHTER_STATUS_KIND_RUN //


pub unsafe extern "C" fn pre_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Run()
}

pub fn install() {
    attack_air::install();
    attack_hi4::install();
    special_hi::install();
    special_lw::install();
    special_n::install();
    special_s::install();
    smashline::Agent::new("lucario")
        .on_init(lucario_init)
        .status(Main, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, shield_break_fly_main)
        .status(Main, *FIGHTER_STATUS_KIND_DEAD, dead_main)
        .status(Main, *FIGHTER_STATUS_KIND_ENTRY, entry_main)
        .status(Pre, *FIGHTER_STATUS_KIND_WALK, pre_walk)
        .status(Pre, *FIGHTER_STATUS_KIND_DASH, pre_dash)
        .status(Pre, *FIGHTER_STATUS_KIND_RUN, pre_run)
        .install();
}