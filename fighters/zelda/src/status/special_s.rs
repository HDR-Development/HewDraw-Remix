use super::*;

unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_LOOP
    && fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END {
        let dein_move = VarModule::get_int(fighter.battle_object, vars::zelda::instance::SPECIAL_S_CURRENT_DEIN_MOVE_OBJECT_ID);
        delete_effects(fighter, dein_move);
        sv_battle_object::end_inhaled(dein_move as u32, true);
    }
    0.into()
}

unsafe extern "C" fn special_s_loop_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END {
        let dein_move = VarModule::get_int(fighter.battle_object, vars::zelda::instance::SPECIAL_S_CURRENT_DEIN_MOVE_OBJECT_ID);
        delete_effects(fighter, dein_move);
        sv_battle_object::end_inhaled(dein_move as u32, true);
    }
    0.into()
}

pub unsafe extern "C" fn delete_effects(fighter: &mut smash::lua2cpp::L2CFighterBase, dein: i32) {
    let article_boma = sv_battle_object::module_accessor(dein as u32);
    //Fizzle effects upon deletion of moving dins, 20% smaller than din mine deletion effects
    // Fire
    let handle1 = EffectModule::req_on_joint(article_boma, Hash40::new("zelda_appeal_s_fire"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.2, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    EffectModule::set_rate_last(article_boma, 2.5);
    EffectModule::set_rgb(article_boma, handle1 as u32, 0.65, 0.3, 0.3);
    // Smoke Dark
    //let handle = EffectModule::req_on_joint(article_boma, Hash40::new("sys_steam"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    //EffectModule::set_rgb(article_boma, handle as u32, 0.0, 0.0, 0.0);
    //EffectModule::set_alpha(article_boma, handle as u32, 3.0);
    // Smoke Light
    let handle2 = EffectModule::req_on_joint(article_boma, Hash40::new("sys_steam"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.2, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    EffectModule::set_rgb(article_boma, handle2 as u32, 0.1, 0.1, 0.1);
    EffectModule::set_alpha(article_boma, handle2 as u32, 3.0);
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);
    agent.status(End, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_LOOP, special_s_loop_end);
}