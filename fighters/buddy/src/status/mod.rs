use super::*;
use globals::*;
// status script import
 
mod run;
mod special_n;
mod special_s;
mod special_hi;

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)) 
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) { 
        VarModule::on_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_S_FAIL_ENABLE);
    }
    return true.into();
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _)); 

    VarModule::off_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_ACTIVE);
    VarModule::off_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_DISABLE);
    
    VarModule::set_int(fighter.battle_object, vars::buddy::instance::HUD_DISPLAY_TIME, 60);
    VarModule::set_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_FRAME, 0);
    VarModule::set_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_BOUNCE_TYPE, 0);
    VarModule::set_int(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_EGGS_FIRED, 0);
    
    let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    BAYONET_EGGS[entry] = 0;

    VarModule::set_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN, 0.0);
    VarModule::set_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_ANGLE, 0.0);
}

unsafe extern "C" fn win_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("buddy_special_s_count"), false, true);
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_WIN)(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    agent.status(Main, *FIGHTER_STATUS_KIND_WIN, win_main);

    run::install(agent);
    special_n::install(agent);
    special_s::install(agent);
    special_hi::install(agent);
}