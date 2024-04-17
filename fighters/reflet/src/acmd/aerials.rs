use super::*;

unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::reflet::instance::LEVIN_AERIAL_LENIENCY, 5);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 13.0, 361, 60, 0, 61, 4.3, 0.0, 7.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 13.0, 361, 60, 0, 61, 4.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 13.0, 361, 60, 0, 61, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 9.0, 55, 68, 0, 36, 3.5, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 9.0, 55, 68, 0, 36, 3.5, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 9.0, 55, 68, 0, 36, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 13.0, 41, 60, 0, 61, 4.3, 0.0, 7.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 13.0, 41, 60, 0, 61, 4.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 13.0, 41, 60, 0, 61, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 9.0, 45, 73, 0, 36, 3.5, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 9.0, 45, 73, 0, 36, 3.5, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 9.0, 45, 73, 0, 36, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
        }
        else {
            AttackModule::clear_all(boma);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
                WorkModule::off_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 5, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 5, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 5, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 6, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
    }
}

unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::reflet::instance::LEVIN_AERIAL_LENIENCY, 5);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 9.0/(12.0));
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {   
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 13.0, 67, 90, 0, 51, 4.3, 0.0, 7.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 13.0, 67, 90, 0, 51, 4.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 13.0, 67, 90, 0, 51, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 8.5, 67, 86, 0, 51, 3.5, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 8.5, 67, 86, 0, 51, 3.5, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 8.5, 67, 86, 0, 51, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 72, 80, 0, 45, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        }
        else {
            AttackModule::clear_all(boma);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
                WorkModule::off_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn expression_attackairf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if WorkModule::is_flag(boma,  *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
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
        if WorkModule::is_flag(boma,  *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
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
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 7, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else{
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 6, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
            EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
    }
}

unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::reflet::instance::LEVIN_AERIAL_LENIENCY, 5);
    }
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 15.0, 361, 98, 0, 36, 4.3, 0.0, 7.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 15.0, 361, 98, 0, 36, 4.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 15.0, 361, 98, 0, 36, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 11.5, 361, 79, 0, 36, 3.5, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 11.5, 361, 79, 0, 36, 3.5, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 11.5, 361, 79, 0, 36, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        }
        else {
            AttackModule::clear_all(boma);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
                WorkModule::off_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 6, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1.2, true);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else{
            EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 7.0, 0, 0, 0, 0, 0.5, true);
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
            EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
    }
}

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::reflet::instance::LEVIN_AERIAL_LENIENCY, 5);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 14.0, 80, 91, 0, 50, 4.3, 0.0, 7.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 14.0, 80, 91, 0, 50, 4.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 14.0, 80, 91, 0, 50, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 10.0, 80, 75, 0, 51, 3.5, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 10.0, 80, 75, 0, 51, 3.5, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 10.0, 80, 75, 0, 51, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma,  *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        }
        else {
            AttackModule::clear_all(boma);
        }
    }
    wait(lua_state, 7.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
                WorkModule::off_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 6, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(lua_state, 24.0);
    if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
    }
}

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::set_int(agent.battle_object, vars::reflet::instance::LEVIN_AERIAL_LENIENCY, 5);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            //Air-Only Hitboxes
            ATTACK(agent, 0, 0, Hash40::new("sword"), 14.0, 270, 54, 0, 22, 4.3, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 14.0, 270, 54, 0, 22, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 14.0, 270, 54, 0, 22, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            //Ground-Only Hitboxes
            ATTACK(agent, 3, 0, Hash40::new("sword"), 14.0, 70, 81, 0, 24, 4.3, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 4, 0, Hash40::new("sword"), 14.0, 70, 81, 0, 24, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 5, 0, Hash40::new("armr"), 14.0, 70, 81, 0, 24, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 10.2, 361, 61, 0, 58, 3.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 10.2, 361, 61, 0, 58, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 10.2, 361, 61, 0, 58, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 15.0, 40, 66, 0, 57, 4.3, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 15.0, 40, 66, 0, 57, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 15.0, 40, 66, 0, 57, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 3, false);
            AttackModule::clear(boma, 4, false);
            AttackModule::clear(boma, 5, false);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 9.0, 361, 58, 0, 49, 3.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("sword"), 9.0, 361, 58, 0, 49, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("armr"), 9.0, 361, 58, 0, 49, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(agent, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
                WorkModule::off_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 6, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT_FOLLOW(agent, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 8.5, 0, 0, 0, 0, 1.8, true);
        }
        else {
            AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(agent, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 7, 0, 0, 0, 0, 1.3, true);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("reflet_swoed_flare2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed"), false, false);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("reflet_thunderswoed_flare"), false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackairn", game_attackairn);
    agent.acmd("effect_attackairn", effect_attackairn);

    agent.acmd("game_attackairf", game_attackairf);
    agent.acmd("expression_attackairf", expression_attackairf);
    agent.acmd("effect_attackairf", effect_attackairf);

    agent.acmd("game_attackairb", game_attackairb);
    agent.acmd("effect_attackairb", effect_attackairb);

    agent.acmd("game_attackairhi", game_attackairhi);
    agent.acmd("effect_attackairhi", effect_attackairhi);
    
    agent.acmd("game_attackairlw", game_attackairlw);
    agent.acmd("effect_attackairlw", effect_attackairlw);
}
