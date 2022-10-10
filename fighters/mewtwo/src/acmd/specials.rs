
use super::*;


#[acmd_script( agent = "mewtwo", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_MEWTWO_CLIFF_HANG_DATA_SPECIAL_HI as u32);
	    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    install_acmd_scripts!(
        //special_hi_game
        bindball_shoot_game, 
    );
}

#[acmd_script( agent = "mewtwo_bindball", script = "game_shoot" , category = ACMD_GAME , low_priority)]
unsafe fn bindball_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 30, 80, 0, 50, 3.0, 0.0, -1.7, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 2.3, 0.0, -1.7, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        //ATTACK(ID=0, Part=0, Bone=Hash40::new("top"), Damage=1.0, Angle=361, KBG=140, FKB=0, BKB=0, Size=2.3, X=0.0, Y=-1.7, Z=2.5, X2=None, Y2=None, Z2=None, Hitlag=1.0, SDI=1.0, Clang_Rebound=*ATTACK_SETOFF_KIND_OFF, FacingRestrict=*ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G_d, Hitbits=*COLLISION_CATEGORY_MASK_FEB, CollisionPart=*COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new("collision_attr_bind_extra"), SFXLevel=*ATTACK_SOUND_LEVEL_S, SFXType=*COLLISION_SOUND_ATTR_ELEC, Type=*ATTACK_REGION_NONE);
	    //ATTACK(ID=1, Part=0, Bone=Hash40::new("top"), Damage=1.0, Angle=361, KBG=180, FKB=0, BKB=20, Size=3.0, X=0.0, Y=-1.7, Z=2.5, X2=None, Y2=None, Z2=None, Hitlag=1.0, SDI=1.0, Clang_Rebound=*ATTACK_SETOFF_KIND_OFF, FacingRestrict=*ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=*COLLISION_SITUATION_MASK_A, Hitbits=*COLLISION_CATEGORY_MASK_FEB, CollisionPart=*COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=Hash40::new("collision_attr_bind_extra"), SFXLevel=*ATTACK_SOUND_LEVEL_S, SFXType=*COLLISION_SOUND_ATTR_ELEC, Type=*ATTACK_REGION_NONE);
	    //AttackModule::set_lr_check_front_lr(0);
	    //AttackModule::set_no_finish_camera(0, true, false);
    }
}
