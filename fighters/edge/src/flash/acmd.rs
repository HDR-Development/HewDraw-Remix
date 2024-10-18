use super::*;

unsafe extern "C" fn effect_wait(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID)) as u32);
        let team_color = FighterUtil::get_team_color(owner_boma);
        let mut effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
        EffectModule::req_follow(boma, Hash40::new("sys_direction"), Hash40::new("top"), &Vector3f::new(0.0, 16.0, 0.0), &Vector3f::new(0.0, 90.0, 180.0), 0.67, true, 0, 0, 0, 0, 0, false, false);
        EffectModule::set_rgb_partial_last(boma, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
        let flash_handle = EffectModule::req_follow(boma, Hash40::new("edge_senkou_shield"), Hash40::new("top"), &Vector3f::new(0.0, 2.0, 0.0), &Vector3f::new(0.0, 0.0, 0.0), 0.7, false, 0, 0, 0, 0, 0, false, false);
        EffectModule::set_scale_last(boma, &Vector3f::new(0.7, 0.7, 0.4));
        VarModule::set_int64(agent.battle_object, vars::edge_flash::status::EFFECT_HANDLE, flash_handle);
    }
    wait(lua_state, 10.0);
    if is_excute(agent) {
        let flash_handle = VarModule::get_int64(agent.battle_object, vars::edge_flash::status::EFFECT_HANDLE);
        EffectModule::set_rate(boma, flash_handle as u32, (30.0 - 10.0)/90.0);
    }
    for _ in 0..4 {
        wait(lua_state, 90.0);
        if is_excute(agent) {
            let flash_handle = VarModule::get_int64(agent.battle_object, vars::edge_flash::status::EFFECT_HANDLE);
            EffectModule::set_rate(boma, flash_handle as u32, 1.0);
        }
        wait(lua_state, 10.0);
        if is_excute(agent) {
            let flash_handle = EffectModule::req_follow(boma, Hash40::new("edge_senkou_shield"), Hash40::new("top"), &Vector3f::new(0.0, 2.0, 0.0), &Vector3f::new(0.0, 0.0, 0.0), 0.7, false, 0, 0, 0, 0, 0, false, false);
            EffectModule::set_scale_last(boma, &Vector3f::new(0.7, 0.7, 0.4));
            EffectModule::set_frame(boma, flash_handle as u32, 10.0);
            EffectModule::set_rate(boma, flash_handle as u32, (30.0 - 10.0)/90.0);
            VarModule::set_int64(agent.battle_object, vars::edge_flash::status::EFFECT_HANDLE, flash_handle);
        }
    }
    for _ in 0..2 {
        wait(lua_state, 45.0);
        if is_excute(agent) {
            let flash_handle = VarModule::get_int64(agent.battle_object, vars::edge_flash::status::EFFECT_HANDLE);
            EffectModule::set_rate(boma, flash_handle as u32, 1.0);
        }
        wait(lua_state, 5.0);
        if is_excute(agent) {
            let flash_handle = EffectModule::req_follow(boma, Hash40::new("edge_senkou_shield"), Hash40::new("top"), &Vector3f::new(0.0, 2.0, 0.0), &Vector3f::new(0.0, 0.0, 0.0), 0.7, false, 0, 0, 0, 0, 0, false, false);
            EffectModule::set_scale_last(boma, &Vector3f::new(0.7, 0.7, 0.4));
            EffectModule::set_frame(boma, flash_handle as u32, 10.0);
            EffectModule::set_rate(boma, flash_handle as u32, (30.0 - 10.0)/90.0);
            VarModule::set_int64(agent.battle_object, vars::edge_flash::status::EFFECT_HANDLE, flash_handle);
        }
    }
    for _ in 0..4 {
        wait(lua_state, 20.0);
        if is_excute(agent) {
            let flash_handle = VarModule::get_int64(agent.battle_object, vars::edge_flash::status::EFFECT_HANDLE);
            EffectModule::set_rate(boma, flash_handle as u32, 1.0);
        }
        wait(lua_state, 5.0);
        if is_excute(agent) {
            let flash_handle = EffectModule::req_follow(boma, Hash40::new("edge_senkou_shield"), Hash40::new("top"), &Vector3f::new(0.0, 2.0, 0.0), &Vector3f::new(0.0, 0.0, 0.0), 0.7, false, 0, 0, 0, 0, 0, false, false);
            EffectModule::set_scale_last(boma, &Vector3f::new(0.7, 0.7, 0.4));
            EffectModule::set_frame(boma, flash_handle as u32, 10.0);
            EffectModule::set_rate(boma, flash_handle as u32, (30.0 - 10.0)/90.0);
            EffectModule::set_rgb(boma, flash_handle as u32, 0.65, 0.65, 0.65);
            VarModule::set_int64(agent.battle_object, vars::edge_flash::status::EFFECT_HANDLE, flash_handle);
        }
    }
}

unsafe extern "C" fn sound_wait(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_special_l01_01"));
    }
}

unsafe extern "C" fn game_attack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if owner.kind() == *FIGHTER_KIND_EDGE {
            ArticleModule::remove(owner, *FIGHTER_EDGE_GENERATE_ARTICLE_FLASH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 65, 60, 40, 12.0, 0.0, 1.5, 0.0, None, None, None, 0.8, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -1, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.5, 60, 70, 0, 80, 13.0, 0.0, 1.5, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_wait", acmd_stub, Priority::Low);
    agent.acmd("effect_wait", effect_wait, Priority::Low);
    agent.acmd("sound_wait", sound_wait, Priority::Low);

    agent.acmd("game_attack", game_attack, Priority::Low);

    agent.acmd("game_vanish", acmd_stub, Priority::Low);
    agent.acmd("effect_vanish", acmd_stub, Priority::Low);
    agent.acmd("sound_vanish", acmd_stub, Priority::Low);
}