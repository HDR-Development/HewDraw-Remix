use super::*;
use globals::*;

pub unsafe extern "C" fn special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END)(fighter);
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main,*FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END,special_s_end_main,)
}
