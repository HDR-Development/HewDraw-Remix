
use super::*;

unsafe extern "C" fn game_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.4);
    }
	frame(lua_state, 11.0); // Effectively F15
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn sound_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_pickel_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

unsafe extern "C" fn game_turndash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.2);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        //WorkModule::on_flag(boma, 60192);
		WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_BLEND_TURN);
    }
    frame(lua_state, 13.0); // Effectively F15
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

unsafe extern "C" fn game_escapeair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, 29.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_escapeairslide(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_appeals(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 90.0);
    if is_excute(fighter){
        if DamageModule::damage(boma, 0) > 2.0 {
            DamageModule::add_damage(boma, -2.0, 0);
        }
    }
}

unsafe extern "C" fn game_guardon(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, false, -1);
    }
}

unsafe extern "C" fn game_guarddamage(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if !ArticleModule::is_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF) {
            ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, false, -1);
        }
    }
}

unsafe extern "C" fn sound_landingheavy(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if WorkModule::is_flag(boma, *FIGHTER_PICKEL_STATUS_LANDING_FLAG_HIGH_PLACE) {
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_pickel_landing_high_place"));
            DamageModule::add_damage(boma, 0.1, 0);
        }
    } else {
        frame(lua_state, 0.0);
        if is_excute(fighter) {
            PLAY_LANDING_SE(fighter, Hash40::new("se_pickel_landing02"));
        }
    }
}

unsafe extern "C" fn game_passive(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_MELT, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_MELT, Hash40::new("passive"), false, 0.0);
        let melt = ArticleModule::get_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_MELT);
        let melt_id = smash::app::lua_bind::Article::get_battle_object_id(melt) as u32;
        let melt_boma = sv_battle_object::module_accessor(melt_id);
        let lr = PostureModule::lr(boma);
        PostureModule::add_pos_2d(melt_boma, &Vector2f {x: (-10.0 * lr), y: 0.0});
    }
}

unsafe extern "C" fn melt_game_passive(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.boma();
    if is_excute(weapon) {
        AttackModule::clear_all(boma);
    }
} 

unsafe extern "C" fn melt_effect_passive(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.boma();
    if is_excute(weapon) {
        EFFECT_FOLLOW(weapon, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, false);
    }
} 

unsafe extern "C" fn melt_sound_passive(weapon: &mut L2CAgentBase) {
    let lua_state = weapon.lua_state_agent;
    let boma = weapon.boma();
    if is_excute(weapon) {
        PLAY_SE(weapon, Hash40::new("se_common_swim_middle_01"));
    }
} 

pub fn install() {
    smashline::Agent::new("pickel")
        .acmd("game_dash", game_dash)
        .acmd("sound_dash", sound_dash)
        .acmd("game_turndash", game_turndash)
        .acmd("game_escapeair", game_escapeair)
        .acmd("game_escapeairslide", game_escapeairslide)
        .acmd("game_appealsl", game_appeals)
        .acmd("game_appealsr", game_appeals)
        .acmd("game_guardon", game_guardon)
        .acmd("game_guarddamage", game_guarddamage)
        .acmd("sound_landingheavy", sound_landingheavy)
        .acmd("game_passive", game_passive)
        .install();
    smashline::Agent::new("pickel_melt")
        .acmd("game_passive", melt_game_passive)
        .acmd("effect_passive", melt_effect_passive)
        .acmd("sound_passive", melt_sound_passive)
        .install();
}
