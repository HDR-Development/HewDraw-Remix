
use super::*;

#[acmd_script( agent = "pacman", script = "game_specialnshoot" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_n_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
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

#[acmd_script( agent = "pacman", script = "game_specialairnshoot" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_air_n_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
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

#[acmd_script( agent = "pacman", script = "expression_specialairsreturn", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_specialairsreturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ItemModule::set_attach_item_visibility(boma, false, *ATTACH_ITEM_GROUP_ALL as u8);
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_none") as i64);
        VisibilityModule::set_int64(boma, hash40("pizza") as i64, hash40("pizza_normal") as i64);
        HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("pizzapacman"), *HIT_STATUS_NORMAL);
    }
}

#[acmd_script( agent = "pacman", script = "game_specialairhiend" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_air_hi_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
}

#[acmd_script( agent = "pacman", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT, false, -1);
    }
}

#[acmd_script( agent = "pacman", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT, false, -1);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[acmd_script( agent = "pacman", script = "game_speciallwfailure" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_lw_failure_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.4);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 270, 70, 0, 20, 10.0, 0.0, 3.5, 0.0, None, None, None, 1.6, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "pacman", script = "game_specialairlwfailure" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_air_lw_failure_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.4);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 270, 70, 0, 20, 10.0, 0.0, 3.5, 0.0, None, None, None, 1.6, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        //pacman_special_n_shoot_game,
        //pacman_special_air_n_shoot_game,
        expression_specialairsreturn,
        pacman_special_air_hi_end_game,
        game_speciallw,
        game_specialairlw,
        pacman_special_lw_failure_game,
        pacman_special_air_lw_failure_game,
    );
}

