use super::*;

unsafe extern "C" fn mewtwo_fall_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == statuses::mewtwo::FLOAT 
    && fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_S 
    && fighter.global_table[CURRENT_FRAME].get_i32() <= 10 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_BUOYANCY);
    }
    return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_FALL)(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_FALL, mewtwo_fall_end);
}