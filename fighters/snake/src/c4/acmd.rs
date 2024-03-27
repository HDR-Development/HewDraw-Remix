use super::*;

unsafe extern "C" fn game_establishtarget(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if VarModule::is_flag(agent.object(), vars::snake::instance::SELF_STICK) {
            SEARCH(agent, 0, 0, Hash40::new("rot"), 0.1, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        }
        else {
            SEARCH(agent, 0, 0, Hash40::new("rot"), 5.0, 0.0, -3.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        }
    }
}

unsafe extern "C" fn game_sticktarget(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn effect_sticktarget(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("snake_final_lockon"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_final_lockon2"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_final_lockon_ready"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_final_lockon_ready2"), Hash40::new("top"), 0, 0.0, 0.0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.65, true);
    }
    wait(lua_state, 60.0);
    if is_excute(agent) {
        //EFFECT_OFF_KIND(agent, Hash40::new("snake_final_lockon_ready"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("snake_final_lockon_ready2"), false, false);
    }
    wait(lua_state, 90.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.5, true);
    }
    for _ in 0..4 {
        wait(lua_state, 150.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
            EFFECT_FOLLOW(agent, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.5, true);
        }
    }
    wait(lua_state, 150.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke2"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke3"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke4"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
    }
    for _ in 0..10 {
        wait(lua_state, 10.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke2"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke3"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke4"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        }
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 10.0, true);
    }
}

unsafe extern "C" fn sound_sticktarget(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_C3){
            PLAY_SE(agent, Hash40::new("se_snake_special_l08"));        
        }
        else{
            PLAY_SE(agent, Hash40::new("se_snake_special_l03"));
        }
        PLAY_SE(agent, Hash40::new("se_snake_final02"));
    }
}

unsafe extern "C" fn effect_stickother(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.65, true);
    }
    for _ in 0..5 {
        wait(lua_state, 150.0);
        if is_excute(agent) {
            //EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(agent) {
            //EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(agent) {
            //EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
        }
        wait(lua_state, 150.0);
        if is_excute(agent) {
            //EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
            //EFFECT_FOLLOW(agent, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.5, true);
        }
    }
    wait(lua_state, 150.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 0.3, true);
    }
    wait(lua_state, 150.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_light"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_c4_flash"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.0, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke2"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke3"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke4"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
    }
    for _ in 0..10 {
        wait(lua_state, 10.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke2"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke3"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke4"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 1.5, true);
        }
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("snake_missile_smoke"), Hash40::new("top"), 0, 1.025, 0.7, 0, 0, 0, 10.0, true);
    }
}

unsafe extern "C" fn game_explosion(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        // Hitbox for opponents
        ATTACK(agent, 0, 0, Hash40::new("rot"), 16.0, 86, 78, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        // Snake-only hitbox
        ATTACK(agent, 1, 0, Hash40::new("rot"), 16.0, 86, 78, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        VisibilityModule::set_whole(boma, false);
        QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 17.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_GROUND){
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_GROUND){
            AttackModule::clear_all(boma);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_establishtarget", game_establishtarget);
    agent.acmd("game_sticktarget", game_sticktarget);
    agent.acmd("effect_sticktarget", effect_sticktarget);
    agent.acmd("sound_sticktarget", sound_sticktarget);
    agent.acmd("effect_stickother", effect_stickother);
    agent.acmd("game_explosion", game_explosion);
}