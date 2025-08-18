use super::*;

unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("zelda_nayru_l"), true, true);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("zelda_nayru_r"), true, true);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_end);
}