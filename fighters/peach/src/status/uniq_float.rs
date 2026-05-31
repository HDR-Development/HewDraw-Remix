use super::*;

extern "Rust" {
    #[link_name = "float_pre_common"]
    fn float_pre_common(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "float_main_common"]
    fn float_main_common(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "float_end_common"]
    fn float_end_common(fighter: &mut L2CFighterCommon) -> L2CValue;
}

// FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT

unsafe extern "C" fn uniq_float_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_pre_common(fighter)
}

unsafe extern "C" fn uniq_float_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_main_common(fighter)
}

unsafe extern "C" fn uniq_float_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_end_common(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, uniq_float_pre);
    agent.status(Main, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, uniq_float_main);
    agent.status(End, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, uniq_float_end);
}