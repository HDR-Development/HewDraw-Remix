use super::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 1.0);
    if stance != 2 {
        FT_MOTION_RATE(agent, 0.7);
    }
    else {
        FT_MOTION_RATE(agent, 9.0/(9.0 - 1.0));
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE_SPIKEBALL);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 0.7);
}

unsafe extern "C" fn game_specialsshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    let charged = WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60;
    let hit = false;
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 10.0, 3.0);
    }
    if stance == 0 {
        FT_DESIRED_RATE(agent, 5.0, 6.0);
        frame(lua_state, 5.0);
        FT_MOTION_RATE(agent, 1.0);
        if is_excute(agent) {
            if charged {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 14.0, 30, 66, 0, 60, 9.0, 2.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 10.0, 30, 66, 0, 60, 9.0, 2.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
            }
            VarModule::on_flag(boma.object(), vars::packun::status::FLAME_ACTIVE);
        }
        wait(lua_state, 5.0);
        FT_DESIRED_RATE(agent, 40.0, 30.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            VarModule::off_flag(boma.object(), vars::packun::status::FLAME_ACTIVE);
        }
    }
    else if stance == 1 {
        frame(lua_state, 2.0);
        if !WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 18, 100, 30, 0, 5.0, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(10.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            }
        }
        frame(lua_state, 10.0);
        if !WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                ArticleModule::generate_article(boma, *FIGHTER_PACKUN_GENERATE_ARTICLE_POISONBREATH, false, -1);
            }
        }
        frame(lua_state, 21.0);
        FT_MOTION_RATE(agent, 0.9);
        if is_excute(agent) {
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 5.0, 5.0);
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 31.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC);
            CancelModule::enable_cancel(boma);
        }
        frame(lua_state, 20.0);
        FT_MOTION_RATE(agent, 0.55);
    }
}

unsafe extern "C" fn game_specialsshoots(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let charged = WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60;
    let hit = false;
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 10.0, 3.0);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::packun::status::BITE_START);
    }
    FT_DESIRED_RATE(agent, 11.0, 4.0);
    frame(lua_state, 12.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 20.0);
    if is_excute(agent) {
        VarModule::off_flag(boma.object(), vars::packun::status::BITE_START);
        if VarModule::is_flag(boma.object(), vars::packun::status::BURST){
            if charged {
                ATTACK(agent, 2, 0, Hash40::new("mouth"), 17.0, 90, 100, 80, 0, 9.0, 4.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BITE);
                AttackModule::set_add_reaction_frame(boma, 0, 8.0, false);
            }
            else {
                ATTACK(agent, 2, 0, Hash40::new("mouth"), 12.0, 90, 100, 80, 0, 9.0, 4.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BITE);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
            }
        }
        else {
            if charged {
                if agent.is_situation(*SITUATION_KIND_GROUND) {
                    ATTACK(agent, 0, 0, Hash40::new("mouth"), 15.0, 180, 100, 30, 0, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                    ATTACK(agent, 1, 0, Hash40::new("mouth"), 15.0, 180, 100, 30, 0, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                    AttackModule::set_add_reaction_frame_revised(boma, 0, 14.0, false);
                    AttackModule::set_add_reaction_frame_revised(boma, 1, 3.0, false);
                }
                else {
                    ATTACK(agent, 0, 0, Hash40::new("mouth"), 15.0, 90, 100, 80, 0, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                    ATTACK(agent, 1, 0, Hash40::new("mouth"), 15.0, 90, 100, 80, 0, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                }
            }
            else {
                if agent.is_situation(*SITUATION_KIND_GROUND) {
                    ATTACK(agent, 0, 0, Hash40::new("mouth"), 10.0, 180, 100, 30, 0, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                    ATTACK(agent, 1, 0, Hash40::new("mouth"), 10.0, 180, 100, 30, 0, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                    ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
                    AttackModule::set_add_reaction_frame_revised(boma, 0, 8.0, false);
                    AttackModule::set_add_reaction_frame_revised(boma, 1, 1.0, false);
                }
                else {
                    ATTACK(agent, 0, 0, Hash40::new("mouth"), 10.0, 90, 100, 80, 0, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                    ATTACK(agent, 1, 0, Hash40::new("mouth"), 10.0, 90, 100, 80, 0, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                    ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
                }
            }
        }
    }
    wait(lua_state, 3.0);
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
        FT_DESIRED_RATE(agent, 30.0, 16.0);
    }
    else {
        FT_DESIRED_RATE(agent, 30.0, 26.0);
    }
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::off_flag(boma.object(), vars::packun::status::BURST);
    }
    wait(lua_state, 30.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specialsend(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        if stance == 0 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("mouth"), 2, -2, 0, 0, 0, 0, 0.75, false);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("mouth"), 3, -1, 0, 0, 0, 0, 0.6, true);
        }
        if stance == 1 {
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 3, -1, 0, 0, 0, 0, 1, true);
        }
        if stance == 2 {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("mouth"), 5, -3, 0, 10, 50, -20, 0.9, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("mouth"), 3, 0, 0, 0, -150, 20, 0.9, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_poison_max"), -1);
    }
}

unsafe extern "C" fn effect_specialsshoot(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if stance == 0 {
            EFFECT(agent, Hash40::new("packun_spikeball_shoot"), Hash40::new("mouth"), 2, -0.6, 0, 0, 90, -100, 1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.35, 0.02);
            let effect = if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60 { Hash40::new("sys_flame") } else { Hash40::new("packun_atk_air_b_fire") };
            let size = if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60 { 0.8 } else { 1.5 };
            EFFECT_FOLLOW(agent, effect, Hash40::new("mouth"), 7.5, 0, 0, 0, 0, 0, size, true);
        }
    }
    frame(lua_state, 6.0);
    if stance == 1 {
        if WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_breath2"), Hash40::new("mouth"), 5, -0.6, 0, 0, 90, -100, 1.2, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(lua_state);
                agent.pop_lua_stack(1);
            }
        }
        else {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_breath"), Hash40::new("mouth"), 5, -0.6, 0, 0, 90, -100, 1.1, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(lua_state);
                agent.pop_lua_stack(1);
                LAST_EFFECT_SET_RATE(agent, 1.6);
            }
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_poison_breath"), -1);
    }
}

