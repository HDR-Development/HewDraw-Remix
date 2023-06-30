
use super::*;


#[acmd_script( agent = "donkey", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.875);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        CATCH(fighter, 0, Hash40::new("top"), 6.8, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(14.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "donkey", script = "game_catchdash" , category = ACMD_GAME , low_priority)]
unsafe fn catch_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 6.8, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(18.200001), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "donkey", script = "game_catchturn" , category = ACMD_GAME , low_priority)]
unsafe fn catch_turn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 6.8, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(-21.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "donkey", script = "game_throwff" , category = ACMD_GAME , low_priority)]
unsafe fn game_throwff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 65, 53, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 38, 19);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) { 
		let release_position = Vector3f{x:-5.0, y: 8.0, z: 24.0 };
		ModelModule::set_joint_translate(boma, Hash40::new("throw"), &release_position, false, false);
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}

}

#[acmd_script( agent = "donkey", script = "game_throwfb" , category = ACMD_GAME , low_priority)]
unsafe fn game_throwfb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 65, 52, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(fighter, 26, 14);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        let release_position = Vector3f{ x:5.0, y: 9.0, z: -12.0 };
        ModelModule::set_joint_translate(boma, Hash40::new("throw"), &release_position, false, false);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    
}

#[acmd_script( agent = "donkey", script = "game_throwfhi" , category = ACMD_GAME , low_priority)]
unsafe fn game_throwfhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 90, 30, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 1, 31);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) { 
		let release_position = Vector3f{x:0.0, y: 16.0, z: 0.0 };
		ModelModule::set_joint_translate(boma, Hash40::new("throw"), &release_position, false, false);
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

#[acmd_script( agent = "donkey", script = "game_throwflw" , category = ACMD_GAME , low_priority)]
unsafe fn game_throwflw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 46, 45, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 30, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_THROW_FLAG_STOP);
        CHECK_FINISH_CAMERA(fighter, 4, 2);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        SET_SPEED_EX(fighter, 0.0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    
}

#[acmd_script( agent = "donkey", script = "game_itemheavythrowf" , category = ACMD_GAME , low_priority)]
unsafe fn heavy_item_throw_f(fighter: &mut L2CAgentBase) {
  let lua_state = fighter.lua_state_agent;
  let boma = fighter.boma();
  frame(lua_state, 2.0);
  if is_excute(fighter) {
    FT_MOTION_RATE(fighter, 0.5);
  }
  frame(lua_state, 10.0);
  if is_excute(fighter) {
    FT_MOTION_RATE(fighter, 1.0);
  }
  frame(lua_state, 15.0);
  if is_excute(fighter) {
    FT_MOTION_RATE(fighter, 2.0);
  }
  frame(lua_state, 17.0);
  if is_excute(fighter) {
    ItemModule::throw_item(boma, 55.0, 3.0, 1.0, 0, true, 0.0);
    FT_MOTION_RATE(fighter, 1.0);
  }
}

#[acmd_script( agent = "donkey", script = "game_itemheavythrowb" , category = ACMD_GAME , low_priority)]
unsafe fn heavy_item_throw_b(fighter: &mut L2CAgentBase) {
  let lua_state = fighter.lua_state_agent;
  let boma = fighter.boma();
  frame(lua_state, 2.0);
  if is_excute(fighter) {
    FT_MOTION_RATE(fighter, 0.5);
  }
  frame(lua_state, 10.0);
  if is_excute(fighter) {
    FT_MOTION_RATE(fighter, 1.0);
  }
  frame(lua_state, 19.0);
  if is_excute(fighter) {
    FT_MOTION_RATE(fighter, 2.0);
  }
  frame(lua_state, 19.5);
  if is_excute(fighter) {
    // the exact *real* frame we are on needs to stay a whole
    // number in order for the barrel (or other item) to be 
    // released at an appropriate location.
    ItemModule::throw_item(boma, 135.0, 4.0, 1.0, 0, true, 0.0);
  }
  frame(lua_state, 20.0);
  if is_excute(fighter) {
    FT_MOTION_RATE(fighter, 0.75);
  }
}

#[acmd_script( agent = "donkey", scripts = ["game_itemheavythrowlw", "game_itemheavythrowlw4"], category = ACMD_GAME, low_priority )]
unsafe fn game_itemheavythrowlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(fighter, 8.0, 13.0, 9.0);
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ItemModule::throw_item(boma, 270.0, 4.0, 1.0, 0, true, 0.0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        let main_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let stick_add_x = fighter.stick_x();
        
        // change to kinetic type fall and change to air situation
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        let kinetic = *FIGHTER_KINETIC_TYPE_FALL;
        KineticModule::change_kinetic(fighter.module_accessor, kinetic);

        // pop up into the air
        SET_SPEED_EX(fighter, (main_speed_x + stick_add_x) * PostureModule::lr(fighter.module_accessor), 2.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }

    // when we reach the cancel frame, transition into fall instead
    let motion_kind = Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor));
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(
        boma, motion_kind, true);
    frame(lua_state, cancel_frame);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
        }
    }
}

#[acmd_script( agent = "donkey", scripts = ["effect_itemheavythrowlw", "effect_itemheavythrowlw4"], category = ACMD_EFFECT, low_priority )]
unsafe fn effect_heavyitemthrowlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_attack_arc"), Hash40::new("donkey_attack_arc"), Hash40::new("top"), -2, 16, -1, -5, -33, -102, 1.2, true, *EF_FLIP_YZ);
    }
}

pub fn install() {
    install_acmd_scripts!(
        catch,
        catch_dash,
        catch_turn,
        game_throwff,
        game_throwfb,
        game_throwfhi,
        game_throwflw,
        heavy_item_throw_f,
        heavy_item_throw_b,
        game_itemheavythrowlw,
        effect_heavyitemthrowlw,
    );
}

