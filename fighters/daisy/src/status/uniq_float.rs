use super::*;
use globals::*;

// FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START

extern "Rust" {
    #[link_name = "peach_float_start_main_common"]
    fn peach_float_start_main_common(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn uniq_float_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    peach_float_start_main_common(fighter)
}

// FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT

extern "Rust" {
    #[link_name = "peach_float_main_common"]
    fn peach_float_main_common(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn uniq_float_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    peach_float_main_common(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, uniq_float_start_main);
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, uniq_float_main);
}