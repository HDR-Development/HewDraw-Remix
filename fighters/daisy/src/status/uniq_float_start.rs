use super::*;
use globals::*;

extern "Rust" {
    #[link_name = "peach_float_start_main_common"]
    fn peach_float_start_main_common(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn uniq_float_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    peach_float_start_main_common(fighter)
}

pub fn install() {
    Agent::new("daisy")
        .status(Main, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START, uniq_float_start)
        .install();
}