use super::*;

unsafe extern "C" fn game_start(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)} else {0};
    let scale = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SCALE)} else {1.0};
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ModelModule::set_scale(boma, scale);
        MotionModule::set_rate(boma, 1.4);
        if stance == 1 {
            ATTACK(agent, 0, 0, Hash40::new("trans"), 12.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        else {
            let kbg = if stance == 0 { 0 } else { 5 };
            ATTACK(agent, 0, 0, Hash40::new("trans"), 14.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

unsafe extern "C" fn game_startair(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)} else {0};
    let scale = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SCALE)} else {1.0};
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ModelModule::set_scale(boma, scale);
        MotionModule::set_rate(boma, 1.4);
        if stance == 1 {
            ATTACK(agent, 0, 0, Hash40::new("trans"), 12.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        else {
            let kbg = if stance == 0 { 0 } else { 5 };
            ATTACK(agent, 0, 0, Hash40::new("trans"), 14.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

unsafe extern "C" fn game_loop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)} else {0};
    let scale = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SCALE)} else {1.0};
    if is_excute(agent) {
        if stance == 1 {
            ATTACK(agent, 0, 0, Hash40::new("trans"), 12.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        else {
            let kbg = if stance == 0 { 0 } else { 5 };
            ATTACK(agent, 0, 0, Hash40::new("trans"), 14.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

unsafe extern "C" fn effect_loop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)} else {0};
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
    let stance = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)} else {0};
    let scale = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SCALE)} else {1.0};
    if WorkModule::is_flag(boma, *WEAPON_PACKUN_SPIKEBALL_INSTANCE_WORK_ID_FLAG_REACTIVE) {
        if is_excute(agent) {
            if stance == 1 {
                ATTACK(agent, 0, 0, Hash40::new("trans"), 14.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
            }
            else {
                let kbg = if stance == 0 { 0 } else { 5 };
                ATTACK(agent, 0, 0, Hash40::new("trans"), 18.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
    }
    else {
        if is_excute(agent) {
            if stance == 1 {
                ATTACK(agent, 0, 0, Hash40::new("trans"), 12.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                AttackModule::set_poison_param(boma, 0, 121, 30, 3.0, false);
            }
            else {
                let kbg = if stance == 0 { 0 } else { 5 };
                ATTACK(agent, 0, 0, Hash40::new("trans"), 14.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            }
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if stance == 1 {
            ATTACK(agent, 0, 0, Hash40::new("trans"), 14.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        else {
            let kbg = if stance == 0 { 0 } else { 5 };
            ATTACK(agent, 0, 0, Hash40::new("trans"), 18.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

unsafe extern "C" fn effect_shoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)} else {0};
    if stance == 1 {
        for _ in 0..999 {
            if is_excute(agent) {
                if boma.is_status(*WEAPON_PACKUN_SPIKEBALL_STATUS_KIND_HOP){
                    EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 3.0, false);
                }
            }
            wait(lua_state, 3.0);
        }
    }
}

unsafe extern "C" fn game_fall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)} else {0};
    let scale = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SCALE)} else {1.0};
    if is_excute(agent) {
        if stance == 1 {
            ATTACK(agent, 0, 0, Hash40::new("trans"), 12.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        else {
            let kbg = if stance == 0 { 0 } else { 5 };
            ATTACK(agent, 0, 0, Hash40::new("trans"), 14.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

unsafe extern "C" fn effect_fall(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)} else {0};
    if stance == 1 {
        if is_excute(agent) {
            //EFFECT_BRANCH_SITUATION(agent, Hash40::new("null"), Hash40::new("sys_bound_smoke"), Hash40::new("top"), 0, -4, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, false);
        }
    }
}

unsafe extern "C" fn game_wait(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)} else {0};
    let scale = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_float(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SCALE)} else {1.0};
    if is_excute(agent) {
        if stance == 1 {
            ATTACK(agent, 0, 0, Hash40::new("trans"), 12.0, 55, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 121, 30, 2.5, false);
        }
        else {
            let kbg = if stance == 0 { 0 } else { 5 };
            ATTACK(agent, 0, 0, Hash40::new("trans"), 14.0 * scale, 55, 70 - kbg, 0, 50, 5.0 * scale, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
    /*frame(lua_state, 55.0);
    if is_excute(agent) {
        if stance == 1 {
            WorkModule::off_flag(boma, *WEAPON_PACKUN_SPIKEBALL_STATUS_HOP_WORK_FLAG_CLEARED_ATTACK);
            ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 55, 80, 0, 50, 999.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            AttackModule::set_poison_param(boma, 0, 361, 45, 3.0, false);
        }
    }*/
}

unsafe extern "C" fn effect_wait(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = if (owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN) || (owner_module_accessor.kind() == *FIGHTER_KIND_KIRBY) {VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE)} else {0};
    if stance == 1 {
        if is_excute(agent) {
            //EFFECT_BRANCH_SITUATION(agent, Hash40::new("null"), Hash40::new("sys_bound_smoke"), Hash40::new("top"), 0, -4, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, false);
        }
    }
}

unsafe extern "C" fn game_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ModelModule::set_scale(boma, 0.1);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 45, 120, 0, 45, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::set_poison_param(boma, 0, 151, 30, 3.5, false);
    }
}

unsafe extern "C" fn effect_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_flame"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 16.0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.01, 0.7);
        LAST_EFFECT_SET_RATE(agent, 1.25);
    }
}

unsafe extern "C" fn sound_explode(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let stance = VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_start", game_start);
    agent.acmd("game_startair", game_startair);

    agent.acmd("game_loop", game_loop);
    agent.acmd("effect_loop", effect_loop);

    agent.acmd("game_shoot", game_shoot);
    agent.acmd("effect_shoot", effect_shoot)

    agent.acmd("game_fall", game_fall);
    agent.acmd("effect_fall", effect_fall);

    agent.acmd("game_wait", game_wait);
    agent.acmd("effect_wait", effect_wait);

    agent.acmd("game_explode", game_explode);
    agent.acmd("effect_explode", effect_explode);
    agent.acmd("sound_explode", sound_explode);
}