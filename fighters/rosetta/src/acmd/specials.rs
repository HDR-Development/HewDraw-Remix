use super::*;

use vars::common::instance::GIMMICK_TIMER;
use vars::rosetta::instance::*;
use vars::rosetta::status::*;

unsafe extern "C" fn effect_specialnchargestart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //EFFECT_FOLLOW(agent, Hash40::new("rosetta_wand_light"), Hash40::new("havel"), 0, 7.5, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("rosetta_wand_stardust"), Hash40::new("havel"), 0, 7.5, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FOLLOW(agent, Hash40::new("rosetta_ticoshot_hold_end"), Hash40::new("havel"), 0, 7.5, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(agent, 55.0 / 65.0);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, true);
    }
    wait(lua_state, 6.0);
}

unsafe extern "C" fn sound_specialnchargestart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        let sound = SoundModule::play_status_se(boma, Hash40::new("se_rosetta_special_n01"), true, false, false);
        SoundModule::set_se_vol(boma, sound as i32, 0.45, 0);
        //PLAY_STATUS(agent, Hash40::new("se_rosetta_special_n01"));
    }
}

unsafe extern "C" fn game_specialnreturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.6);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO) {
            let tico = ArticleModule::get_article(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO);
            let tico_id = smash::app::lua_bind::Article::get_battle_object_id(tico) as u32;
            let tico_battle_object: *mut BattleObject = utils::util::get_battle_object_from_id(tico_id);
            let tico_boma: &mut BattleObjectModuleAccessor = &mut *(*tico_battle_object).module_accessor;
            if !VarModule::is_flag(agent.battle_object, vars::rosetta::instance::SPECIAL_LW_TICO_UNAVAILABLE) {//if lima is just floating around
                StatusModule::change_status_force(tico_boma, statuses::rosetta_tico::STANDBY, true); //sit still for me deer
            }
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 30, 0, 6.0, 0.0, 12.0, 14.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    FT_MOTION_RATE(agent, 0.75);
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
}

