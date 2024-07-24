use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, false, 0);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 2.5, 24, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_daisy_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_common_smashswing_02"));
    }
}

unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
}

unsafe extern "C" fn game_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, false, 0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_DIVE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 315, 100, 50, 0, 4.5, 0.0, 6.5, 6.0, Some(0.0), Some(6.5), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        VarModule::off_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_DIVE);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 51.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_AUTOCANCEL);
        ArticleModule::remove_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn effect_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 12, 19, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11, -4, -120, 0, 0, 1.0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.6, 0.6, 0.6);
    }
    for i in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11, 0, -120, 0, 0, 1.0, false);
            LAST_EFFECT_SET_COLOR(agent, 0.3, 0.3, 0.3);
            EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("arml"), 4, 0, 0, 0, 0, 0, (0.30 - (i as f32 * 0.015)), false);
            LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        }
        wait(lua_state, 5.0);
    }
    frame(lua_state, 51.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        let rng = app::sv_math::rand(hash40("fighter"), 5);
        if rng > 2 {
            PLAY_SE(agent, Hash40::new("vc_daisy_attack07"));
        } else {
            PLAY_SEQUENCE(agent, Hash40::new("seq_daisy_rnd_attack"));
        }
        PLAY_SE(agent, Hash40::new("se_common_smashswing_02"));
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_spirits_critical_l_tail"));
    }
}

unsafe extern "C" fn expression_specialairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
}

unsafe extern "C" fn game_specialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        VarModule::on_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_CRYSTAL_ACTIVE);
        ArticleModule::generate_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, false, 0);
        if !VarModule::is_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_AIR_START) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 17.0, 75, 83, 0, 64, 4.0, 0.0, 4.0, -6.5, Some(0.0), Some(4.0), Some(12.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 60, 83, 0, 64, 4.0, 0.0, 4.0, -6.5, Some(0.0), Some(4.0), Some(12.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        }
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 75, 78, 0, 64, 3.5, 0.0, 10.5, -6.5, Some(0.0), Some(10.5), Some(-6.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 75, 78, 0, 64, 3.5, 0.0, 10.5, 12.5, Some(0.0), Some(10.5), Some(12.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(0));
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_CRYSTAL_ACTIVE);
    }
}

unsafe extern "C" fn effect_specialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let is_aerial = VarModule::is_flag(agent.battle_object, vars::daisy::status::SPECIAL_N_AIR_START);
    let offset = if is_aerial { 3.0 } else { 0.0 };
    let crystals: [[f32;7];6] = [
            // pos_x  pos_y   pos_z  scale_x scale_y scale_z  id
            [   0.0,   7.0,   13.0,   0.25,    0.8,   0.25,   0.0  ],
            [   0.0,   7.0,   -7.0,   0.25,    0.8,   0.25,   1.0  ],
            [  10.0,   3.7,    7.0,   0.15,    0.4,   0.15,   2.0  ],
            [  10.0,   4.7,   -2.0,   0.15,    0.5,   0.15,   3.0  ],
            [ -10.0,   4.7,    8.0,   0.15,    0.5,   0.15,   4.0  ],
            [ -10.0,   3.7,   -1.0,   0.15,    0.4,   0.15,   5.0  ],
        ];
    let mut crystal_handles: [u32;6] = [0, 0, 0, 0, 0, 0];
    frame(lua_state, 1.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        EFFECT_FLIP(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 3.0 + offset, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(agent, 1.2);
        EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("top"), -7.0 + offset, 1, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 13.0 + offset, 1, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        for entry in crystals {
            EFFECT_FOLLOW(agent, Hash40::new("sys_ice"), Hash40::new("top"), entry[0] + offset, entry[1], entry[2], 0.0, 200.0, 0.0, 1, true);
            EffectModule::set_scale_last(boma, &Vector3f::new(entry[3], entry[4], entry[5]));
            LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
            if is_aerial {
                let handle = EffectModule::get_last_handle(boma) as u32;
                crystal_handles[entry[6] as usize] = handle;
            }
        }
    }
    if is_aerial {
        // distance that daisy has traveled from the original origin point, per frame. may have to be re-recorded if physics are ever updated
        let x_offsets: [f32;12] = [0.0, 0.92, 1.76, 2.52, 3.2, 3.8, 4.32, 4.76, 5.12, 5.4, 5.6, 5.72];
        for x_delta in x_offsets {
            if is_excute(agent) {
                //EFFECT_OFF_KIND(agent, Hash40::new("sys_ice"), false, false);
                for entry in crystals {
                    EffectModule::set_pos(boma, crystal_handles[entry[6] as usize], &Vector3f{
                        x: entry[0],
                        y: entry[1],
                        z: PostureModule::pos_z(boma) + entry[2] + offset - x_delta
                    })
                } 
            }
            wait(lua_state, 1.0);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_ice"), false, false);
        EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("top"), -7.0 - offset, 1, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 13.0 - offset, 1, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        EFFECT(agent, Hash40::new("sys_freezer"), Hash40::new("top"), 3.0 - offset, 1, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
    }
}

unsafe extern "C" fn sound_specialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_frieze_l"));
    }
    frame(lua_state, 41.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_freeze"));
    }
}

