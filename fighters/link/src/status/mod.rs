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

// unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
//     return true.into()
// }

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
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