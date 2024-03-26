use super::*;
use globals::*;

utils::import_noreturn!(common::shoto_status::{
    fgc_end_dashback
});

extern "Rust" {
    fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

// FIGHTER_DEMON_STATUS_KIND_DASH_BACK //

pub unsafe extern "C" fn main_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

pub unsafe extern "C" fn end_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::shoto_status::fgc_end_dashback(fighter);
    smashline::original_status(End, fighter, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK, main_dashback);
    agent.status(End, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK, end_dashback);
}