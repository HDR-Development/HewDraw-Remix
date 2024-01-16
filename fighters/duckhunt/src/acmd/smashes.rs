use super::*;

#[acmd_script( agent = "duckhunt" , script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, 5, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_DELAY_FRAME,);
        WorkModule::set_int(boma, 6, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_RETICLE_DISPLAY_FRAME,);
        FT_MOTION_RATE_RANGE(fighter, 17.0, 57.0, 28.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 100, /*KBG*/ 145, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 5.3, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_NONE,);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.5, /*Angle*/ 85, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 8.3, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_NONE,);
    }
}

#[acmd_script( agent = "duckhunt" , script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE_RANGE(fighter, 12.0, 18.0, 4.0);
    if is_excute(fighter) {
        WorkModule::set_int(
            boma,
            5,
            *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_DELAY_FRAME,
        );
        WorkModule::set_int(
            boma,
            6,
            *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_RETICLE_DISPLAY_FRAME,
        );
        ATTACK(
            fighter,
            0,
            0,
            Hash40::new("top"),
            4.0,
            45,
            100,
            66,
            0,
            6.0,
            0.0,
            6.0,
            10.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_fire"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_PUNCH,
            *ATTACK_REGION_NONE,
        );
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE_RANGE(fighter, 18.0, 24.0, 4.0);
    if is_excute(fighter) {
        ATTACK(
            fighter,
            1,
            0,
            Hash40::new("top"),
            4.0,
            45,
            100,
            70,
            0,
            7.0,
            0.0,
            6.0,
            18.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_fire"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_PUNCH,
            *ATTACK_REGION_NONE,
        );
    }
    frame(lua_state, 24.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(
            fighter,
            2,
            0,
            Hash40::new("top"),
            9.0,
            361,
            131,
            0,
            50,
            9.0,
            0.0,
            6.0,
            26.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_fire"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_KICK,
            *ATTACK_REGION_NONE,
        );
    }
}

#[acmd_script( agent = "duckhunt" , script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(fighter, 21.0, 57.0, 24.0);
    if is_excute(fighter) {
        WorkModule::set_int(
            boma,
            5,
            *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_DELAY_FRAME,
        );
        WorkModule::set_int(
            boma,
            6,
            *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_RETICLE_DISPLAY_FRAME,
        );
        ATTACK(
            fighter,
            0,
            0,
            Hash40::new("top"),
            5.0,
            170,
            0,
            0,
            120,
            6.0,
            0.0,
            4.0,
            10.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
            *ATTACK_LR_CHECK_POS,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_fire"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_PUNCH,
            *ATTACK_REGION_NONE,
        );
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(
            fighter,
            1,
            0,
            Hash40::new("top"),
            5.0,
            35,
            70,
            0,
            70,
            7.0,
            0.0,
            8.0,
            -10.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_THRU,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_fire"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_PUNCH,
            *ATTACK_REGION_NONE,
        );
    }
}

pub fn install() {
    install_acmd_scripts!(
        duckhunt_attack_hi4_game,
        duckhunt_attack_s4_s_game,
        duckhunt_attack_lw4_game,
    );
}
