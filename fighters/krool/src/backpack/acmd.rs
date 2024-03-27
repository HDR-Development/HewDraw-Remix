use super::*;

unsafe extern "C" fn effect_start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("krool_buckpack_start"), Hash40::new("backpack"), 0, 5, 0, 0, 0, 0, 0.75, true);
        EFFECT_FOLLOW(agent, Hash40::new("krool_propeller"), Hash40::new("propeller"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("krool_buckpack"), Hash40::new("backpack"), -12, -1.5, -6, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("krool_buckpack"), Hash40::new("backpack"), -12, -1.5, -6, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("krool_buckpack"), Hash40::new("backpack"), -12, -1.5, -6, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }

}

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let krool = utils::util::get_battle_object_from_accessor(owner_boma);
    let damage =  3.0 + (VarModule::get_int(krool, vars::krool::instance::SPECIAL_HI_FUEL) as f32 * 0.158).clamp(0.0, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("wingl1"), damage, 80, 30, 0, 90, 4.5, 2.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("krool_propeller"), Hash40::new("propeller"), 1, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("krool_buckpack"), Hash40::new("backpack"), -12, -1.5, -6, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_start", effect_start);
    agent.acmd("game_fly", game_fly);
    agent.acmd("game_flywind", game_fly);
    agent.acmd("effect_fly", effect_fly);
}