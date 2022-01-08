
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
use crate::hooks::sys_line::meter::*;


#[acmd_script( agent = "ryu", script = "game_specialn" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.909);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if     ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            FT_MOTION_RATE(fighter, 0.667);
        }
         else {
            WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
            FT_MOTION_RATE(fighter, 0.702);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        add_meter(fighter, boma, 3);
     }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }

}

#[acmd_script( agent = "ryu", script = "game_specialairn" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.909);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if     ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)|| ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            FT_MOTION_RATE(fighter, 0.667);
        }
         else {
            WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
            FT_MOTION_RATE(fighter, 0.714);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        add_meter(fighter, boma, 3);
     }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }

}

#[acmd_script( agent = "ryu", script = "game_specialsstart" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 2.000);
        ex_special[hdr::get_player_number(boma)] = false;
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 1.0, 3.5, 8.5, 8.5);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if get_meter_level(boma) >= 2 {
                use_meter_level(fighter, boma, 2);
                 ex_special[hdr::get_player_number(boma)] = true;
            }
        }
        if ex_special[hdr::get_player_number(boma)] {
		}
		else {
            //ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 25, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(4.5), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *COLLISION_SOUND_ATTR_NONE);
        }
		WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {  }
	frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }

}

#[acmd_script( agent = "ryu", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 5.5, 3.0, 9.0, 3.0);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        FT_MOTION_RATE(fighter, 1.000);
        if ex_special[hdr::get_player_number(boma)] {
            HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_INVINCIBLE);
            HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_INVINCIBLE);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if ex_special[hdr::get_player_number(boma)] {
			MotionModule::set_rate(boma, 2.500);
			ATTACK(fighter, /*ID*//*ID*/ 0, /*Part*/ /*Part*/ 0, /*Bone*/ /*Bone*/ Hash40::new("legl"), /*Damage*/ /*Damage*/ 3.0, /*Angle*/ /*Angle*/ 367, /*KBG*/ /*KBG*/ 100, /*FKB*/ /*FKB*/ 70, /*BKB*/ /*BKB*/ 0, /*Size*/ /*Size*/ 7.0, /*X*/ /*X*/ 0.0, /*Y*/ /*Y*/ 0.0, /*Z*/ /*Z*/ 0.0, /*X2*/ /*X2*/ Some(2.0), /*Y2*/ /*Y2*/ Some(0.0), /*Z2*/ /*Z2*/ Some(0.0), /*Hitlag*/ /*Hitlag*/ 1.5, /*SDI*/ /*SDI*/ 1.0, /*Clang_Rebound*/ /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ /*SetWeight*/ false, /*ShieldDamage*/ /*ShieldDamage*/ 0, /*Trip*/ /*Trip*/ 0.0, /*Rehit*/ /*Rehit*/ 3, /*Reflectable*/ /*Reflectable*/ false, /*Absorbable*/ /*Absorbable*/ false, /*Flinchless*/ /*Flinchless*/ false, /*DisableHitlag*/ /*DisableHitlag*/ false, /*Direct_Hitbox*/ /*Direct/Indirect*/ true, /*Ground_or_Air*/ /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ /*FriendlyFire*/ false, /*Effect*/ /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*//*ID*/ 1, /*Part*/ /*Part*/ 0, /*Bone*/ /*Bone*/ Hash40::new("kneel"), /*Damage*/ /*Damage*/ 3.0, /*Angle*/ /*Angle*/ 367, /*KBG*/ /*KBG*/ 100, /*FKB*/ /*FKB*/ 70, /*BKB*/ /*BKB*/ 0, /*Size*/ /*Size*/ 7.0, /*X*/ /*X*/ 0.0, /*Y*/ /*Y*/ 0.0, /*Z*/ /*Z*/ 0.0, /*X2*/ /*X2*/ Some(5.0), /*Y2*/ /*Y2*/ Some(0.0), /*Z2*/ /*Z2*/ Some(0.0), /*Hitlag*/ /*Hitlag*/ 1.5, /*SDI*/ /*SDI*/ 1.0, /*Clang_Rebound*/ /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ /*SetWeight*/ false, /*ShieldDamage*/ /*ShieldDamage*/ 0, /*Trip*/ /*Trip*/ 0.0, /*Rehit*/ /*Rehit*/ 3, /*Reflectable*/ /*Reflectable*/ false, /*Absorbable*/ /*Absorbable*/ false, /*Flinchless*/ /*Flinchless*/ false, /*DisableHitlag*/ /*DisableHitlag*/ false, /*Direct_Hitbox*/ /*Direct/Indirect*/ true, /*Ground_or_Air*/ /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ /*FriendlyFire*/ false, /*Effect*/ /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ /*Type*/ *ATTACK_REGION_KICK);
		}
		// EX Tatsumaki
		else {
			// Weak
			if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
				MotionModule::set_rate(boma,  1.250);
				ATTACK(fighter, /*ID*//*ID*/ 0, /*Part*/ /*Part*/ 0, /*Bone*/ /*Bone*/ Hash40::new("top"), /*Damage*/ /*Damage*/ 12.0, /*Angle*/ /*Angle*/ 68, /*KBG*/ /*KBG*/ 35, /*FKB*/ /*FKB*/ 0, /*BKB*/ /*BKB*/ 73, /*Size*/ /*Size*/ 3.5, /*X*/ /*X*/ 0.0, /*Y*/ /*Y*/ 12.5, /*Z*/ /*Z*/ 12.5, /*X2*/ /*X2*/ Some(0.0), /*Y2*/ /*Y2*/ Some(12.5), /*Z2*/ /*Z2*/ Some(2.0), /*Hitlag*/ /*Hitlag*/ 1.5, /*SDI*/ /*SDI*/ 1.0, /*Clang_Rebound*/ /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ /*SetWeight*/ false, /*ShieldDamage*/ /*ShieldDamage*/ 0, /*Trip*/ /*Trip*/ 0.0, /*Rehit*/ /*Rehit*/ 0, /*Reflectable*/ /*Reflectable*/ false, /*Absorbable*/ /*Absorbable*/ false, /*Flinchless*/ /*Flinchless*/ false, /*DisableHitlag*/ /*DisableHitlag*/ false, /*Direct_Hitbox*/ /*Direct/Indirect*/ true, /*Ground_or_Air*/ /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ /*FriendlyFire*/ false, /*Effect*/ /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ /*Type*/ *ATTACK_REGION_KICK);
			}
			// Medium
			if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
				MotionModule::set_rate(boma,  1.000);
				ATTACK(fighter, /*ID*//*ID*/ 0, /*Part*/ /*Part*/ 0, /*Bone*/ /*Bone*/ Hash40::new("top"), /*Damage*/ /*Damage*/ 13.0, /*Angle*/ /*Angle*/ 62, /*KBG*/ /*KBG*/ 67, /*FKB*/ /*FKB*/ 0, /*BKB*/ /*BKB*/ 80, /*Size*/ /*Size*/ 3.5, /*X*/ /*X*/ 0.0, /*Y*/ /*Y*/ 12.5, /*Z*/ /*Z*/ 12.5, /*X2*/ /*X2*/ Some(0.0), /*Y2*/ /*Y2*/ Some(12.5), /*Z2*/ /*Z2*/ Some(2.0), /*Hitlag*/ /*Hitlag*/ 1.5, /*SDI*/ /*SDI*/ 1.0, /*Clang_Rebound*/ /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ /*SetWeight*/ false, /*ShieldDamage*/ /*ShieldDamage*/ 0, /*Trip*/ /*Trip*/ 0.0, /*Rehit*/ /*Rehit*/ 0, /*Reflectable*/ /*Reflectable*/ false, /*Absorbable*/ /*Absorbable*/ false, /*Flinchless*/ /*Flinchless*/ false, /*DisableHitlag*/ /*DisableHitlag*/ false, /*Direct_Hitbox*/ /*Direct/Indirect*/ true, /*Ground_or_Air*/ /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ /*FriendlyFire*/ false, /*Effect*/ /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ /*Type*/ *ATTACK_REGION_KICK);
			}
			// Strong
			if ![*FIGHTER_RYU_STRENGTH_M, *FIGHTER_RYU_STRENGTH_W].contains(&WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH)) {
				MotionModule::set_rate(boma,  1.000);
				ATTACK(fighter, /*ID*//*ID*/ 0, /*Part*/ /*Part*/ 0, /*Bone*/ /*Bone*/ Hash40::new("top"), /*Damage*/ /*Damage*/ 14.0, /*Angle*/ /*Angle*/ 70, /*KBG*/ /*KBG*/ 71, /*FKB*/ /*FKB*/ 0, /*BKB*/ /*BKB*/ 80, /*Size*/ /*Size*/ 3.5, /*X*/ /*X*/ 0.0, /*Y*/ /*Y*/ 12.5, /*Z*/ /*Z*/ 12.5, /*X2*/ /*X2*/ Some(0.0), /*Y2*/ /*Y2*/ Some(12.5), /*Z2*/ /*Z2*/ Some(2.0), /*Hitlag*/ /*Hitlag*/ 1.5, /*SDI*/ /*SDI*/ 1.0, /*Clang_Rebound*/ /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ /*SetWeight*/ false, /*ShieldDamage*/ /*ShieldDamage*/ 0, /*Trip*/ /*Trip*/ 0.0, /*Rehit*/ /*Rehit*/ 0, /*Reflectable*/ /*Reflectable*/ false, /*Absorbable*/ /*Absorbable*/ false, /*Flinchless*/ /*Flinchless*/ false, /*DisableHitlag*/ /*DisableHitlag*/ false, /*Direct_Hitbox*/ /*Direct/Indirect*/ true, /*Ground_or_Air*/ /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ /*FriendlyFire*/ false, /*Effect*/ /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ /*Type*/ *ATTACK_REGION_KICK);
			}
		}
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if ex_special[hdr::get_player_number(boma)] {
            MotionModule::set_rate(boma, 2.500);
        }
         else {
            if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
                MotionModule::set_rate(boma, 1.250);
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 64, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ -11.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(-2.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
            }
            if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
                FT_MOTION_RATE(fighter, 1.000);
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.5, /*Angle*/ 50, /*KBG*/ 49, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ -11.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(-2.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
            }
            if ![*FIGHTER_RYU_STRENGTH_M, *FIGHTER_RYU_STRENGTH_W].contains(&WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH)) {
                FT_MOTION_RATE(fighter, 1.000);
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 50, /*KBG*/ 56, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ -11.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(-2.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
            }
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }

}

