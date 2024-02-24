use super::*;
use globals::*;


pub fn install() {
    install_status_scripts!(
        rebirth_end
    );
}

#[status_script(agent = "pikmin", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn rebirth_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, -1);
    original!(fighter)
}