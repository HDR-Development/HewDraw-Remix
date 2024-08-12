use super::*;
 
// FIGHTER_PICKEL_STATUS_KIND_RECREATE_TABLE


unsafe extern "C" fn recreate_table_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // prevent steve from spawning the crafting table through vanilla circumstances
    if !fighter.is_prev_status(*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT)
    || !VarModule::is_flag(fighter.object(), vars::pickel::instance::CAN_RESPAWN_TABLE) {
        VarModule::on_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
        StatusModule::change_status_force(fighter.boma(), *FIGHTER_STATUS_KIND_GUARD_OFF, true); // steve will instead parry
        
        return 1.into();
    }

    // extra assurance that the table is able to fall when spawned in, and isn't held up from the cooldown indicator effect
    if ArticleModule::is_exist(fighter.boma(), *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE) {
        let table = ArticleModule::get_article(fighter.boma(), *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE);
        let table_id = smash::app::lua_bind::Article::get_battle_object_id(table) as u32;
        let table_boma = sv_battle_object::module_accessor(table_id);
        KineticModule::resume_energy_all(table_boma);
    }

    smashline::original_status(Main, fighter, *FIGHTER_PICKEL_STATUS_KIND_RECREATE_TABLE)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PICKEL_STATUS_KIND_RECREATE_TABLE, recreate_table_main);
}