#[acmd_script( agent = "ryu", script = "game_specialsend" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if ex_special[hdr::get_player_number(boma)] {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 10.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(2.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 10.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(5.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

#[acmd_script( agent = "ryu", script = "game_specialairsstart" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 2.000);
        ex_special[hdr::get_player_number(boma)] = false;
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 1.0, 3.5, 8.5, 8.5);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if get_meter_level(boma) >= 2 {
                use_meter_level(fighter, boma, 2);
                 ex_special[hdr::get_player_number(boma)] = true;
            }
        }
        if ex_special[hdr::get_player_number(boma)] {
		}
		else {
            //ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 25, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 4.5, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(4.5), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *COLLISION_SOUND_ATTR_NONE);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {  }frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }

}

#[acmd_script( agent = "ryu", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if ex_special[hdr::get_player_number(boma)] {
            MotionModule::set_rate(boma, 2.500);
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 3.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 70, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(2.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 3.0, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 70, /*BKB*/ 0, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(5.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
        }
         else {
            if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
                MotionModule::set_rate(boma, 1.250);
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 68, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(2.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
            }
            if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
                FT_MOTION_RATE(fighter, 1.000);
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 11.0, /*Angle*/ 62, /*KBG*/ 67, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(2.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
            }
            if ![*FIGHTER_RYU_STRENGTH_M, *FIGHTER_RYU_STRENGTH_W].contains(&WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH)) {
                FT_MOTION_RATE(fighter, 1.000);
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 70, /*KBG*/ 71, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ 12.5, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(2.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
            }
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if  ex_special[hdr::get_player_number(boma)]  {
            MotionModule::set_rate(boma, 2.500);
        }
         else {
            if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
                MotionModule::set_rate(boma, 1.250);
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 64, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ -11.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(-2.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
            }
            if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
                FT_MOTION_RATE(fighter, 1.000);
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.5, /*Angle*/ 50, /*KBG*/ 49, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ -11.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(-2.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
            }
            if ![*FIGHTER_RYU_STRENGTH_M, *FIGHTER_RYU_STRENGTH_W].contains(&WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH)) {
                FT_MOTION_RATE(fighter, 1.000);
                ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 50, /*KBG*/ 56, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 12.5, /*Z*/ -11.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.5), /*Z2*/ Some(-2.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
            }
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }

}

