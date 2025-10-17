use super::*;
use globals::*;

unsafe extern "C" fn landing_fall_special_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_BUNSHIN, ArticleOperationTarget(0));

    original_status(End, fighter, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, landing_fall_special_end);
}