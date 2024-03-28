use super::*;

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP

pub unsafe extern "C" fn init_special_hi_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.select_cliff_hangdata_from_name("special_hi");
    0.into()
}

// FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_FALL

pub unsafe extern "C" fn init_special_hi_fall(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.select_cliff_hangdata_from_name("special_hi");
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP, init_special_hi_jump);
    agent.status(Init, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_FALL, init_special_hi_fall);
}