unsafe extern "C" fn expression_specialnattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    FT_MOTION_RATE_RANGE(agent, 5.0, 15.0, 6.0);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialsjump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE_RANGE(agent, 1.0, 25.0, 15.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
        SEARCH(agent, 0, 0, Hash40::new("hip"), 3.5, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_PEACH_SPECIAL_S_BRAKE);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        WorkModule::enable_transition_term(boma, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_ID_TIME_OUT);
    }
}

unsafe extern "C" fn game_specialshitend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 78, 75, 0, 60, 7.7, 0.0, 5.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HIP);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE_RANGE(agent, 4.0, 23.0, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_start"), false, 1.0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PEACH_STATUS_SPECIAL_HI_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.5, 65, 75, 0, 70, 6.0, 0.0, 6.0, 5.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("haver"), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), app::AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ATTACK(agent, 0, 0, Hash40::new("havel"), 7.0, 78, 80, 0, 85, 4.0, 0.0, -0.5, 0.0, Some(0.0), Some(-6.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn effect_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 6, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    for i in 0..5 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_freezer"), Hash40::new("arml"), 4, 0, 0, 0, 0, 0, (0.30 - (i as f32 * 0.03)), false);
            LAST_EFFECT_SET_COLOR(agent, 0.3, 1.0, 0.8);
        }
        wait(lua_state, 2.0);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_erace_smoke"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_daisy_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_common_smashswing_02"));
        PLAY_SE(agent, Hash40::new("se_roulette_circle_spark"));
    }
}

unsafe extern "C" fn expression_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 2);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn expression_specialairhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialhiopen(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_open"), false, 1.0);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let item_kind = agent.get_int(*FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND); 
        if item_kind == *ITEM_KIND_NONE {
            ArticleModule::generate_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_DAIKON, false, -1);
        } else if item_kind == *ITEM_KIND_BOMBHEI {
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_FIREFLOWER), 0, 0, false, false);
        } else if item_kind == *ITEM_KIND_DOSEISAN {
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_SOCCERBALL), 0, 0, false, false);
            ItemModule::drop_item(boma, 90.0, 0.0, 0);
        } else if item_kind == *ITEM_KIND_BEAMSWORD {
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_MAGICBALL), 0, 0, false, false);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        if !agent.is_situation(*SITUATION_KIND_GROUND) {
            agent.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("daisy_hikkonuki"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1.0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("daisy_hikkonuki"), -1);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_daisy_special_l02"));
    }
    wait(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_daisy_special_l01"));
        if agent.get_int(*FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND) != *ITEM_KIND_NONE 
        && ItemModule::is_have_item(boma, 0) {
            PLAY_SE(agent, Hash40::new("vc_daisy_appeal_s01"));
        } 
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn, Priority::Low);
    agent.acmd("effect_specialn", effect_specialn, Priority::Low);
    agent.acmd("sound_specialn", sound_specialn, Priority::Low);
    agent.acmd("expression_specialn", expression_specialn, Priority::Low);

    agent.acmd("game_specialairn", game_specialairn, Priority::Low);
    agent.acmd("effect_specialairn", effect_specialairn, Priority::Low);
    agent.acmd("sound_specialairn", sound_specialairn, Priority::Low);
    agent.acmd("expression_specialairn", expression_specialairn, Priority::Low);

    agent.acmd("game_specialnattack", game_specialnattack, Priority::Low);
    agent.acmd("effect_specialnattack", effect_specialnattack, Priority::Low);
    agent.acmd("sound_specialnattack", sound_specialnattack, Priority::Low);
    agent.acmd("expression_specialnattack", expression_specialnattack, Priority::Low);

    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialairsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialsjump", game_specialsjump, Priority::Low);
    agent.acmd("game_specialshitend", game_specialshitend, Priority::Low);

    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialairhistart", game_specialhistart, Priority::Low);
    agent.acmd("effect_specialhistart", effect_specialhistart, Priority::Low);
    agent.acmd("effect_specialairhistart", effect_specialhistart, Priority::Low);
    agent.acmd("sound_specialhistart", sound_specialhistart, Priority::Low);
    agent.acmd("sound_specialairhistart", sound_specialhistart, Priority::Low);
    agent.acmd("expression_specialhistart", expression_specialhistart, Priority::Low);
    agent.acmd("expression_specialairhistart", expression_specialairhistart, Priority::Low);
    agent.acmd("game_specialhiopen", game_specialhiopen, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("sound_speciallw", sound_speciallw, Priority::Low);
}