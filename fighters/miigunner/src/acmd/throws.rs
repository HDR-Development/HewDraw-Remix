
use smash::app::{sv_system, sv_animcmd::{frame, wait}, self, lua_bind::*};
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::utility::*;
use smash_script::*;
use smashline::*;
use smashline::*;
use smash_script::macros::*;
use crate::utils::hdr;
use crate::vars::*;


#[acmd_script( agent = "miigunner", script = "game_throwf" , category = ACMD_GAME , low_priority)]
unsafe fn throwf_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 65, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
		ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 48, /*KBG*/ 100, /*FKB*/ 140, /*BKB*/ 10, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 18.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		AttackModule::set_catch_only_all(boma, true, false);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CHECK_FINISH_CAMERA(fighter, 13.0, 0.0);
		//FighterCutInManager::set_throw_finish_zoom_rate(boma, 1.5);
		//FighterCutInManager::set_throw_finish_offset(boma, 9, -2, 0);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		AttackModule::clear_all(boma);
	}
}

#[acmd_script( agent = "miigunner", script = "game_throwb" , category = ACMD_GAME , low_priority)]
unsafe fn throwb_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		heavy_attack[hdr::get_player_number(boma)] = false;
		
		// Heavy throw
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
            heavy_attack[hdr::get_player_number(boma)] = true;
			FT_MOTION_RATE(fighter, 4.0);
			ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 7.0, /*Angle*/ 52, /*KBG*/ 56, /*FKB*/ 2, /*BKB*/ 75, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
        }
		// Normal throw
		else {
			ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 7.0, /*Angle*/ 50, /*KBG*/ 45, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
		}
		
		ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if heavy_attack[hdr::get_player_number(boma)] {
			FT_MOTION_RATE(fighter, 1.0);
		}
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		REVERSE_LR(fighter);
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		// Heavy throw
		if heavy_attack[hdr::get_player_number(boma)] {
			FT_MOTION_RATE(fighter, 0.75);
		}
		// Normal throw
		else {
			FT_MOTION_RATE(fighter, 0.5);
		}
	}
	frame(lua_state, 21.0);
	if is_excute(fighter) {
		// Explosion on heavy throw
		if heavy_attack[hdr::get_player_number(boma)] {
			FT_MOTION_RATE(fighter, 1.0);
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 142, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 98, /*Size*/ 18.0, /*X*/ 0.0, /*Y*/ 22.0, /*Z*/ -31.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BOMB);
		}
		else {
			FT_MOTION_RATE(fighter, 0.75);
		}
	}
	frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "miigunner", script = "effect_throwb" , category = ACMD_EFFECT , low_priority)]
unsafe fn throwb_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		// Heavy throw
		if heavy_attack[hdr::get_player_number(boma)] {
			EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 18.0, -27.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 10.0, 0.15, 0.15);
			LAST_EFFECT_SET_RATE(fighter, 1.25);
			
			EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 5.0, 0.0, 0.0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 0.15, 0.55, 10.0);
		}
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	frame(lua_state, 19.0);
	if is_excute(fighter) {
		// Heavy throw
		if heavy_attack[hdr::get_player_number(boma)] {
			EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 13, -13, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
		}
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		// Heavy throw
		if heavy_attack[hdr::get_player_number(boma)] {
			EFFECT(fighter, Hash40::new_raw(0x1287005d5d), Hash40::new("armr"), 5.3, 0, 0, 70, 0, 90, 0.9, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 10.0, 0.7, 0.7);
			EFFECT(fighter, Hash40::new_raw(0x168b3949cd), Hash40::new("armr"), 5.3, 0, 0, 90, 0, 90, 0.9, 0, 0, 0, 0, 0, 0, true);
			LAST_EFFECT_SET_COLOR(fighter, 10.0, 0.7, 0.7);
			LAST_EFFECT_SET_RATE(fighter, 1.6);
			
			EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 22.0, -31.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
		}
	}
}

#[acmd_script( agent = "miigunner", script = "sound_throwb" , category = ACMD_SOUND , low_priority)]
unsafe fn throwb_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
	}
	wait(lua_state, 8.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_miigunner_rnd_attack01"));
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
	}
	wait(lua_state, 9.0);
	if is_excute(fighter) {
		// Heavy throw
		if heavy_attack[hdr::get_player_number(boma)] {
			PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_n01"));
		}
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		// Heavy throw
		if heavy_attack[hdr::get_player_number(boma)] {
			PLAY_SE(fighter, Hash40::new("se_miigunner_special_c2_s04"));
		}
	}
}
   

pub fn install() {
    install_acmd_scripts!(
		throwf_game,
		throwb_game,
		throwb_effect,
		throwb_sound,
	);
}