unsafe extern "C" fn effect_specialsshoots(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("packun_bite_line"), Hash40::new("packun_bite_line"), Hash40::new("top"), -5, 11, 19, 0, -130, 35, 1, true, *EF_FLIP_YZ);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("top"), -12, 9, 20, 10, 50, 10, 0.8, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("top"), -9, 11, 18, 0, -120, 20, 1, true, *EF_FLIP_YZ);
        if VarModule::is_flag(boma.object(), vars::packun::status::BURST) {
            EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("mouth"), 0, 0, 0, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.01, 0.6);
		    LAST_EFFECT_SET_RATE(agent, 0.7);
        }
    }
}

unsafe extern "C" fn effect_specialairsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        if stance == 0 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("mouth"), 2, -2, 0, 0, 0, 0, 0.75, false);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("mouth"), 3, -1, 0, 0, 0, 0, 0.6, true);
        }
        if stance == 1 {
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 3, -1, 0, 0, 0, 0, 1, true);
        }
        if stance == 2 {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("mouth"), 5, -3, 0, 10, 50, -20, 0.9, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("mouth"), 3, 0, 0, 0, -150, 20, 0.9, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_poison_max"), -1);
    }
}

unsafe extern "C" fn effect_specialairsshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if stance == 0 {
            EFFECT(agent, Hash40::new("packun_spikeball_shoot"), Hash40::new("mouth"), 2, -0.6, 0, 0, 90, -100, 1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.35, 0.02);
            let effect = if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60 { Hash40::new("sys_flame") } else { Hash40::new("packun_atk_air_b_fire") };
            let size = if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60 { 0.8 } else { 1.5 };
            EFFECT_FOLLOW(agent, effect, Hash40::new("mouth"), 7.5, 0, 0, 0, 0, 0, size, true);
        }
    }
    frame(lua_state, 6.0);
    if stance == 1 {
        if WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_breath2"), Hash40::new("mouth"), 5, -0.6, 0, 0, 90, -100, 1.2, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(lua_state);
                agent.pop_lua_stack(1);
            }
        }
        else {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_breath"), Hash40::new("mouth"), 5, -0.6, 0, 0, 90, -100, 1.1, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(lua_state);
                agent.pop_lua_stack(1);
                LAST_EFFECT_SET_RATE(agent, 1.6);
            }
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_poison_breath"), -1);
    }
}

