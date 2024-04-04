use super::*;

utils::import_noreturn!(common::shoto_status::{
    fgc_end_dashback,
});

extern "Rust" {
    fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

// FIGHTER_DOLLY_STATUS_KIND_DASH_BACK

pub unsafe extern "C" fn dash_back_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub unsafe extern "C" fn dash_back_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::shoto_status::fgc_end_dashback(fighter);
    smashline::original_status(End, fighter, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, dash_back_main);
    agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, dash_back_end);
}