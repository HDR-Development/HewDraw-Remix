use super::*;

// FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND

// Forces Grounded Earthquake punch on the ground
unsafe extern "C" fn special_lw1_ground_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND)(fighter);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND, special_lw1_ground_main);
}