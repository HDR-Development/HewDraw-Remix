use super::*;

unsafe extern "C" fn game_luigispecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, 0);
    }
}

unsafe extern "C" fn effect_luigispecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("luigi_fb_shoot"), Hash40::new("havel"), 0, 0, 0, -30, 0, 0, 1, true);
        FLASH(agent, 0, 1, 0, 0.353);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("luigi_fb_shoot"), false, false);
    }
}

unsafe extern "C" fn sound_luigispecialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_luigi_special_n01"));
    }
}

unsafe extern "C" fn game_luigispecialnthunder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 68, 55, 0, 65, 5.0, 0.0, 6.5, 9.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 68, 55, 0, 65, 3.0, 0.0, 6.5, 3.0, None, None, None, 0.6, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_luigispecialnthunder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 5, 15, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        let mut rand = &Vector3f::new(
            app::sv_math::rand(hash40("fighter"), 50) as f32,
            app::sv_math::rand(hash40("stage"), 50) as f32,
            app::sv_math::rand(hash40("luigi"), 50) as f32
        );
        let mut flip = &Vector3f::new(
            if app::sv_math::rand(hash40("fighter"), 1) == 0 { -1 } else { 1 } as f32,
            if app::sv_math::rand(hash40("stage"), 1) == 0 { -1 } else { 1 } as f32,
            if app::sv_math::rand(hash40("luigi"), 1) == 0 { -1 } else { 1 } as f32
        );
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 6.5, 9.0, 0.0 + (rand.x * flip.x), 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 6.5, 9.0, 120.0 + (rand.y * flip.y), 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 6.5, 9.0, 240.0 + (rand.z * flip.z), 0, 0, 0.5, true);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0.25, 1.0, 0.7);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 6.5, 9.0, 0, 90, 90, 0.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0.0, 6.5, 9.0, 0, 90, 90, 0.5, true);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
            LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
            LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_luigispecialnthunder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_kirby_attack02"));
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_l"));
    }
}

unsafe extern "C" fn expression_luigispecialnthunder(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_55_smash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_luigispecialn", game_luigispecialn);
    agent.acmd("game_luigispecialairn", game_luigispecialn);
    agent.acmd("effect_luigispecialn", effect_luigispecialn);
    agent.acmd("effect_luigispecialairn", effect_luigispecialn);
    agent.acmd("sound_luigispecialn", sound_luigispecialn);
    agent.acmd("sound_luigispecialairn", sound_luigispecialn);
    agent.acmd("game_luigispecialnthunder", game_luigispecialnthunder);
    agent.acmd("game_luigispecialairnthunder", game_luigispecialnthunder);
    agent.acmd(
        "effect_luigispecialnthunder",
        effect_luigispecialnthunder,
    );
    agent.acmd(
        "effect_luigispecialairnthunder",
        effect_luigispecialnthunder,
    );
    agent.acmd("sound_luigispecialnthunder", sound_luigispecialnthunder);
    agent.acmd(
        "sound_luigispecialairnthunder",
        sound_luigispecialnthunder,
    );
    agent.acmd(
        "expression_luigispecialnthunder",
        expression_luigispecialnthunder,
    );
    agent.acmd(
        "expression_luigispecialairnthunder",
        expression_luigispecialnthunder,
    );
}