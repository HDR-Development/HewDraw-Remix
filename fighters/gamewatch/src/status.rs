use super::*;
use globals::*;

mod special_hi;
mod special_hi_open;

unsafe extern "C" fn should_use_special_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::gamewatch::instance::UP_SPECIAL_PARACHUTE) {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_CAPTURE_JUMP,
        *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY,
        *FIGHTER_STATUS_KIND_SWALLOWED,
        *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN,
        *FIGHTER_STATUS_KIND_KOOPA_DIVED,
        *FIGHTER_STATUS_KIND_CLUNG_GANON,
        *FIGHTER_STATUS_KIND_MEWTWO_THROWN,
        *FIGHTER_STATUS_KIND_BITTEN_WARIO_START,
        *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN,
        *FIGHTER_STATUS_KIND_CATCHED_REFLET,
        *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD,
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED,
        *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
        *FIGHTER_STATUS_KIND_DEMON_DIVED
    ]) {
        VarModule::off_flag(fighter.battle_object, vars::gamewatch::instance::UP_SPECIAL_PARACHUTE);
    }
    true.into()
}


extern "C" fn gamewatch_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_GAMEWATCH {
            fighter.global_table[globals::USE_SPECIAL_N_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
            fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
            fighter.global_table[globals::USE_SPECIAL_HI_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
            fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}


pub fn install() {
    smashline::Agent::new("gamewatch")
        .on_init(gamewatch_init)
        .install();
}
