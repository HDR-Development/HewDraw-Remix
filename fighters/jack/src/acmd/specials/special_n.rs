use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            let y_offset = if agent.is_motion(Hash40::new("jack_special_n")) { 5.0 } else if agent.is_motion(Hash40::new("jack_special_n_2")) { 4.0 } else { 4.5 };
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 9, 2.5, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(20.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 5, 2.5, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(36.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
        else {
            let y_offset = if agent.is_motion(Hash40::new("special_n")) { 11.0 } else if agent.is_motion(Hash40::new("special_n_2")) { 8.5 } else { 9.0 };
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 9, 2.5, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(20.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 5, 2.5, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(36.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if is_excute(agent) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        frame(lua_state, 26.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
        }
        frame(lua_state, 27.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
        }
        frame(lua_state, 31.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        }
        frame(lua_state, 34.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
        } 
    }
    else {
        frame(lua_state, 14.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
        }
        frame(lua_state, 17.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
        }
        frame(lua_state, 21.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
        }
        frame(lua_state, 31.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        }
        frame(lua_state, 34.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
        }
    }
}

unsafe extern "C" fn game_specialn_ex(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let y_offset = if agent.is_motion(Hash40::new("special_n2")) { 11.0 } else if agent.is_motion(Hash40::new("special_n2_2")) { 8.5 } else { 9.0 };
    frame(lua_state, 12.0);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 50, 0, 60, 2.8, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(20.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.8, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(36.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(20.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(36.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(20.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(36.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, y_offset, 9.0, Some(0.0), Some(y_offset), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        frame(lua_state, 31.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
        }
        frame(lua_state, 32.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
        }
        frame(lua_state, 36.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        }
        frame(lua_state, 39.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
        }
    }
    else {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
        }
        frame(lua_state, 28.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
        }
        frame(lua_state, 31.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        }
        frame(lua_state, 34.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
        }
    }
}

unsafe extern "C" fn game_specialnescapeb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(14.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.5, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(26.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(69.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(14.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(26.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(69.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if is_excute(agent) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        frame(lua_state, 39.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
        }
        frame(lua_state, 42.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
        }
    }
    else {
        frame(lua_state, 32.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
        }
        frame(lua_state, 36.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
        }
        frame(lua_state, 39.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
        }
        frame(lua_state, 42.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
        }
    }
}

unsafe extern "C" fn game_specialnescapeb_ex(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 22.0);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 75, 0, 5, 2.8, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(20.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 361, 50, 0, 1, 2.8, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(26.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(69.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 28.0);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(20.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(26.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(69.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 75, 0, 5, 2.8, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(20.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 50, 0, 1, 2.8, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(26.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 7.5, 9.0, Some(0.0), Some(7.5), Some(69.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        frame(lua_state, 43.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
        }
        frame(lua_state, 45.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
        }
    }
    else {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
        }
        frame(lua_state, 38.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
        }
        frame(lua_state, 40.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
        }
        frame(lua_state, 42.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
        }
    }
}

unsafe extern "C" fn game_specialnescapef(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 67, 0, 30, 2.5, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(14.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 45, 0, 4, 2.5, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(36.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.5, 0.0, 8.5, 9.0, Some(0.0), Some(8.5), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 67, 0, 30, 2.5, 0.0, 4.5, 9.0, Some(0.0), Some(4.5), Some(14.0), 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 45, 0, 4, 2.5, 0.0, 4.5, 9.0, Some(0.0), Some(4.5), Some(36.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.5, 0.0, 4.5, 9.0, Some(0.0), Some(4.5), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if is_excute(agent) {
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        frame(lua_state, 33.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
        }
        frame(lua_state, 36.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
        }
    }
    else {
        frame(lua_state, 30.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
        }
        frame(lua_state, 31.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
        }
        frame(lua_state, 33.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
        }
        frame(lua_state, 36.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
        }
    }
}

unsafe extern "C" fn game_specialnescapef_ex(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 30, 120, 0, 30, 2.5, 0.0, 10.5, 9.0, Some(0.0), Some(10.5), Some(69.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 8, 4);
    }
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 50, 0, 50, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(20.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 50, 0, 4, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(36.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(20.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(36.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 32.0);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(20.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(36.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.8, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 4, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        frame(lua_state, 40.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
        }
        frame(lua_state, 41.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
        }
    }
    else {
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
        }
        frame(lua_state, 34.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
        }
        frame(lua_state, 36.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
        }
        frame(lua_state, 38.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
        }
    }
}

unsafe extern "C" fn game_specialnjump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        KineticModule::add_speed(boma, &Vector3f{x: 0.75, y: 0.4, z: 0.0});
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 365, 70, 0, 10, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
    }
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 365, 70, 0, 10, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
        }
    }
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
        }
    }
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 365, 70, 0, 10, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
    }
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 365, 70, 0, 10, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
        }
    }
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
        }
    }
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 365, 70, 0, 10, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 50, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
    }
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 365, 70, 0, 10, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 50, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.8, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
        }
    }
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 50, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
        }
    }
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -0.8, z: 0.0});
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 125, 70, 0, 50, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 90, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 0.8, 45, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("top"), 0.8, 45, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 2, 5, 4);
        FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
    }
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 125, 70, 0, 60, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 90, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 0.8, 45, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("top"), 0.8, 45, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 2, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
        }
    }
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairndown(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    for _ in 0..20 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 50, 80, 0, 25, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-15.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 45, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-45.0), Some(-2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.2, 55, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-45.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("top"), 1.2, 45, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-45.0), Some(2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
        }
        if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 50, 80, 0, 25, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-15.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 45, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-45.0), Some(-2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 2, 0, Hash40::new("top"), 1.2, 55, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-45.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 3, 0, Hash40::new("top"), 1.2, 45, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-45.0), Some(2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
            }
        }
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 6.0);
        if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 50, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-17.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 45, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 2, 0, Hash40::new("top"), 0.8, 55, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 3, 0, Hash40::new("top"), 0.8, 45, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
            }
        }
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 6.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn1", game_specialn, Priority::Low);
    agent.acmd("game_specialn2", game_specialn, Priority::Low);
    agent.acmd("game_specialn3", game_specialn, Priority::Low);
    agent.acmd("game_specialairn1", game_specialn, Priority::Low);
    agent.acmd("game_specialairn2", game_specialn, Priority::Low);
    agent.acmd("game_specialairn3", game_specialn, Priority::Low);
    
    agent.acmd("game_specialn1_ex", game_specialn_ex, Priority::Low);
    agent.acmd("game_specialn2_ex", game_specialn_ex, Priority::Low);
    agent.acmd("game_specialn3_ex", game_specialn_ex, Priority::Low);
    agent.acmd("game_specialairn1_ex", game_specialn_ex, Priority::Low);
    agent.acmd("game_specialairn2_ex", game_specialn_ex, Priority::Low);
    agent.acmd("game_specialairn3_ex", game_specialn_ex, Priority::Low);

    agent.acmd("game_specialnescapeb", game_specialnescapeb, Priority::Low);
    agent.acmd("game_specialairnescapeb", game_specialnescapeb, Priority::Low);

    agent.acmd("game_specialnescapebex", game_specialnescapeb_ex, Priority::Low);
    agent.acmd("game_specialairnescapebex", game_specialnescapeb_ex, Priority::Low);

    agent.acmd("game_specialnescapef", game_specialnescapef, Priority::Low);
    agent.acmd("game_specialairnescapef", game_specialnescapef, Priority::Low);

    agent.acmd("game_specialnescapefex", game_specialnescapef_ex, Priority::Low);
    agent.acmd("game_specialairnescapefex", game_specialnescapef_ex, Priority::Low);

    agent.acmd("game_specialnjump", game_specialnjump, Priority::Low);

    agent.acmd("game_specialairndown", game_specialairndown, Priority::Low);
}