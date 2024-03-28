use super::*;

unsafe extern "C" fn game_specialnshoot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
            let charge_level = WorkModule::get_int(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
            if charge_level <= 0 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANCHERRY), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 1 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANSTRAWBERRY), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 3 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANORANGE), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 5 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANAPPLE), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 7 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANMELON), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 9 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANBOSS), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 11 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANBELL), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 12 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANKEY), 0, 0, false, false);   
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            //WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_FLAG_THROW);
        }
    }
}

unsafe extern "C" fn expression_specialairsreturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ItemModule::set_attach_item_visibility(boma, false, *ATTACH_ITEM_GROUP_ALL as u8);
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_none") as i64);
        VisibilityModule::set_int64(boma, hash40("pizza") as i64, hash40("pizza_normal") as i64);
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("pizzapacman"), *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn game_specialairhiend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
}

unsafe extern "C" fn game_speciallwfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 2.4);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.5, 270, 70, 0, 20, 10.0, 0.0, 3.5, 0.0, None, None, None, 1.3, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnshoot", game_specialnshoot);
    agent.acmd("game_specialairnshoot", game_specialnshoot);

    agent.acmd("expression_specialairsreturn", expression_specialairsreturn);

    agent.acmd("game_specialairhiend", game_specialairhiend);

    agent.acmd("game_speciallwfailure", game_speciallwfailure);
    agent.acmd("game_specialairlwfailure", game_speciallwfailure);
}