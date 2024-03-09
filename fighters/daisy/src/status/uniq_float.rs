use super::*;
use globals::*;

extern "Rust" {
    #[link_name = "peach_float_main_common"]
    fn peach_float_main_common(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn uniq_float(fighter: &mut L2CFighterCommon) -> L2CValue {
    peach_float_main_common(fighter)
}

pub fn install() {
    Agent::new("daisy")
        .status(Main, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, uniq_float)
        .install();
}