#[acmd_script( agent = "ryu", script = "game_specialairsend" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_air_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if ex_special[hdr::get_player_number(boma)] {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 10.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(2.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
            ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 10.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(5.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_KICK, /*Type*/ *ATTACK_REGION_KICK);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}

#[acmd_script( agent = "ryu", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn ryu_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if(ex_special_scripting[hdr::get_player_number(boma)] != 3){
            ex_special[hdr::get_player_number(boma)] = false;
            ex_special_scripting[hdr::get_player_number(boma)] = 0;
        }
        if ex_special[hdr::get_player_number(boma)] && ex_special_scripting[hdr::get_player_number(boma)] == 3 {
            FT_MOTION_RATE(fighter, 0.500);
        }
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
	/*
        if ex_special_scripting[hdr::get_player_number(boma)] != 3 {
            if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                if get_meter_level(boma) >= 2 {
                    use_meter_level(fighter, boma, 2);
                     ex_special[hdr::get_player_number(boma)] = true;
                    ex_special_scripting[hdr::get_player_number(boma)] = 1;
                }
            }
        }
		*/
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        if ex_special[hdr::get_player_number(boma)] && ex_special_scripting[hdr::get_player_number(boma)] != 3 {
            ControlModule::clear_command(boma, true);
            if(WorkModule::is_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_NEAR_OPPONENT)){
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
            }
            else{
                ex_special_scripting[hdr::get_player_number(boma)] = 3;
            }
        }
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        if ex_special[hdr::get_player_number(boma)] && ex_special_scripting[hdr::get_player_number(boma)] == 3 {
            WorkModule::set_int(boma, *FIGHTER_RYU_STRENGTH_S, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
            ex_special_scripting[hdr::get_player_number(boma)] = 4;
        }
         else {
            WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
        }
        if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 13.0, /*Angle*/ 80, /*KBG*/ 58, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 7.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 14.0, /*Angle*/ 80, /*KBG*/ 64, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 7.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        if ![*FIGHTER_RYU_STRENGTH_M, *FIGHTER_RYU_STRENGTH_W].contains(&WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH)) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 15.0, /*Angle*/ 80, /*KBG*/ 69, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 7.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        if ex_special[hdr::get_player_number(boma)] && ex_special_scripting[hdr::get_player_number(boma)] == 4 {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 20.0, /*Angle*/ 80, /*KBG*/ 69, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 7.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_HEAVY, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 64, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 64, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        if ![*FIGHTER_RYU_STRENGTH_M, *FIGHTER_RYU_STRENGTH_W].contains(&WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH)) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 64, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.0, /*X*/ 4.0, /*Y*/ -0.4, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.5, /*X*/ 4.0, /*Y*/ -0.4, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
        if ![*FIGHTER_RYU_STRENGTH_M, *FIGHTER_RYU_STRENGTH_W].contains(&WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH)) {
            ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 6.0, /*X*/ 4.0, /*Y*/ -0.4, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_RYU_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }

}

pub fn install() {
    install_acmd_scripts!(
        ryu_special_n_game,
        ryu_special_air_n_game,
        ryu_special_s_start_game,
        ryu_special_s_game,
        ryu_special_s_end_game,
        ryu_special_air_s_start_game,
        ryu_special_air_s_game,
        ryu_special_air_s_end_game,
        ryu_special_hi_game,
    );
}
