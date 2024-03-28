use super::*;

unsafe extern "C" fn game_shot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let palutena = owner_module_accessor.kind() == *FIGHTER_KIND_PALUTENA;
    let damage = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::POWERED) {10.0} else {6.0};
    let paralyze = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::POWERED) {0.6} else {0.4};
    if !palutena {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 361, 41, 0, 40, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1.7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_palutena_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beamss"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), damage, 65, 40, 0, 75, 2.3, 0.0, 0.0, 0.0, None, None, None, paralyze, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            ControlModule::set_rumble(boma, Hash40::new("rbkind_beamss"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

unsafe extern "C" fn effect_shot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let palutena = owner_module_accessor.kind() == *FIGHTER_KIND_PALUTENA;
    let power = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::POWERED) {Hash40::new("sys_hit_elec")} else {Hash40::new("sys_hit_elec_s")};
    let size = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::instance::POWERED) {2.0} else {1.0};
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

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shot", game_shot);
    agent.acmd("effect_shot", effect_shot);
}