unsafe extern "C" fn game_specialhiend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let tico = ArticleModule::get_article(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO);
    let tico_id = smash::app::lua_bind::Article::get_battle_object_id(tico) as u32;
    let tico_boma = sv_battle_object::module_accessor(tico_id);
    let is_teleport = (
        !VarModule::is_flag(boma.object(), SPECIAL_LW_TICO_UNAVAILABLE) 
        && VarModule::get_int(boma.object(), GIMMICK_TIMER) == 0
        && WorkModule::is_flag(tico_boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_FREE)
    );
    if is_teleport {
        frame(lua_state, 17.0);
        if !VarModule::is_flag(boma.object(), SPECIAL_LW_INVALID_WARP) {
            if is_excute(agent) {
                if boma.is_situation(*SITUATION_KIND_GROUND) { 
                    VarModule::on_flag(boma.object(), SPECIAL_LW_WARP_GROUND_START); 
                }
                // disappear in preparation for the teleport
                HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
                VisibilityModule::set_whole(boma, false);
                JostleModule::set_status(boma, false);
                if ArticleModule::is_exist(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO) {
                    HitModule::set_whole(tico_boma, HitStatus(*HIT_STATUS_XLU), 0);
                    VisibilityModule::set_whole(tico_boma, false);
                    JostleModule::set_status(tico_boma, false);
                }
            }
            frame(lua_state, 25.0);
            if is_excute(agent) {
                if ArticleModule::is_exist(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO) {
                    // store luma's position for rosalina to use
                    VarModule::set_int(boma.object(), TICO_X, PostureModule::pos_x(tico_boma) as i32);
			        VarModule::set_int(boma.object(), TICO_Y, PostureModule::pos_y(tico_boma) as i32);
                }
                // store rosalina's position for luma to use
                VarModule::set_int(boma.object(), ROSA_X, PostureModule::pos_x(boma) as i32);
			    VarModule::set_int(boma.object(), ROSA_Y, PostureModule::pos_y(boma) as i32);
            }
            frame(lua_state, 26.0);
            if is_excute(agent) {
                // perform the actual swap
                let pos = Vector3f { 
                    x: VarModule::get_int(boma.object(), TICO_X) as f32, 
                    y: VarModule::get_int(boma.object(), TICO_Y) as f32, 
                    z: 0.0 
                };
                PostureModule::set_pos(boma, &pos);
                PostureModule::init_pos(boma, &pos, true, true);
                if ArticleModule::is_exist(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO) {
                    let tico_pos = Vector3f { 
                        x: VarModule::get_int(boma.object(), ROSA_X) as f32, 
                        y: VarModule::get_int(boma.object(), ROSA_Y) as f32, 
                        z: 0.0
                    };
                    PostureModule::set_pos(tico_boma, &tico_pos);
                    PostureModule::init_pos(tico_boma, &tico_pos, true, true);
                }
            }
            frame(lua_state, 27.0);
            if is_excute(agent) {
                // revert to normal state
                VisibilityModule::set_whole(boma, true);
                JostleModule::set_status(boma, true);	
                HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                if ArticleModule::is_exist(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO) {
                    JostleModule::set_status(tico_boma, true);	
                    VisibilityModule::set_whole(tico_boma, true);
                    HitModule::set_whole(tico_boma, HitStatus(*HIT_STATUS_NORMAL), 0);
                }
                VarModule::set_int(boma.object(), GIMMICK_TIMER, 300); // 300 Frame (5 second) cooldown
            }
            frame(lua_state, 38.0);
            if is_excute(agent) {
                if VarModule::is_flag(boma.object(), SPECIAL_LW_WARP_GROUND_START) {
                    CancelModule::enable_cancel(boma);
                }
            }
        }
    } else {
        // gravitational pull
        frame(lua_state, 1.0);
        FT_MOTION_RATE_RANGE(agent, 1.0, 5.0, 7.0);
        frame(lua_state, 5.0);//3->8
        FT_MOTION_RATE(agent, 1.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_ROSETTA_STATUS_SPECIAL_LW_FLAG_ENABLE_SEARCH);
        }
        for _ in (0..25) {
            if is_excute(agent) {
                ItemModule::pickup_item(boma, ItemSize{_address: *ITEM_SIZE_LIGHT as u8}, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
                ItemModule::use_item(boma, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, false);
            }
            wait(lua_state, 1.0);
        }
        wait(lua_state, 1.0);//30?->32 mot frame
        FT_MOTION_RATE_RANGE(agent, 32.0, 40.0, 10.0); //faf 40->45
        if is_excute(agent) {
            WorkModule::off_flag(boma, *FIGHTER_ROSETTA_STATUS_SPECIAL_LW_FLAG_ENABLE_SEARCH);
        }
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let tico = ArticleModule::get_article(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO);
    let tico_id = smash::app::lua_bind::Article::get_battle_object_id(tico) as u32;
    let tico_boma: *mut BattleObjectModuleAccessor = sv_battle_object::module_accessor(tico_id);
    let is_teleport = (
        !VarModule::is_flag(boma.object(), SPECIAL_LW_TICO_UNAVAILABLE) 
        && VarModule::get_int(boma.object(), GIMMICK_TIMER) == 0
        && WorkModule::is_flag(tico_boma, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_FREE)
    );
    if is_teleport {
        frame(lua_state, 13.0);
        if is_excute(agent) { 
            EFFECT(agent, Hash40::new("rosetta_escape"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            if ArticleModule::is_exist(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO) {
                let handle = EffectModule::req_on_joint(tico_boma, Hash40::new("rosetta_escape"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.5, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
                EffectModule::set_alpha(tico_boma, handle as u32, 1.0);
            }
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            if !VarModule::is_flag(boma.object(), SPECIAL_LW_INVALID_WARP) {
                EFFECT(agent, Hash40::new("rosetta_escape_end"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                if ArticleModule::is_exist(boma, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO) {
                    let handle = EffectModule::req_on_joint(tico_boma, Hash40::new("rosetta_escape_end"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
                    EffectModule::set_alpha(tico_boma, handle as u32, 1.0);
                }
            }
        }
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let is_teleport = (
        !VarModule::is_flag(boma.object(), SPECIAL_LW_TICO_UNAVAILABLE) 
        && VarModule::get_int(boma.object(), GIMMICK_TIMER) == 0
    ); 
    if is_teleport {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            ItemModule::set_have_item_visibility(boma, false, 0);
        }
        frame(lua_state, 17.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 27.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 35.0);
        if is_excute(agent) {
            ItemModule::set_have_item_visibility(boma, true, 0);
        }
    } else {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            ItemModule::set_have_item_visibility(boma, false, 0);
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 14.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        
        frame(lua_state, 35.0);
        if is_excute(agent) {
            ItemModule::set_have_item_visibility(boma, true, 0);
        }
        frame(lua_state, 38.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_specialnchargestart", effect_specialnchargestart, Priority::Low);
    agent.acmd("sound_specialnchargestart", sound_specialnchargestart, Priority::Low);
    agent.acmd("effect_specialairnchargestart", effect_specialnchargestart, Priority::Low);
    agent.acmd("sound_specialairnchargestart", sound_specialnchargestart, Priority::Low);

    agent.acmd("game_specialnreturn", game_specialnreturn, Priority::Low);
    agent.acmd("game_specialairnreturn", game_specialnreturn, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialhiend", game_specialhiend, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw, Priority::Low);
    agent.acmd("expression_speciallw", expression_speciallw, Priority::Low);
    agent.acmd("expression_specialairlw", expression_speciallw, Priority::Low);
}