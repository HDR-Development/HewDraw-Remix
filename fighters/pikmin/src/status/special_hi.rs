use super::*;

unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    // ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, true, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_AUTONOMY);
    ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_HI_DISABLE_AIR_FOLLOW);
    ArticleModule::set_int(fighter.module_accessor,*FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, *WEAPON_PIKMIN_PIKMIN_OWNER_CONDITION_AIR, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_OWNER_CONDITION_CURRENT);
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exec);
    agent.status(Exec, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT, special_hi_exec);
    agent.status(Exec, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END, special_hi_exec);
}