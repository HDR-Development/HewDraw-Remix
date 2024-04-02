use super::*;
use globals::*;
// status script import

mod special_s;
mod special_hi;

/// Prevents side b from being used again in air
unsafe extern "C" fn should_use_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR && WorkModule::is_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS) {
        false.into()
    } else {
        true.into()
    }
}

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let damage_statuses = &[*FIGHTER_STATUS_KIND_DAMAGE,
    *FIGHTER_STATUS_KIND_DAMAGE_AIR,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
    *FIGHTER_STATUS_KIND_DAMAGE_FALL];

    if (fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF))
    || fighter.is_status_one_of(damage_statuses) { 
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_HI_CALLBACK].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
}


pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_s::install(agent);
    special_hi::install(agent);
}