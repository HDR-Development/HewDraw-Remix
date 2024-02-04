use super::*;
use globals::*;



unsafe extern "C" fn special_lw_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION);
    if start_situation == *SITUATION_KIND_AIR
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        return 0.into();
    }
    smashline::original_status(Init, fighter, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END)(fighter)
}


unsafe extern "C" fn special_lw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END)(fighter);

    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION);
    if start_situation == *SITUATION_KIND_AIR
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        // Allows you to slide when landing late into Falcon Kick's animation
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KICK_START_SITUATION);
    }

    ret
}

pub fn install() {
    smashline::Agent::new("captain")
        .status(Init, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, special_lw_end_init)
        .status(Main, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, special_lw_end_main)
        .install();
}