use super::*;
use globals::*;
// status script import

mod special_lw;
mod special_s;
mod special_hi;

// Restricts Limit Charge to once-per-airtime
unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_flag(*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK)
    && VarModule::is_flag(fighter.battle_object, vars::cloud::instance::DISABLE_SPECIAL_LW) {
        return false.into();
    }

    return true.into();
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Reset Cross Slash stall flag on landing or ledgegrab
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&fighter.global_table[SITUATION_KIND].get_i32())
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.battle_object, vars::cloud::instance::SPECIAL_S_DISABLE_STALL);
    }

    // Re-enable Limit Charge on landing, ledgegrab, or when hit
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
        *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP
    ])
    || (fighter.is_status(*FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END) && fighter.is_flag(*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK))
    {
        VarModule::off_flag(fighter.battle_object, vars::cloud::instance::DISABLE_SPECIAL_LW);
    }

    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    special_lw::install(agent);
    special_s::install(agent);
    special_hi::install(agent);
}