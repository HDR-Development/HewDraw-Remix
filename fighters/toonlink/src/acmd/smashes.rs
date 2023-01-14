
use super::*;


#[acmd_script( agent = "toonlink", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn toonlink_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("sword2"), 16.0, 40, 105, 0, 25, 3.0, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("sword2"), 16.0, 40, 105, 0, 25, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("arml"),   16.0, 40, 105, 0, 25, 3.0, 1.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        //ATTACK(fighter, 3, 0, Hash40::new_raw(0x09aee445d1), 16.0, 40, 105, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "toonlink", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn toonlink_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 0.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0/(5.0-0.0));
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        FT_MOTION_RATE(fighter, 5.0/(11.0-5.0));
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 5.0/(14.0-11.0));
        ATTACK(fighter, 0, 0, Hash40::new("top"),    14.5, 100, 120, 0, 45, 4.9, 0.0, 6.0, 8.15, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("arml"),   14.5, 100, 120, 0, 45, 3.0, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("sword2"), 14.5, 100, 120, 0, 45, 3.4, 5.5, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        //ATTACK(fighter, 3, 0, Hash40::new("shoulderl"), 15.0, 96, 100, 0, 43, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 11.3);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("arml"),   11.0, 100, 110, 0, 40, 3.0, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("sword2"), 11.0, 100, 110, 0, 40, 3.4, 5.5, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }    
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 22.0/(47.0-14.0));
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    
}

#[acmd_script( agent = "toonlink", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn toonlink_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0/(9.0-5.0));
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        //Ground (links together)
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 6.0, 170, 95, 130, 0, 3.0, -1.0, -1.5, 0.0, None, None, None, 0.6, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 168, 100, 130, 0, 3.0, 0.0, 2.9, 11.3, None, None, None, 0.6, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 170, 95, 130, 0, 3.0, 0.0, 2.2, 16.225, None, None, None, 0.6, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        //Air (sends up)
        ATTACK(fighter, 3, 0, Hash40::new("top"), 7.5, 70, 80, 0, 50, 3.0, 0.5, 2.9, 11.3, None, None, None, 0.6, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 7.5, 70, 80, 0, 50, 3.0, 0.5, 2.2, 16.225, None, None, None, 0.6, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 5.0/(17.0-11.0));
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 50, 115, 0, 50, 3.0, 0.0, 4.2, -9.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.5, 50, 115, 0, 50, 3.0, 0.0, 4.2, -15.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 7.5, 50, 115, 0, 50, 3.5, -1.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.880);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    
}


// Not installed. I'll get help relearning how to reimplement effects
//#[acmd_script( agent = "toonlink", script = "effect_attacks4" , category = ACMD_EFFECT , low_priority)]
//unsafe fn toonlink_attack_s4_s_effect(fighter: &mut L2CAgentBase) {
   // let lua_state = fighter.lua_state_agent;
  //  let boma = fighter.boma();
  //  frame(fighter.lua_state_agent, 2.0);
  //  if macros::is_excute(fighter) {
  //      macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), 10.0, 0, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
   // }
   // frame(fighter.lua_state_agent, 14.0);
//  if macros::is_excute(fighter) {
  //      AFTER_IMAGE4_ON_arg29(Hash40::new("toonlink_sword3"), Hash40::new("toonlink_sword2"), 5, Hash40::new("sword1"), 0.5, 0, -0.5, Hash40::new("sword1"), 10.57, -0.13, -0.12, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, 0.2);
  //      macros::EFFECT_FOLLOW(fighter, Hash40::new("toonlink_kaiten_flare"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
   // }
   // frame(fighter.lua_state_agent, 15.0);
   // if macros::is_excute(fighter) {
   //     macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
   // }
    //frame(fighter.lua_state_agent, 20.0);
    //if macros::is_excute(fighter) {
    //    macros::AFTER_IMAGE_OFF(fighter, 10);
    //}
    //frame(fighter.lua_state_agent, 23.0);
    //if macros::is_excute(fighter) {
    //    macros::EFFECT_OFF_KIND(fighter, Hash40::new("toonlink_kaiten_flare"), false, false);
    //}
//}

pub fn install() {
    install_acmd_scripts!(
        toonlink_attack_s4_s_game,
        toonlink_attack_hi4_game,
        toonlink_attack_lw4_game,
        //toonlink_attack_s4_s_effect,
    );
}

