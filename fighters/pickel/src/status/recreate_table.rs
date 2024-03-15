use super::*;
use globals::*;
 
// FIGHTER_PICKEL_STATUS_KIND_RECREATE_TABLE

// prevent steve from spawning the crafting table through vanilla circumstances
unsafe extern "C" fn recreate_table_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_prev_status(*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT)
    || !VarModule::is_flag(fighter.object(), vars::pickel::instance::CAN_RESPAWN_TABLE) {
        VarModule::on_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
        StatusModule::change_status_force(fighter.boma(), *FIGHTER_STATUS_KIND_GUARD_OFF, true); // steve will instead parry
        
        return 1.into();
    }

    smashline::original_status(Main, fighter, *FIGHTER_PICKEL_STATUS_KIND_RECREATE_TABLE)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PICKEL_STATUS_KIND_RECREATE_TABLE, recreate_table_main);
}