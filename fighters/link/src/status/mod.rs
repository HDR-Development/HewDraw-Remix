use super::*;
use globals::*;
// status script import

mod special_n;
mod special_s;
mod special_hi;
mod special_lw;

pub unsafe extern "C" fn link_situation_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 1.into()
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            return 1.into()
        }
        if fighter.global_table[PREV_SITUATION_KIND] != SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            return 1.into()
        }
    }
    return 0.into()
}

// attempting to boomerang when one was already out would use a fail motion anyways
// so lets just prevent using boomerang in that instance, removing the annoying fail motion
unsafe extern  "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOOMERANG) {
        return false.into()
    }
    return true.into()
}

// fixes some issues where DSpecial wouldn't work unexpectedly
unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    return true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
    fighter.global_table[0x27].assign(&false.into()); // ground special transition handler
    fighter.global_table[0x26].assign(&false.into()); // air special transition handler
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_n::install(agent);
    special_s::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
}