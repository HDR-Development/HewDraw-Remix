
use super::*;

#[acmd_script( agent = "sonic", script = "game_attack11" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
frame(lua_state, 2.0);
if is_excute(fighter) {
FighterAreaModuleImpl::enable_fix_jostle_area(boma, 3.0, 3.0);
}frame(lua_state, 3.0);
if is_excute(fighter) {
ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 1.4, 0.0, 7.0, 6.7, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 20, 0, 20, 1.4, 0.0, 7.0, 9.2, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 15, 0, 20, 1.6, 0.0, 7.0, 12.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 361, 15, 0, 20, 1.6, 0.0, 7.0, 12.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
AttackModule::set_add_reaction_frame(boma, /*ID*/ 0, /*Frames*/ 8.0, /*Unk*/ false);
AttackModule::set_add_reaction_frame(boma, /*ID*/ 1, /*Frames*/ 8.0, /*Unk*/ false);
AttackModule::set_add_reaction_frame(boma, /*ID*/ 2, /*Frames*/ 8.0, /*Unk*/ false);
AttackModule::set_add_reaction_frame(boma, /*ID*/ 3, /*Frames*/ 8.0, /*Unk*/ false);

}wait(lua_state, 1.0);
if is_excute(fighter) {
AttackModule::clear_all(boma);
}frame(lua_state, 6.0);
if is_excute(fighter) {
WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
}frame(lua_state, 10.0);
if is_excute(fighter) {
WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
}

}

#[acmd_script( agent = "sonic", script = "game_attack12" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_attack_12_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
    ATTACK(fighter, 0, 0, Hash40::new("kneer"), 1.5, 361, 20, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    ATTACK(fighter, 1, 0, Hash40::new("kneer"), 1.5, 361, 20, 0, 20, 2.3, 2.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    ATTACK(fighter, 2, 0, Hash40::new("kneer"), 1.5, 361, 20, 0, 15, 2.8, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    AttackModule::set_add_reaction_frame(boma, /*ID*/ 0, /*Frames*/ 4.0, /*Unk*/ false);
    AttackModule::set_add_reaction_frame(boma, /*ID*/ 1, /*Frames*/ 4.0, /*Unk*/ false);
    AttackModule::set_add_reaction_frame(boma, /*ID*/ 2, /*Frames*/ 4.0, /*Unk*/ false);
    }wait(lua_state, 6.0);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.7)
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
    WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script( agent = "sonic", script = "game_attack13" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_attack_13_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    MotionModule::set_rate(boma, 0.3);
if is_excute(fighter) {
ATTACK(fighter, 0, 0, Hash40::new("kneel"), 4.0, 361, 110, 0, 50, 2.6, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
ATTACK(fighter, 1, 0, Hash40::new("kneel"), 4.0, 361, 110, 0, 50, 2.6, 3.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
ATTACK(fighter, 2, 0, Hash40::new("kneel"), 4.0, 361, 110, 0, 50, 5.6, 5.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
}
wait(lua_state, 2.0);
if is_excute(fighter) {
MotionModule::set_rate(boma, 2.5);
AttackModule::clear_all(boma);
}
}


#[acmd_script( agent = "sonic", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
       }

    frame(lua_state, 10.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
        }

    

    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 0.5, 361, 100, 40, 0, 6.0, 5.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 0.5, 361, 100, 40, 0, 6.0, 1.0, 3.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }

    frame(lua_state, 12.2);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }


    wait(lua_state, 0.5);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 6.0, 55, 80, 0, 85, 6.0, 5.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 6.0, 55, 80, 0, 85, 5.0, 0.5, 0.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
      }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
        AttackModule::clear_all(boma);
    }

}

pub fn install() {
    install_acmd_scripts!(
        sonic_attack_dash_game,
        sonic_attack_11_game,
        sonic_attack_12_game,
        sonic_attack_13_game
    );
}

