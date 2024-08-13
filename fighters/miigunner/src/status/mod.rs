use super::*;
use globals::*;
// status script import

// Prevents side special from being used if a missile is present
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Grab the stored missile ID
    let missile_object_id = VarModule::get_int(fighter.battle_object, vars::miigunner::instance::MISSILE_OBJECT_ID) as u32;
    // Check if the stored object ID is *actually* a Gunner missile or not.
    if sv_battle_object::is_active(missile_object_id)
    && sv_battle_object::category(missile_object_id) == *BATTLE_OBJECT_CATEGORY_WEAPON
    && sv_battle_object::kind(missile_object_id) == *WEAPON_KIND_MIIGUNNER_SUPERMISSILE {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::miigunner::instance::BOOSTED_DAIR_AIRTIME);
    }
    true.into()
}

extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}