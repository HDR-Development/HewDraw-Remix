use super::*;

// FIGHTER_STATUS_KIND_REBIRTH

unsafe extern "C" fn rebirth_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if [
        hash40("appeal_hi_l"),
        hash40("appeal_hi_r"),
        hash40("appeal_s_l"),
        hash40("appeal_s_r"),
        hash40("appeal_lw_l"),
        hash40("appeal_lw_r")
    ].contains(&mot) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX) {
            ArticleModule::remove(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    else {
        let fighta = fighter.global_table[0x2].get_ptr() as *mut Fighter;
        if FighterSpecializer_Snake::is_constraint_article(
            fighta,
            *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)
        ) & 1 != 0 {
            ArticleModule::shoot(
                fighter.module_accessor,
                *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX,
                ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL),
                false
            );
        }
    }
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_end);
}