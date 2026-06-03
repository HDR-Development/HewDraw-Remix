use super::*;
use globals::*;
// status script import

mod jump;
mod catch;
mod pass;
mod landing;

mod attack;
mod attack_air;
mod attack_jump_aerial;
mod attack_landing;
mod attack_ext;
mod attack_s3;
mod attack_s4;

mod special_n;
mod special_hi;

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        reset_vars(fighter);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
    reset_vars(fighter);
}

unsafe fn reset_vars(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_GROUND_START);
    VarModule::off_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_AIR_JUMP);
    VarModule::off_flag(fighter.battle_object, vars::tantan::instance::SPECIAL_HI_ENABLE_FREEFALL);
    VarModule::off_flag(fighter.battle_object, vars::tantan::instance::ARMS_ATTACK_CANCEL);
}

unsafe fn reset_dragon(fighter: &mut L2CFighterCommon) {
    // remove double dragon effect
    let dragonEffect = VarModule::get_int(fighter.object(), vars::tantan::instance::ARMR_DRAGONIZE_EFFECT_HANDLE) as u32;
    if EffectModule::is_exist_effect(fighter.module_accessor, dragonEffect) {
        EffectModule::kill(fighter.module_accessor, dragonEffect, false, false);
    }
    VarModule::set_int(fighter.object(), vars::tantan::instance::ARMR_DRAGONIZE_EFFECT_HANDLE, 0);
}

unsafe extern "C" fn entry_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ENTRY)(fighter);
    reset_dragon(fighter);

    return ret;
}

unsafe extern "C" fn dead_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DEAD)(fighter);
    reset_dragon(fighter);
    reset_vars(fighter);

    return ret;
}

unsafe extern "C" fn rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_REBIRTH)(fighter);
    reset_dragon(fighter);
    reset_vars(fighter);

    return ret;
}

unsafe extern "C" fn win_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_WIN)(fighter);
    reset_dragon(fighter);

    return ret;
}

unsafe extern "C" fn lose_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_LOSE)(fighter);
    reset_dragon(fighter);

    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    jump::install(agent);
    catch::install(agent);
    pass::install(agent);
    landing::install(agent);

    attack::install(agent);
    attack_air::install(agent);
    attack_jump_aerial::install(agent);
    attack_landing::install(agent);
    attack_ext::install(agent);
    attack_s3::install(agent);
    attack_s4::install(agent);

    special_n::install(agent);
    special_hi::install(agent);

    agent.status(Main, *FIGHTER_STATUS_KIND_ENTRY, entry_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_DEAD, dead_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_WIN, win_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_LOSE, lose_main);
}