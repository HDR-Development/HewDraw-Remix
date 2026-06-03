use super::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::reflet::instance::ATTACK_AIR_LEVIN_LENIENCY, 5);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        agent.on_flag( *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 13.0, 50, 76, 0, 40, 4.3, 0.0, 7.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 13.0, 50, 76, 0, 40, 4.0, 0.0, 1.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 13.0, 50, 76, 0, 40, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 8.0, 55, 87, 0, 40, 3.5, 0.0, 6.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 8.0, 55, 87, 0, 40, 3.5, 0.0, 0.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 8.0, 55, 87, 0, 40, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 14.0, 361, 76, 0, 40, 4.3, 0.0, 7.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 14.0, 361, 76, 0, 40, 4.0, 0.0, 1.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 14.0, 361, 76, 0, 40, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 9.0, 361, 86, 0, 40, 3.5, 0.0, 6.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 9.0, 361, 86, 0, 40, 3.5, 0.0, 0.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 9.0, 361, 86, 0, 40, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 26.0);
    FT_MOTION_RATE_RANGE(agent, 26.0, 36.0, 8.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 361, 80, 0, 40, 2.5, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        } else {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 32.25);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 33.0); 
    if is_excute(agent) {
        LEVIN_OFF(boma);
    }
    frame(lua_state, 36.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        agent.off_flag( *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_levin"), Hash40::new("tex_reflet_sword2"), 5, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.5, 0.0, true, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword_hdr"), Hash40::new("tex_reflet_sword5"), 5, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 1);
        EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_levin"), Hash40::new("tex_reflet_sword2"), 5, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 11.0, 0.0, true, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword_hdr"), Hash40::new("tex_reflet_sword5"), 6, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.75, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 1);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("reflet_thunderswoed"), true, true);
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::reflet::instance::ATTACK_AIR_LEVIN_LENIENCY, 5);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 9.0/(12.0));
    frame(lua_state, 6.0);
    if is_excute(agent) {
        agent.on_flag( *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {   
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("armr"), 12.5, 66, 95, 0, 42, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 12.5, 66, 95, 0, 42, 4.0, 0.0, 1.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("sword"), 12.5, 71, 95, 0, 42, 4.3, 0.0, 7.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("armr"), 7.5, 66, 110, 0, 40, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 7.5, 66, 110, 0, 40, 3.5, 0.0, 0.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("sword"), 7.5, 71, 110, 0, 40, 3.5, 0.0, 6.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 71, 80, 0, 40, 2.5, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        }
        else {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 21.0);
    if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        LEVIN_OFF(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        agent.off_flag( *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn expression_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if agent.is_flag( *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(agent) {
            AttackModule::set_attack_reference_joint_id(boma, Hash40::new("sword"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 11.0);
    app::sv_animcmd::execute(lua_state, 11.0);
    if is_excute(agent) {
        if agent.is_flag( *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
        } else {
            RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        }
    }
}

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_levin"), Hash40::new("tex_reflet_sword2"), 7, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.5, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword_hdr"), Hash40::new("tex_reflet_sword5"), 6, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 1);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("reflet_thunderswoed"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("reflet_thunderswoed_flare"), true, true);
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::reflet::instance::ATTACK_AIR_LEVIN_LENIENCY, 5);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        agent.on_flag( *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 14.5, 361, 101, 0, 25, 4.3, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 14.5, 361, 101, 0, 25, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 14.5, 361, 101, 0, 25, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 9.5, 361, 114, 0, 25, 3.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 9.5, 361, 114, 0, 25, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 9.5, 361, 114, 0,  25, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 361, 80, 0, 40, 2.5, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        }
        else {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        LEVIN_OFF(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        agent.off_flag( *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_levin"), Hash40::new("tex_reflet_sword2"), 6, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1.2, true);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        } else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword_hdr"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT_FOLLOW(agent, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 8.5, 0, 0, 0, 0, 1.35, true);
        }
        else{
            EFFECT_FOLLOW(agent, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 7.0, 0, 0, 0, 0, 1.15, true);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 1);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EffectModule::kill_kind(boma, Hash40::new("reflet_thunderswoed"), true, true);
            EffectModule::kill_kind(boma, Hash40::new("reflet_thunderswoed_flare"), true, true);
        }
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::reflet::instance::ATTACK_AIR_LEVIN_LENIENCY, 5);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        agent.on_flag( *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 14.0, 79, 90, 0, 40, 4.3, 0.0, 7.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 14.0, 79, 90, 0, 40, 4.0, 0.0, 1.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 14.0, 79, 90, 0, 40, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 9.0, 79, 102, 0, 40, 3.5, 0.0, 6.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 9.0, 79, 102, 0, 40, 3.5, 0.0, 0.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 9.0, 79, 102, 0, 40, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE_RANGE(agent, 15.0, 27.0, 10.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 79, 80, 0, 40, 2.5, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        } else {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 27.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        LEVIN_OFF(boma);
    }
    frame(lua_state, 29.0);
    if is_excute(agent) {
        agent.off_flag( *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 44.0);
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_levin"), Hash40::new("tex_reflet_sword2"), 6, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword_hdr"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 1);
    }
    frame(lua_state, 20.0);
    if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        EffectModule::kill_kind(boma, Hash40::new("reflet_thunderswoed"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("reflet_thunderswoed_flare"), true, true);
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::reflet::instance::ATTACK_AIR_LEVIN_LENIENCY, 5);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        agent.on_flag( *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            //Air-Only
            ATTACK(agent, 0, 0, Hash40::new("sword"), 15.0, 270, 46, 0, 22, 4.3, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 15.0, 270, 46, 0, 22, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 15.0, 270, 46, 0, 22, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            //Ground-Only
            ATTACK(agent, 3, 0, Hash40::new("sword"), 15.0, 70, 76, 0, 24, 4.3, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 4, 0, Hash40::new("sword"), 15.0, 70, 76, 0, 24, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 5, 0, Hash40::new("armr"), 15.0, 70, 76, 0, 24, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            //Air-Only
            ATTACK(agent, 0, 0, Hash40::new("sword"), 10.0, 270, 50, 0, 22, 3.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 10.0, 270, 50, 0, 22, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 10.0, 270, 50, 0, 22, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            //Ground-Only
            ATTACK(agent, 3, 0, Hash40::new("sword"), 10.0, 70, 85, 0, 24, 3.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 4, 0, Hash40::new("sword"), 10.0, 70, 85, 0, 24, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 5, 0, Hash40::new("armr"), 10.0, 70, 85, 0, 24, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 3, false);
        AttackModule::clear(boma, 4, false);
        AttackModule::clear(boma, 5, false);
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 15.0, 361, 75, 0, 30, 4.3, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 15.0, 361, 75, 0, 30, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 15.0, 361, 75, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 10.0, 361, 84, 0, 30, 3.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 10.0, 361, 84, 0, 30, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 10.0, 361, 84, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE_RANGE(agent, 18.0, 48.0, 25.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) { //lev
            ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 361, 80, 0, 40, 2.5, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        } else {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 24.0); // 23
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        LEVIN_OFF(boma);
    }
    frame(lua_state, 48.0); // 43
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        agent.off_flag( *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_levin"), Hash40::new("tex_reflet_sword2"), 6, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        } else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword_hdr"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_swoed_flare2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        } // makes bronze have a trail arc too
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if agent.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT_FOLLOW(agent, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 8.5, 0, 0, 0, 0, 1.8, true);
        } else {
            EFFECT_FOLLOW(agent, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 7, 0, 0, 0, 0, 1.3, true);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 1);
        EFFECT_OFF_KIND(agent, Hash40::new("reflet_swoed_flare2"), false, false);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        EffectModule::kill_kind(boma, Hash40::new("reflet_thunderswoed"), true, true);
        EffectModule::kill_kind(boma, Hash40::new("reflet_thunderswoed_flare"), true, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn, Priority::Low);
    agent.acmd("effect_attackairn", effect_attackairn, Priority::Low);

    agent.acmd("game_attackairf", game_attackairf, Priority::Low);
    agent.acmd("expression_attackairf", expression_attackairf, Priority::Low);
    agent.acmd("effect_attackairf", effect_attackairf, Priority::Low);

    agent.acmd("game_attackairb", game_attackairb, Priority::Low);
    agent.acmd("effect_attackairb", effect_attackairb, Priority::Low);

    agent.acmd("game_attackairhi", game_attackairhi, Priority::Low);
    agent.acmd("effect_attackairhi", effect_attackairhi, Priority::Low);
    
    agent.acmd("game_attackairlw", game_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw", effect_attackairlw, Priority::Low);
}
