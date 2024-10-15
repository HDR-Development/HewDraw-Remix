use super::*;

unsafe extern "C" fn game_shot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let damage = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::SPECIAL_N_PRIMARY_POWERED) {7.0} else {4.0};
    let paralyze = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::SPECIAL_N_PRIMARY_POWERED) {0.4} else {0.2};
    if is_excute(agent) {
        if owner_module_accessor.kind() == *FIGHTER_KIND_PALUTENA {
            ATTACK(agent, 0, 0, Hash40::new("top"), damage, 65, 40, 0, 75, 2.3, 0.0, 0.0, 0.0, None, None, None, paralyze, 0.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beamss"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

unsafe extern "C" fn effect_shot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let palutena = owner_module_accessor.kind() == *FIGHTER_KIND_PALUTENA;
    let power = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::SPECIAL_N_PRIMARY_POWERED) {Hash40::new("sys_hit_elec")} else {Hash40::new("sys_hit_elec_s")};
    let size = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::SPECIAL_N_PRIMARY_POWERED) {2.0} else {1.0};
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_bullet_shot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        if palutena {
            LAST_EFFECT_SET_COLOR(agent, 0.85, 0.40, 0.001);
        }
    }
    if palutena {
        for _ in 0..99 {
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 2.2, 1.2, 0, 0, 0, 0.23 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 0.2, -1.4, 0, 0, 0, 0.17 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 1.7, 0.1, 0, 0, 0, 0.32 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
                LAST_EFFECT_SET_RATE(agent, 1);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 1.4, 1.0, 0, 0, 0, 0.2 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_FOLLOW(agent, power, Hash40::new("top"), 0.0, 2.3, -1.4, 0, 0, 0, 0.15 * size, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
                LAST_EFFECT_SET_RATE(agent, 3.0);
                EFFECT_FOLLOW(agent, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.3, true);
                LAST_EFFECT_SET_COLOR(agent, 0.75, 0.40, 0.001);
            }
            wait(lua_state, 1.0);
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, power, false, true);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
        }
    }
}

unsafe extern "C" fn effect_check(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn game_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 160, 100, 50, 0, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.7, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 6.0);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 7.2);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 8.4);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 9.6);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 10.8);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::set_size(boma, 0, 12.0);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        //AREA_WIND_2ND_RAD(agent, 0, 1, 0.02, 1000, 1, 0, 0, 29);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 1, Hash40::new("top"), 5.5, 84, 141, 0, 60, 15.5, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_bomb"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("palutena_bomb_appear"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("palutena_bomb_finish"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_palutena_special_s02"));
    }
}

unsafe extern "C" fn effect_miss(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, true);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, true);
    }
    frame(lua_state, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shot", game_shot, Priority::Low);
    agent.acmd("effect_shot", effect_shot, Priority::Low);

    agent.acmd("game_check", acmd_stub, Priority::Low);
    agent.acmd("effect_check", effect_check, Priority::Low);
    agent.acmd("sound_check", acmd_stub, Priority::Low);

    agent.acmd("game_explode", game_explode, Priority::Low);
    agent.acmd("effect_explode", effect_explode, Priority::Low);
    agent.acmd("sound_explode", sound_explode, Priority::Low);

    agent.acmd("game_miss", acmd_stub, Priority::Low);
    agent.acmd("effect_miss", effect_miss, Priority::Low);
    agent.acmd("sound_miss", acmd_stub, Priority::Low);
}