unsafe extern "C" fn expression_specialsshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    if stance == 0 {
        frame(lua_state, 5.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
            if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60 {
                RUMBLE_HIT(agent, Hash40::new("rbkind_explosionm"), 0);
            }
            else {
                RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
            }
        }
    }
    else {
        frame(lua_state, 2.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 5.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 8.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

unsafe extern "C" fn effect_specialairsshoots(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("packun_bite_line"), Hash40::new("packun_bite_line"), Hash40::new("top"), -5, 11, 19, 0, -130, 35, 1, true, *EF_FLIP_YZ);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("top"), -12, 9, 20, 10, 50, 10, 0.8, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("top"), -9, 11, 18, 0, -120, 20, 1, true, *EF_FLIP_YZ);
        if VarModule::is_flag(boma.object(), vars::packun::status::BURST) {
            EFFECT(agent, Hash40::new("sys_flame"), Hash40::new("mouth"), 0, 0, 0, 0, 0, 0, 1.75, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.15, 0.01, 0.6);
		    LAST_EFFECT_SET_RATE(agent, 0.7);
        }
    }
}

unsafe extern "C" fn sound_specialsshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        if stance == 0 {
            PLAY_SE(agent, Hash40::new("se_packun_special_n03"));
            if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60 {
                PLAY_SE(agent, Hash40::new("se_common_fire_m"));
            }
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if stance == 1 {
            PLAY_SE(agent, Hash40::new("se_packun_special_s03"));
        }
        else if stance == 2 {
            PLAY_SE(agent, Hash40::new("se_packun_attackhard_s03"));
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        if stance == 2 {
            PLAY_SE(agent, Hash40::new("se_packun_attackhard_s04"));
        }
    }
}

unsafe extern "C" fn sound_specialsshoots(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_attackhard_s03"));
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_packun_attackhard_s04"));
        if VarModule::is_flag(boma.object(), vars::packun::status::BURST) {
            PLAY_SE(agent, Hash40::new("se_common_bomb_s"));
        }
    }
}

unsafe extern "C" fn expression_specialsshoots(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 60 {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
        else {
            RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    FT_MOTION_RATE(agent, (10.0/15.0));
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 10.0 * stance.damage_other, 30, 70, 0, 70, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 10.0 * stance.damage_other, 30, 70, 0, 70, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 10.0 * stance.damage_other, 30, 70, 0, 70, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("armr"), 10.0 * stance.damage_other, 30, 70, 0, 70, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("arml"), 7.0 * stance.damage_other, 40, 70, 0, 70, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 7.0 * stance.damage_other, 40, 70, 0, 70, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("arml"), 7.0 * stance.damage_other, 40, 70, 0, 70, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("armr"), 7.0 * stance.damage_other, 40, 70, 0, 70, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 70.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 86.0);
    FT_MOTION_RATE(agent, 3.03);
    frame(lua_state, 90.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_HI_DIVE);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    FT_MOTION_RATE(agent, (10.0/15.0));
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        if !boma.is_status(*FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END) {
            ATTACK(agent, 0, 0, Hash40::new("arml"), 10.0 * stance.damage_other, 40, 70, 0, 50, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("armr"), 10.0 * stance.damage_other, 40, 70, 0, 50, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("arml"), 10.0 * stance.damage_other, 40, 70, 0, 50, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 3, 0, Hash40::new("armr"), 10.0 * stance.damage_other, 40, 70, 0, 50, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 38.0);
    if is_excute(agent) {
        if !boma.is_status(*FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END) {
            ATTACK(agent, 0, 0, Hash40::new("arml"), 7.0 * stance.damage_other, 50, 70, 0, 50, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("armr"), 7.0 * stance.damage_other, 50, 70, 0, 50, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("arml"), 7.0 * stance.damage_other, 50, 70, 0, 50, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 3, 0, Hash40::new("armr"), 7.0 * stance.damage_other, 50, 70, 0, 50, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 70.0);
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 86.0);
    FT_MOTION_RATE(agent, 3.03);
    frame(lua_state, 90.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_HI_DIVE);
    }
}

unsafe extern "C" fn game_speciallwbiteattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    if WorkModule::get_float(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_WORK_FLOAT_CHARGE_RATE) > 1.0 {
        if is_excute(agent) {
            if stance.label == 1 {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 20.0, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 121, 30, 3.0, false);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 26.0 * stance.damage_head, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            }
        }
    }
    else {
        if is_excute(agent) {
            if stance.label == 1 {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 0.0, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 121, 30, 2.0, false);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 0.0 * stance.damage_head, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            }
            WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_ATTACK_LERP);
        }
    }
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_BITE_ATTACK);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

