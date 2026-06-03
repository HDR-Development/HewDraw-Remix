use super::*;

unsafe extern "C" fn game_start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let (stance, scale) = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {
        (VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE),
        VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::SPECIAL_N_PTOOIE_SCALE))
    } else { (0, 1.0) };
    VarModule::off_flag(agent.battle_object, vars::packun_spikeball::instance::ENABLE_EXPLODE);
    ModelModule::set_scale(boma, 1.0);
    frame(lua_state, 1.0);
    if stance == 2 {
        FT_MOTION_RATE(agent, 11.0/(9.0 - 1.0));
    }
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.4);
        match stance {
            1 => {
                // Putrid
                VarModule::on_flag(agent.battle_object, vars::packun_spikeball::instance::ENABLE_EXPLODE);
                ATTACK(agent, 0, 0, Hash40::new("trans"), 10.0, 55, 60, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 1, Hash40::new("trans"), 0.0, 0, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
                AttackModule::set_poison_param(boma, 0, 241, 60, 2.5, false);
            }
            2 => {
                // Prickly
                ATTACK(agent, 0, 0, Hash40::new("trans"), 18.0, 55, 60, 0, 60, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
            _ => {
                // Piranha
                ATTACK(agent, 0, 0, Hash40::new("trans"), 14.0, 55, 70, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
    }
    if stance == 2 {
        // Smoothly increase the size so it doesn't offset the Ptooie as much at the start
        for h in 1..=15 {
            if is_excute(agent) {
                ModelModule::set_scale(boma, 1.0 + ((scale - 1.0) * h as f32)/15.0);
            }
            wait(lua_state, 1.0);
        }
    }
}

unsafe extern "C" fn game_loopwait(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let (stance, scale) = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {
        (VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE),
        VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::SPECIAL_N_PTOOIE_SCALE))
    } else { (0, 1.0) };
    VarModule::off_flag(agent.battle_object, vars::packun_spikeball::instance::ENABLE_EXPLODE);
    frame(lua_state, 1.0);
    if stance == 2 {
        FT_MOTION_RATE(agent, 11.0/(9.0 - 1.0));
    }
    if is_excute(agent) {
        ModelModule::set_scale(boma, scale);
        MotionModule::set_rate(boma, 1.4);
        match stance {
            1 => {
                // Putrid
                VarModule::on_flag(agent.battle_object, vars::packun_spikeball::instance::ENABLE_EXPLODE);
                ATTACK(agent, 0, 0, Hash40::new("trans"), 10.0, 55, 60, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 1, Hash40::new("trans"), 0.0, 0, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
                AttackModule::set_poison_param(boma, 0, 241, 60, 2.5, false);
            }
            2 => {
                // Prickly
                ATTACK(agent, 0, 0, Hash40::new("trans"), 18.0, 55, 60, 0, 60, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
            _ => {
                // Piranha
                ATTACK(agent, 0, 0, Hash40::new("trans"), 14.0, 55, 70, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
    }
}

unsafe extern "C" fn effect_loop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY)
        { VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE) } else { 0 };
    if stance == 1 {    
        if is_excute(agent) {
            //EFFECT_BRANCH_SITUATION(agent, Hash40::new("null"), Hash40::new("sys_bound_smoke"), Hash40::new("top"), 0, -4, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, false);
        }
    }
}

unsafe extern "C" fn game_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY)
        { VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE) } else { 0 };
    if is_excute(agent) {
        match stance {
            1 => {
                // Putrid
                ATTACK(agent, 0, 0, Hash40::new("trans"), 12.0, 55, 60, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 1, Hash40::new("trans"), 0.0, 0, 0, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
                AttackModule::set_poison_param(boma, 0, 241, 60, 2.5, false);
            }
            2 => {
                // Prickly
                ATTACK(agent, 0, 0, Hash40::new("trans"), 20.0, 55, 60, 0, 60, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
            _ => {
                // Normal
                ATTACK(agent, 0, 0, Hash40::new("trans"), 16.0, 55, 70, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
    }
}

unsafe extern "C" fn game_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ModelModule::set_scale(boma, 0.001);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 45, 120, 0, 45, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 1, Hash40::new("top"), 0.0, 0, 0, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(boma, 0, 241, 60, 3.5, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::set_int(boma, 4, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
}

unsafe extern "C" fn effect_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1600, false);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.01, 0.7);
        LAST_EFFECT_SET_RATE(agent, 1.25);
    }
}

unsafe extern "C" fn sound_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_start", game_start, Priority::Low);
    agent.acmd("game_startair", game_start, Priority::Low);

    agent.acmd("game_loop", game_loopwait, Priority::Low);
    agent.acmd("effect_loop", effect_loop, Priority::Low);

    agent.acmd("game_shoot", game_shoot, Priority::Low);
    agent.acmd("effect_shoot", acmd_stub, Priority::Low);

    agent.acmd("game_fall", game_shoot, Priority::Low);
    agent.acmd("effect_fall", acmd_stub, Priority::Low);

    agent.acmd("game_wait", game_loopwait, Priority::Low);

    agent.acmd("game_explode", game_explode, Priority::Low);
    agent.acmd("effect_explode", effect_explode, Priority::Low);
    agent.acmd("sound_explode", sound_explode, Priority::Low);
}