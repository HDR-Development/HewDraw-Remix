use super::*;

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::dolly::status::UNABLE_CANCEL_S3_DASH);
        VarModule::off_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            VarModule::on_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
        }
        FT_MOTION_RATE(agent, 7.0/(9.0-1.0));
        MeterModule::watch_damage(agent.battle_object, true);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        //HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 9.0, 39, 50, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("legr"), 9.0, 39, 50, 0, 50, 3.5, 2.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneer"), 11.0, 39, 35, 0, 45, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("kneer"), 11.0, 39, 35, 0, 45, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 2, 6.0, false);
        AttackModule::set_add_reaction_frame(boma, 3, 6.0, false);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        VarModule::off_flag(boma.object(), vars::dolly::status::UNABLE_CANCEL_S3_DASH);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::dolly::status::UNABLE_CANCEL_S3_DASH);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        VarModule::on_flag(boma.object(), vars::dolly::status::UNABLE_CANCEL_S3_DASH);
        MeterModule::watch_damage(agent.battle_object, true);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        VarModule::off_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            VarModule::on_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
         if VarModule::is_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {
            ATTACK(agent, 0, 0, Hash40::new("bust"), 6.5, 65, 40, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 6.5, 65, 40, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("arml"), 6.5, 65, 40, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 3, 0, Hash40::new("arml"), 6.5, 65, 40, 0, 80, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 4, 0, Hash40::new("top"), 6.5, 65, 40, 0, 80, 4.0, 0.0, 4.0, 9.5, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
         }
         else {
            ATTACK(agent, 0, 0, Hash40::new("bust"), 9.0, 75, 45, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 9.0, 75, 45, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("arml"), 9.0, 75, 45, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 3, 0, Hash40::new("arml"), 9.0, 75, 45, 0, 80, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
         }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        VarModule::off_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
         if VarModule::is_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {

         }
         else {
            ATTACK(agent, 3, 0, Hash40::new("arml"), 9.0, 75, 45, 0, 80, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
         }
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 4, false);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 4, false);
        if VarModule::is_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {

        }
         else {
            ATTACK(agent, 0, 0, Hash40::new("bust"), 8.0, 77, 30, 0, 86, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 9.0, 77, 40, 0, 86, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 2, 0, Hash40::new("arml"), 8.0, 77, 30, 0, 86, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 3, 0, Hash40::new("arml"), 8.0, 77, 30, 0, 86, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.5, 0.75, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
        VarModule::off_flag(boma.object(), vars::dolly::status::UNABLE_CANCEL_S3_DASH);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        VarModule::on_flag(boma.object(), vars::dolly::status::UNABLE_CANCEL_S3_DASH);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, true);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        VarModule::off_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            VarModule::on_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        //if VarModule::is_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) || VarModule::get_int(agent.battle_object, vars::shotos::instance::REPEAT_COUNT_LW) >= 2 {
        if VarModule::is_flag(agent.battle_object, vars::shotos::status::SHOULD_COMBOS_SCALE) {
            ATTACK(agent, 0, 0, Hash40::new("legl"), 4.0, 60, 90, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.75, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -1, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneel"), 4.0, 60, 90, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.75, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -1, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("kneel"), 4.0, 60, 90, 0, 30, 4.0, 6.0, 0.0, 0.0, None, None, None, 1.75, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -1, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
         }
         else {
            ATTACK(agent, 0, 0, Hash40::new("legl"), 5.0, 76, 36, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.75, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -1, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 1, 0, Hash40::new("kneel"), 5.0, 76, 36, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.75, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -1, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            ATTACK(agent, 2, 0, Hash40::new("kneel"), 5.0, 82, 36, 0, 25, 3.5, 5.0, 0.0, 0.0, None, None, None, 1.75, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -1, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(boma, 0, 7.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 7.0, false);
            AttackModule::set_add_reaction_frame(boma, 2, 7.0, false);
         }
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        MeterModule::watch_damage(agent.battle_object, false);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        /*
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            FT_MOTION_RATE(agent, 0.700);
        }
         else {
            FT_MOTION_RATE(agent, 1.250);
        }
        */
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks3", game_attacks3);
    agent.acmd("game_attackhi3", game_attackhi3);
    agent.acmd("game_attacklw3", game_attacklw3);
}
