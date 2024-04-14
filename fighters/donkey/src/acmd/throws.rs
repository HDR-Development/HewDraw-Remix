use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.875);
    frame(lua_state, 7.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 6.0, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(14.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 3.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 10.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 6.0, 0.0, 7.0, 3.4, Some(0.0), Some(7.0), Some(18.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 3.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 6.0, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(-21.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(agent);
    wait(lua_state, 3.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
}

unsafe extern "C" fn game_throwff(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 65, 53, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 38, 19);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) { 
		let release_position = Vector3f{x:-5.0, y: 8.0, z: 24.0 };
		ModelModule::set_joint_translate(boma, Hash40::new("throw"), &release_position, false, false);
		ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
}

unsafe extern "C" fn game_throwfb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 65, 53, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(agent, 26, 14);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        REVERSE_LR(agent);
        let release_position = Vector3f{ x:5.0, y: 9.0, z: -12.0 };
        ModelModule::set_joint_translate(boma, Hash40::new("throw"), &release_position, false, false);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn game_throwfhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    let weight = WorkModule::get_param_float(boma.get_grabbed_opponent_boma(), hash40("weight"), 0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 36.0, 39.0 + 18.0 * (weight / 100.0 - 1.0));
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 90, 30, 0, 110, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, 1, 31);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
		let release_position = Vector3f{ x:0.0, y: 16.0, z: 0.0 };
		ModelModule::set_joint_translate(boma, Hash40::new("throw"), &release_position, false, false);
		ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
    frame(lua_state, 36.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_throwflw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 46, 45, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 30, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_THROW_FLAG_STOP);
        CHECK_FINISH_CAMERA(agent, 4, 2);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        SET_SPEED_EX(agent, 0.0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

unsafe extern "C" fn game_itemheavythrowf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        VarModule::set_flag(agent.object(), consts::vars::donkey::status::IS_NEUTRAL_TOSS, boma.stick_x().abs() < 0.33); 
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.5);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 2.0);
    }
    let is_neutral_toss = VarModule::is_flag(agent.object(), consts::vars::donkey::status::IS_NEUTRAL_TOSS); 
    let toss_frame = match is_neutral_toss {
        true => 16.0,
        false => 17.0
    };
    frame(lua_state, toss_frame);
    if is_excute(agent) {
        ItemModule::throw_item(boma, 45.0, 3.0, 1.0, 0, true, 0.0);
        FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn game_itemheavythrowb(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.5);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 2.0);
    }
    frame(lua_state, 19.5);
    if is_excute(agent) {
        // the exact *real* frame we are on needs to stay a whole
        // number in order for the barrel (or other item) to be 
        // released at an appropriate location.
        ItemModule::throw_item(boma, 135.0, 4.0, 1.0, 0, true, 0.0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.75);
    }
}

unsafe extern "C" fn game_itemheavythrowlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(agent, 8.0, 13.0, 9.0);
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ItemModule::throw_item(boma, 270.0, 4.0, 1.0, 0, true, 0.0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        let main_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let stick_add_x = agent.stick_x() * 0.5;
        
        // change to kinetic type fall and change to air situation
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), true);
        let kinetic = *FIGHTER_KINETIC_TYPE_FALL;
        KineticModule::change_kinetic(boma, kinetic);

        // pop up into the air
        SET_SPEED_EX(agent, (main_speed_x + stick_add_x) * PostureModule::lr(boma), 2.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }

    // when we reach the cancel frame, transition into fall instead
    let motion_kind = Hash40::new_raw(MotionModule::motion_kind(boma));
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma, motion_kind, true);
    frame(lua_state, cancel_frame);
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            agent.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
        }
    }
}

unsafe extern "C" fn effect_heavyitemthrowlw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("donkey_attack_arc"), Hash40::new("donkey_attack_arc"), Hash40::new("top"), -2, 16, -1, -5, -33, -102, 1.2, true, *EF_FLIP_YZ);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch);
    agent.acmd("game_catchdash", game_catchdash);
    agent.acmd("game_catchturn", game_catchturn);

    agent.acmd("game_throwff", game_throwff);

    agent.acmd("game_throwfb", game_throwfb);

    agent.acmd("game_throwfhi", game_throwfhi);

    agent.acmd("game_throwflw", game_throwflw);

    agent.acmd("game_itemheavythrowf", game_itemheavythrowf);

    agent.acmd("game_itemheavythrowb", game_itemheavythrowb);

    agent.acmd("game_itemheavythrowlw", game_itemheavythrowlw);
    agent.acmd("game_itemheavythrowlw4", game_itemheavythrowlw);
    agent.acmd("effect_itemheavythrowlw", effect_heavyitemthrowlw);
    agent.acmd("effect_itemheavythrowlw4", effect_heavyitemthrowlw);
}
