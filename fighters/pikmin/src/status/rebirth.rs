use super::*;
use globals::*;

pub unsafe extern "C" fn rebirth_end(agent: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, -1);
    smashline::original_status(End, agent, *FIGHTER_STATUS_KIND_REBIRTH)(agent)
}

pub fn install(agent: &mut Agent) {
    smashline::Agent::new("pikmin")
        .status(End, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_end)
}
