use super::*;
use globals::*;
// status script import

mod special_n;
mod special_hi;

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

pub fn install(agent: &mut Agent) {
    special_n::install(agent);
    special_hi::install(agent);
}