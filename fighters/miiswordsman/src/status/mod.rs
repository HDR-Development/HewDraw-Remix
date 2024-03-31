use super::*;
use globals::*;
// status script import

mod special_s1;
mod special_s2;

mod special_hi;
mod special_hi2;
mod special_hi3;

mod special_lw;
mod special_lw1;
mod special_lw3;

mod final_hold;

pub unsafe extern "C" fn miisword_situation_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    special_s1::install(agent);
    special_s2::install(agent);

    special_hi::install(agent);
    special_hi2::install(agent);
    special_hi3::install(agent);

    special_lw::install(agent);
    special_lw1::install(agent);
    special_lw3::install(agent);

    final_hold::install(agent);
}