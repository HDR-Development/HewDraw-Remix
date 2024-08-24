use super::*;
use globals::*;

mod special_s;

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
    if VarModule::is_flag(fighter.battle_object, vars::gamewatch::instance::UP_SPECIAL_PARACHUTE) {
        if (fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
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
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI,
            *FIGHTER_STATUS_KIND_CATCHED_REFLET,
            *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
            *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY,
            *FIGHTER_STATUS_KIND_SWALLOWED,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_CAPTURE_JUMP,
            *FIGHTER_STATUS_KIND_CAPTURE_YOSHI,
            *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN,
            *FIGHTER_STATUS_KIND_CLUNG_DAMAGE_DIDDY,
            *FIGHTER_STATUS_KIND_CLUNG_DIDDY,
            *FIGHTER_STATUS_KIND_CLUNG_GANON,
            *FIGHTER_STATUS_KIND_CLUNG_THROWN_BLANK_DIDDY,
            *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY,
            *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED,
            *FIGHTER_STATUS_KIND_SWING_GAOGAEN_ATTACH_ROPE,
            *FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE,
            *FIGHTER_STATUS_KIND_SWING_GAOGAEN_LARIAT,
            *FIGHTER_STATUS_KIND_SWING_GAOGAEN_RETURN,
            *FIGHTER_STATUS_KIND_SWING_GAOGAEN_SHOULDER,
            *FIGHTER_STATUS_KIND_SWING_GAOGAEN_THROWN,
            *FIGHTER_STATUS_KIND_KOOPA_DIVED,
            *FIGHTER_STATUS_KIND_DEMON_DIVED,
            *FIGHTER_STATUS_KIND_MEWTWO_THROWN,
            *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN,
            *FIGHTER_STATUS_KIND_BITTEN_WARIO_START,
            *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
        ])) {
            VarModule::off_flag(fighter.battle_object, vars::gamewatch::instance::UP_SPECIAL_PARACHUTE);
        }
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_N_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
    fighter.global_table[globals::USE_SPECIAL_HI_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
    fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_s::install(agent);

    special_hi::install(agent);
    special_hi_open::install(agent);
}