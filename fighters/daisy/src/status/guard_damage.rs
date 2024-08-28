use super::*;

unsafe extern "C" fn guard_damage_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::daisy::status::GUARD_OFF_YAP)
    && !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO, false, 0);
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO, statuses::daisy_kinopio::YAP, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO);
        let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(article_id);
        let offset = Vector3f {
            x: PostureModule::pos_x(fighter.module_accessor) + (10.0 * PostureModule::lr(fighter.module_accessor)),
            y: PostureModule::pos_y(fighter.module_accessor) + 7.0,
            z: -6.0
        };
        PostureModule::set_pos(article_boma, &offset);
        PostureModule::set_scale(article_boma, 1.2, true);
        LinkModule::unlink(article_boma, *WEAPON_LINK_NO_CONSTRAINT); // detaches the article from daisy
        let effect = EffectModule::req_on_joint(article_boma, Hash40::new("sys_erace_smoke"), Hash40::new("top"), &Vector3f::new(0.2, 4.5, 0.0), &Vector3f::zero(), 0.6, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        EffectModule::set_rate(fighter.module_accessor, effect as u32, 1.0);
        VarModule::on_flag(&mut *(*article_boma).object(), vars::daisy_kinopio::status::PARRY_YAP);
        VarModule::off_flag(fighter.battle_object, vars::daisy::status::GUARD_OFF_YAP);
    }

    fighter.sub_ftStatusUniqProcessGuardDamage_execStatus()
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guard_damage_exec);
}