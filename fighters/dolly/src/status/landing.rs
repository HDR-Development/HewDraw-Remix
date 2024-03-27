use super::*;

extern "Rust" {
    fn fgc_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

// FIGHTER_STATUS_KIND_LANDING //

pub unsafe extern "C" fn landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_landing_main(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_LANDING, landing_main);
}