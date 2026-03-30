use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdGameCommon_game_SwordSwing3Common)]
pub unsafe fn game_SwordSwing3Common(fighter: &mut L2CFighterAnimcmdGameCommon) {
    let agent = &mut fighter.clone().agent_base;
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };
    if excute {
        ItemModule::set_have_item_scale_anim(boma, 1, 1.88, 0);
        agent.clear_lua_stack();
        lua_args!(agent, 0, 0, Hash40::new("commonhave"), 8.0, 50, 88, 0, 35, 3.5, 0.0, 19.5, -2.0, LUA_VOID, LUA_VOID, LUA_VOID, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 2, 0, Hash40::new("commonhave"), 8.0, 50, 88, 0, 35, 4.0, 0.0, 13.0, -2.0, LUA_VOID, LUA_VOID, LUA_VOID, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 3, 0, Hash40::new("commonhave"), 9.0, 361, 88, 0, 35, 4.0, 0.0, 6.5, -2.0, LUA_VOID, LUA_VOID, LUA_VOID, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 1, 0, Hash40::new("commonhave"), 9.0, 361, 88, 0, 35, 4.0, 0.0, 0.0, -2.0, LUA_VOID, LUA_VOID, LUA_VOID, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("se_item_beamsword_m"));
        PLAY_SE(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("rbkind_attackm"), 0, 0);
        sv_animcmd::RUMBLE_HIT(lua_state);
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdGameCommon_game_SwordSwing4Common)]
pub unsafe fn game_SwordSwing4Common(fighter: &mut L2CFighterAnimcmdGameCommon) {
    let agent = &mut fighter.clone().agent_base;
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };
    if excute {
        ItemModule::set_have_item_scale_anim(boma, 1, 2.96, 0);
        agent.clear_lua_stack();
        lua_args!(agent, 0, 0, Hash40::new("commonhave"), 13.0, 50, 97, 0, 50, 4.5, 0.0, 29.0, -2.0, LUA_VOID, LUA_VOID, LUA_VOID, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 2, 0, Hash40::new("commonhave"), 13.0, 50, 97, 0, 50, 5.0, 0.0, 20.0, -2.0, LUA_VOID, LUA_VOID, LUA_VOID, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 3, 0, Hash40::new("commonhave"), 14.0, 361, 97, 0, 50, 5.0, 0.0, 10.0, -2.0, LUA_VOID, LUA_VOID, LUA_VOID, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 1, 0, Hash40::new("commonhave"), 14.0, 361, 97, 0, 50, 5.0, 0.0, 0.0, -2.0, LUA_VOID, LUA_VOID, LUA_VOID, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("se_item_beamsword_l"));
        PLAY_SE(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("rbkind_attackl"), 0, 0);
        sv_animcmd::RUMBLE_HIT(lua_state);
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdGameCommon_game_SwordSwingDashCommon)]
pub unsafe fn game_SwordSwingDashCommon(fighter: &mut L2CFighterAnimcmdGameCommon) {
    let agent = &mut fighter.clone().agent_base;
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };
    if excute {
        ItemModule::set_have_item_scale_anim(boma, 1, 2.07, 0);
        agent.clear_lua_stack();
        lua_args!(agent, 0, 0, Hash40::new("commonhave"), 7.0, 72, 70, 0, 80, 3.8, 0.0, 20.0, 0.0, LUA_VOID, LUA_VOID, LUA_VOID, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 2, 0, Hash40::new("commonhave"), 7.0, 72, 70, 0, 80, 4.0, 0.0, 13.0, 0.0, LUA_VOID, LUA_VOID, LUA_VOID, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 3, 0, Hash40::new("commonhave"), 8.0, 67, 70, 0, 80, 4.0, 0.0, 6.5, 0.0, LUA_VOID, LUA_VOID, LUA_VOID, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, 1, 0, Hash40::new("commonhave"), 8.0, 67, 70, 0, 80, 4.0, 0.0, 0.0, 0.0, LUA_VOID, LUA_VOID, LUA_VOID, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("se_item_beamsword_m"));
        PLAY_SE(lua_state);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("rbkind_attackm"), 0, 0);
        sv_animcmd::RUMBLE_HIT(lua_state);
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            game_SwordSwing3Common,
            game_SwordSwing4Common,
            game_SwordSwingDashCommon
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}