unsafe extern "C" fn game_speciallwbite_attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    if WorkModule::get_float(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_WORK_FLOAT_CHARGE_RATE) > 1.0 {
        if is_excute(agent) {
            if stance.label == 1 {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 20.0, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 121, 30, 3.0, false);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 26.0 * stance.damage_head, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            }
        }
    }
    else {
        if is_excute(agent) {
            if stance.label == 1 {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 0.0, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 121, 30, 2.0, false);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 0.0 * stance.damage_head, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            }
            WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_ATTACK_LERP);
        }
    }
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_BITE_ATTACK);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

unsafe extern "C" fn effect_speciallwbite(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("packun_longrange_start"), Hash40::new("mouth"), 0, 0, 0, 180, 0, 0, 0.85, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("packun_longrange_bite_line"), Hash40::new("mouth"), 6, 0, 0, 0, 90, 0, 0.8, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if stance.label == 1 {
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 0.9, true);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 2.0);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        let size = if stance.label != 2 { 0.85 } else { 1.0 };
        EFFECT_FOLLOW(agent, Hash40::new("packun_longrange_bite"), Hash40::new("mouth"), 0, 0, 0, 180, 0, 0, size, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_longrange_bite"), -1);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        let size = if stance.label != 2 { 0.9 } else { 1.0 };
        EFFECT_FOLLOW(agent, Hash40::new("packun_longrange_bite_line2"), Hash40::new("mouth"), 5, 0, 0, 0, 90, 0, size, true);
    }
}

unsafe extern "C" fn effect_specialairlwbite(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("packun_longrange_start"), Hash40::new("mouth"), 0, 0, 0, 180, 0, 0, 0.85, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if stance.label == 1 {
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 0.9, true);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 2.0);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        let size = if stance.label != 2 { 0.85 } else { 1.0 };
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("packun_longrange_bite_line"), Hash40::new("mouth"), 6, 0, 0, 0, 90, 0, 0.9, true);
        EFFECT_FOLLOW(agent, Hash40::new("packun_longrange_bite"), Hash40::new("mouth"), 0, 0, 0, 180, 0, 0, size, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_longrange_bite"), -1);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        let size = if stance.label != 2 { 0.9 } else { 1.0 };
        EFFECT_FOLLOW(agent, Hash40::new("packun_longrange_bite_line2"), Hash40::new("mouth"), 5, 0, 0, 0, 90, 0, size, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart);
    agent.acmd("game_specialairnstart", game_specialnstart);

    agent.acmd("game_specialsshoot", game_specialsshoot);
    agent.acmd("game_specialairsshoot", game_specialsshoot);
    agent.acmd("game_specialsshoots", game_specialsshoots);
    agent.acmd("game_specialairsshoots", game_specialsshoots);
    agent.acmd("effect_specialsend", effect_specialsend);
    agent.acmd("effect_specialsshoot", effect_specialsshoot);
    agent.acmd("effect_specialsshoots", effect_specialsshoots);
    agent.acmd("effect_specialairsend", effect_specialairsend);
    agent.acmd("effect_specialairsshoot", effect_specialairsshoot);
    agent.acmd("expression_specialsshoot", expression_specialsshoot);
    agent.acmd("expression_specialairsshoot", expression_specialsshoot);
    agent.acmd("effect_specialairsshoots", effect_specialairsshoots);
    agent.acmd("sound_specialsshoot", sound_specialsshoot);
    agent.acmd("sound_specialairsshoot", sound_specialsshoot);
    agent.acmd("sound_specialsshoots", sound_specialsshoots);
    agent.acmd("sound_specialairsshoots", sound_specialsshoots);
    agent.acmd("expression_specialsshoots", expression_specialsshoots);
    agent.acmd("expression_specialairsshoots", expression_specialsshoots);

    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("game_specialairhi", game_specialairhi);

    agent.acmd("game_speciallwbiteattack", game_speciallwbiteattack);
    agent.acmd("game_speciallwbite_attack", game_speciallwbite_attack);
    agent.acmd("effect_speciallwbite", effect_speciallwbite);
    agent.acmd("effect_specialairlwbite", effect_specialairlwbite);
}