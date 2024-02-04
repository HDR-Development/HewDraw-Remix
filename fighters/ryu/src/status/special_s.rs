use super::*;
use globals::*;
use smashline::*;



// FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND //


pub unsafe extern "C" fn init_special_s_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }
    smashline::original_status(Init, fighter, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND)(fighter)
}

// FIGHTER_STATUS_KIND_SPECIAL_S //


pub unsafe extern "C" fn init_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
}
pub fn install() {
    smashline::Agent::new("ryu")
        .status(
            Init,
            *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
            init_special_s_command,
        )
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, init_special_s)
        .install();
}
