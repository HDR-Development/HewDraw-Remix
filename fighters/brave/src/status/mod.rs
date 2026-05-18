use super::*;

mod special_hi;
mod special_lw;
mod special_n;
mod landing;

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        // Re-enable upB
        VarModule::off_flag(fighter.battle_object, vars::brave::instance::SPECIAL_HI_ENABLE_FREEFALL);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }

    return true.into();
}

unsafe extern "C" fn should_use_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL) {
        false.into()
    } else {
        true.into()
    }
}

extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
    fighter.global_table[globals::USE_SPECIAL_HI_CALLBACK].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));

    VarModule::off_flag(fighter.battle_object, vars::brave::instance::SPECIAL_HI_ENABLE_FREEFALL);
    VarModule::off_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
}

unsafe extern "C" fn entry_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::PERSIST_RNG);
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::SPECIAL_MENU);
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::PSYCHE_UP_ACTIVE);
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ENTRY)(fighter)
}

unsafe extern "C" fn dead_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::PERSIST_RNG);
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::SPECIAL_MENU);
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::PSYCHE_UP_ACTIVE);
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::SPECIAL_HI_ENABLE_FREEFALL);
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DEAD)(fighter)
}

unsafe extern "C" fn rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::brave::instance::SPECIAL_HI_ENABLE_FREEFALL);
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_REBIRTH)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    agent.status(Main, *FIGHTER_STATUS_KIND_ENTRY, entry_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_DEAD, dead_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_main);

    special_hi::install(agent);
    special_lw::install(agent);
    special_n::install(agent);
    landing::install(agent);
}