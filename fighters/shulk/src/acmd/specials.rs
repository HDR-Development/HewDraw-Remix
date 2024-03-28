use super::*;

unsafe extern "C" fn effect_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if VarModule::is_flag(agent.object(), vars::shulk::status::MONADO_BEAT) {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("shulk_vision_start"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1.3, true);
            LAST_EFFECT_SET_RATE(agent, 1.2);
        }
    }
    wait(lua_state, 2.0);
    if VarModule::is_flag(agent.object(), vars::shulk::status::MONADO_BEAT) {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("shulk_vision_start"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1.5, true);
            LAST_EFFECT_SET_RATE(agent, 1.2);
            LAST_EFFECT_SET_ALPHA(agent, 0.75);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent){
        if WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
            if WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_SHIELD {
                damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 20.0);
            }
        }
    }
    frame(lua_state, 18.0);
    if is_excute(agent){
        VarModule::set_int(agent.object(), vars::shulk::instance::SPECIAL_S_STEP, 1);
        if WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
            if WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_SPEED {
                sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.5);
            }
        }
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
            if WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_JUMP {
                ATTACK(agent, 0, 0, Hash40::new("arml"), 12.0, 100, 60, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
                ATTACK(agent, 1, 0, Hash40::new("swordr"), 9.0, 100, 60, 0, 70, 3.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
            else if WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_BUSTER {
                ATTACK(agent, 0, 0, Hash40::new("arml"), 13.0, 361, 80, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
                ATTACK(agent, 1, 0, Hash40::new("swordr"), 9.7, 361, 80, 0, 70, 3.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
            else if WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_SMASH {
                ATTACK(agent, 0, 0, Hash40::new("arml"), 10.2, 361, 85, 0, 75, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
                ATTACK(agent, 1, 0, Hash40::new("swordr"), 7.7, 361, 85, 0, 75, 3.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("arml"), 12.0, 361, 80, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
                ATTACK(agent, 1, 0, Hash40::new("swordr"), 9.0, 361, 80, 0, 70, 3.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            }
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("arml"), 12.0, 361, 80, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(agent, 1, 0, Hash40::new("swordr"), 9.0, 361, 80, 0, 70, 3.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 28.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        VarModule::set_int(agent.object(), vars::shulk::instance::SPECIAL_S_STEP, 2);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        VarModule::set_int(agent.object(), vars::shulk::instance::SPECIAL_S_STEP, 5);
    }
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_circle_red"), Hash40::new("swordr"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword3_red"), Hash40::new("haver"), 0, 2.5, 0.3, 0, 0, 0, 0.4, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_shulk_swordred1"), Hash40::new("tex_shulk_swordred2"), 6, Hash40::new("haver"), 0.0, 3.0, 0.65, Hash40::new("haver"), 0.0, 15.0, 0.65, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_sword3_red"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword3_red"), Hash40::new("haver"), 0, 1.8, 0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -4, 13, -30, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(agent, 1.75);
        if WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
            if WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_SPEED {
                LAST_EFFECT_SET_COLOR(agent, 0.3, 0.3, 1.0)
            }
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -4, 13, -30, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(agent, 1.75);
        if WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
            if WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_SPEED {
                LAST_EFFECT_SET_COLOR(agent, 0.3, 0.3, 1.0)
            }
        }
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 1);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_sword3_red"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_circle_red"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword3_red_end"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.87, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.549, 0.549)
    }
}

unsafe extern "C" fn effect_specialairs(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_circle_red"), Hash40::new("swordr"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword3_red"), Hash40::new("haver"), 0, 2.5, 0.3, 0, 0, 0, 0.4, true);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_shulk_swordred1"), Hash40::new("tex_shulk_swordred2"), 6, Hash40::new("haver"), 0.0, 3.0, 0.65, Hash40::new("haver"), 0.0, 15.0, 0.65, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_sword3_red"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword3_red"), Hash40::new("haver"), 0, 1.8, 0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -4, 13, -30, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(agent, 1.75);
        if WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
            if WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_SPEED {
                LAST_EFFECT_SET_COLOR(agent, 0.3, 0.3, 1.0)
            }
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -4, 13, -30, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(agent, 1.75);
        if WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
            if WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_SPEED {
                LAST_EFFECT_SET_COLOR(agent, 0.3, 0.3, 1.0)
            }
        }
    }
    frame(lua_state, 37.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 1);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_sword3_red"), false, false);
        EFFECT_OFF_KIND(agent, Hash40::new("shulk_monad_circle_red"), false, false);
        EFFECT_FOLLOW(agent, Hash40::new("shulk_monad_sword3_red_end"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.87, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.549, 0.549)
    }
}

