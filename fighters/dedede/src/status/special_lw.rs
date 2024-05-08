use super::*;

// FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT

unsafe extern "C" fn special_lw_jump_squat_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, true);
    }
    return 0.into();
}

unsafe extern "C" fn special_lw_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, true);

    return smashline::original_status(Pre, fighter, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WAIT)(fighter);
}

unsafe extern "C" fn special_lw_walk_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, true);

    return smashline::original_status(Pre, fighter, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WALK)(fighter);
}

unsafe extern "C" fn special_lw_fall_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, true);

    return smashline::original_status(Pre, fighter, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_FALL)(fighter);

}

unsafe extern "C" fn special_lw_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    //VarModule::on_flag(fighter.battle_object, vars::dedede::instance::DISABLE_SPECIAL_LW);

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && MotionModule::frame(fighter.module_accessor) < 25.0 
    && MotionModule::frame(fighter.module_accessor) > 13.0
    && StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_GROUND{
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        VarModule::set_flag(fighter.battle_object, vars::dedede::instance::DISABLE_JET_SPEED, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("dedede_final_jet"), false, true);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
        let rate = MotionModule::rate(fighter.module_accessor);
        if rate < 3.0{
            MotionModule::set_rate(fighter.module_accessor, rate * (1.25));
        }
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER){
            let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER);
            let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_boma = sv_battle_object::module_accessor(object_id);

            MotionModule::change_motion(article_boma, Hash40::new("attack"), 0.0, 1.0, false, 0.0, false, false);

        }

    }
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            //fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(),false.into());
        }
    }

    if MotionModule::frame(fighter.module_accessor) > 30.0 && StatusModule::is_situation_changed(fighter.module_accessor){
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall_special"), 0.0, 1.0, false, 0.0, false, false);

            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into()); 
        }
    }

    return smashline::original_status(Exec, fighter, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK)(fighter);

}

unsafe extern "C" fn special_lw_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue{
    if MotionModule::is_end(fighter.module_accessor) &&
    StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_AIR{
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall_special"), 0.0, 1.0, false, 0.0, false, false);

        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());         //crashes (IDK why help)

    }


    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT, special_lw_jump_squat_exec);
    agent.status(Exec, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, special_lw_attack_exec);
    agent.status(End, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, special_lw_attack_end);
    agent.status (Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WAIT, special_lw_wait_pre);
    agent.status (Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_WALK, special_lw_walk_pre);
    agent.status (Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_FALL, special_lw_fall_pre);
}