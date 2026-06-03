use super::*;

unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((agent.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(agent) {
        GroundModule::set_passable_check(boma, true);
        ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 82, 30, 0, 67, 5.25, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_palutena_bullet"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_ENERGY);
        AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_common_kick_hit_m"));
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        GroundModule::set_passable_check(boma, false);
    }
}

unsafe extern "C" fn effect_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_killereye_light"), Hash40::new("top"), 0, 0, 0.5, 180, 0, 0, 0.65, false);
        EFFECT_FOLLOW(agent, Hash40::new("sys_killereye_bullet"), Hash40::new("top"), 0, 0, 0.5, 180, 0, 0, 0.65, false);
        LAST_EFFECT_SET_COLOR(agent, 1.5, 0.4, 1.3);
    }
}

unsafe extern "C" fn sound_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_palutena_special_n02"));
        PLAY_SE(agent, Hash40::new("se_item_killereye_shot"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_move", game_move, Priority::Low);
    agent.acmd("effect_move", effect_move, Priority::Low);
    agent.acmd("sound_move", sound_move, Priority::Low);
}