unsafe extern "C" fn sound_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_shulk_rnd_special_s"));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shulk_special_s01"));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shulk_monado_open"));
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_shulk_special_s03"));
    }
}

unsafe extern "C" fn expression_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("swordr"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_monad_hand") as i64);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_monad_behind") as i64);
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 9.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
        WorkModule::on_flag(boma,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        WorkModule::on_flag(boma,*FIGHTER_SHULK_STATUS_SPECIAL_HI_FLAG_IS_ADD_SHIFT_RESERVE_INPUT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 6.0, 78, 100, 140, 0, 3.8, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 78, 100, 140, 0, 3.8, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 6.0, 78, 100, 140, 0, 3.8, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 6.0, 78, 100, 140, 0, 3.8, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("haver"), 6.0, 91, 100, 135, 0, 3.8, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 91, 100, 135, 0, 3.8, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("haver"), 6.0, 91, 100, 135, 0, 5.0, 0.0, 14.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 6.0, 91, 100, 135, 0, 5.0, 0.0, 10.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 80, 100, 78, 0, 3.2, 0.0, 20.0, 7.0, Some(0.0), Some(4.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 86, 100, 80, 0, 4.5, 0.0, 20.0, 16.0, Some(0.0), Some(4.0), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 52, 100, 43, 0, 3.2, 0.0, 18.0, 7.0, Some(0.0), Some(10.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 70, 100, 43, 0, 3.2, 0.0, 18.0, 16.0, Some(0.0), Some(10.0), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07),*GROUND_CLIFF_CHECK_KIND_ALWAYS);
        WorkModule::on_flag(boma,*FIGHTER_SHULK_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_ADD_SHIFT_INPUT);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma,*FIGHTER_SHULK_STATUS_SPECIAL_HI_FLAG_IS_FALL);
        WorkModule::on_flag(boma,*FIGHTER_SHULK_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07),*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma,*FIGHTER_SHULK_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_ADD_SHIFT_INPUT);
    }
}

unsafe extern "C" fn game_speciallwattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 50, 80, 0, 70, 12.0, 0.0, 10.5, 28.0, Some(0.0), Some(10.5), Some(20.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 50, 80, 0, 70, 7.0, 0.0, 10.5, 33.0, Some(0.0), Some(10.5), Some(9.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 50, 80, 0, 70, 12.0, 0.0, 10.5, 25.0, Some(0.0), Some(10.5), Some(17.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 50, 80, 0, 70, 7.0, 0.0, 10.5, 30.0, Some(0.0), Some(10.5), Some(6.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    FT_MOTION_RATE(agent, 0.8);
}

unsafe extern "C" fn game_speciallwf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 50, 90, 0, 70, 11.0, 0.0, 9.0, -24.5, Some(0.0), Some(9.0), Some(-15.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 50, 90, 0, 70, 7.0, 0.0, 9.0, -28.5, Some(0.0), Some(9.0), Some(-3.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_specialnstart", effect_specialnstart);
    agent.acmd("effect_specialairnstart", effect_specialnstart);

    agent.acmd("game_specials", game_specials);
    agent.acmd("game_specialairs", game_specials);
    agent.acmd("effect_specials", effect_specials);
    agent.acmd("effect_specialairs", effect_specialairs);
    agent.acmd("sound_specials", sound_specials);
    agent.acmd("sound_specialairs", sound_specials);
    agent.acmd("expression_specials", expression_specials);
    agent.acmd("expression_specialairs", expression_specials);

    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("game_specialairhi", game_specialhi);
    
    agent.acmd("game_speciallwattack", game_speciallwattack);
    agent.acmd("game_specialairlwattack", game_speciallwattack);
    agent.acmd("game_speciallwf", game_speciallwf);
}
