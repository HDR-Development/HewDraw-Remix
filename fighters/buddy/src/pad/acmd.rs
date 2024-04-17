use super::*;

unsafe extern "C" fn game_fall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;    
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let pad_length = 6.0;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 46, 70, 0, 40, 2.0, 0.0, 2.2, pad_length, Some(0.0), Some(2.2), Some(-pad_length), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    //Vanilla code from here on out
    frame(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *WEAPON_BUDDY_PAD_STATUS_WORK_FLAG_FALL_LEANING_TO_THE_LEFT);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, /*Flag*/ *WEAPON_BUDDY_PAD_STATUS_WORK_FLAG_FALL_LEANING_TO_THE_LEFT);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *WEAPON_BUDDY_PAD_STATUS_WORK_FLAG_FALL_LEANING_TO_THE_RIGHT);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, /*Flag*/ *WEAPON_BUDDY_PAD_STATUS_WORK_FLAG_FALL_LEANING_TO_THE_RIGHT);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *WEAPON_BUDDY_PAD_STATUS_WORK_FLAG_FALL_LEANING_TO_THE_LEFT);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, /*Flag*/ *WEAPON_BUDDY_PAD_STATUS_WORK_FLAG_FALL_LEANING_TO_THE_LEFT);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fall", game_fall);
}
