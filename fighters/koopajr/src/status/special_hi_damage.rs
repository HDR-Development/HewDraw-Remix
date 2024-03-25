use super::*;
use globals::*;
// status script import
 

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END

pub unsafe extern "C" fn special_hi_damage_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Main, fighter, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END)(fighter);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DAMAGE_FLY) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_DAMAGE_FALL, false);
        }
        else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
    }
    1.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END, special_hi_damage_end_main);
}