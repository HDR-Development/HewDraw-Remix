
use super::*;
use super::PikminInfo;

// TODO need to check if the "air" names are necessary
#[acmd_script( agent = "pikmin_pikmin", scripts = ["game_splwrespond", "game_splwrespond_b","game_splwrespond_v","game_splwrespond_w","game_splwrespond_y",   "game_splwairrespond", "game_splwairrespond_b","game_splwairrespond_v","game_splwairrespond_w","game_splwairrespond_y"] , category = ACMD_GAME , low_priority)]
unsafe fn pikmin_whistle_damage(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let variation = WorkModule::get_int(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let pikmin = PikminInfo::from(variation);
    if is_excute(fighter) && StatusModule::prev_status_kind(boma, 0) == *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING {
        AttackModule::clear_all(boma);
        let damage = 6.0;
        ATTACK(fighter, 0, 0, Hash40::new("head1"), damage * pikmin.damage, 90, 105, 0, 65, 5.0, 0.0, 0.0, 0.0, None, None, None, pikmin.hitlag * 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, damage * pikmin.shield_damage, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(pikmin.attr), *ATTACK_SOUND_LEVEL_L, pikmin.sound, *ATTACK_REGION_PIKMIN);
        
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        MotionModule::set_rate(boma, 0.5);
    }
    
}


#[acmd_script( agent = "pikmin_pikmin", scripts = ["game_spntakenoutstart", "game_spntakenoutstart_y", "game_spntakenoutstart_b", "game_spntakenoutstart_w", "game_spntakenoutstart_v"] , category = ACMD_GAME , low_priority)]
unsafe fn pikmin_special_n_pikmin(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, WorkModule::get_float(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_PULL_OUT_START_WORK_FLOAT_MOT_RATE));
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
    }
}


#[acmd_script( agent = "pikmin_pikmin", script = "game_spsthrown", category = ACMD_GAME, low_priority )]
unsafe fn game_spsthrown(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if (WorkModule::is_flag(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_TAKE_FROM_POCKET)) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 9.1, 361, 80, 0, 40, 4.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        } else {
            SEARCH(agent, 0, 0, Hash40::new("top"), 4.5, 0.0, 3.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.2, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FI, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 2, 0, Hash40::new("top"), 9.1, 361, 80, 0, 40, 2.2, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ENEMY, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_FLAG_LANDING);
    }
    frame(lua_state, 120.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_spsthrown_b", category = ACMD_GAME, low_priority )]
unsafe fn game_spsthrown_b(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if (WorkModule::is_flag(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_TAKE_FROM_POCKET)) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 6.5, 361, 100, 0, 40, 4.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        } else {
            SEARCH(agent, 0, 0, Hash40::new("top"), 4.5, 0.0, 3.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.2, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FI, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 2, 0, Hash40::new("top"), 6.5, 361, 100, 0, 40, 2.2, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ENEMY, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_FLAG_LANDING);
    }
    frame(lua_state, 120.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_spsthrown_v", category = ACMD_GAME, low_priority )]
unsafe fn game_spsthrown_v(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if (WorkModule::is_flag(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_TAKE_FROM_POCKET)) {
            ATTACK(agent, 0, 0, Hash40::new("hip"), 6.0, 361, 74, 0, 60, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        } else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 10.4, 361, 80, 0, 40, 4.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);   
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_FLAG_LANDING);
    }
    frame(lua_state, 120.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_spsthrown_w", category = ACMD_GAME, low_priority )]
unsafe fn game_spsthrown_w(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if (WorkModule::is_flag(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_TAKE_FROM_POCKET)) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 5.2, 361, 100, 0, 40, 4.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        } else {
            SEARCH(agent, 0, 0, Hash40::new("top"), 4.5, 0.0, 3.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.2, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FI, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 2, 0, Hash40::new("top"), 5.2, 361, 100, 0, 40, 2.2, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ENEMY, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_FLAG_LANDING);
    }
    frame(lua_state, 120.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_spsthrown_y", category = ACMD_GAME, low_priority )]
unsafe fn game_spsthrown_y(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if (WorkModule::is_flag(boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_TAKE_FROM_POCKET)) {
            ATTACK(agent, 2, 0, Hash40::new("top"), 6.5, 361, 100, 0, 40, 4.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
        } else {
            SEARCH(agent, 0, 0, Hash40::new("top"), 4.5, 0.0, 3.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.2, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FI, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
            ATTACK(agent, 2, 0, Hash40::new("top"), 6.5, 361, 100, 0, 40, 2.2, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ENEMY, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_FLAG_LANDING);
    }
    frame(lua_state, 120.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}


pub fn install() {
    install_acmd_scripts!(
        pikmin_whistle_damage,
        pikmin_special_n_pikmin,
        game_spsthrown,
        game_spsthrown_b,
        game_spsthrown_v,
        game_spsthrown_w,
        game_spsthrown_y,
    );
}

