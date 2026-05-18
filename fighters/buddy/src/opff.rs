use super::*;
use globals::*;

pub const HUD_DISPLAY_TIME_MAX: i32 = 90;
const FEATHERS_RED_COOLDOWN_GROUND_RATE: f32 = 1.25;
pub const FEATHERS_RED_COOLDOWN_MAX: f32 = 450.0;
const BEAKBOMB_END_FRAME: i32 = 25; // Dash timer is shared between ground and air in vl.prc
 
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn blue_eggs_land_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) {
        let landing_lag = 12.0;
        fighter.check_land_cancel(Some(landing_lag));
    }
}

// Banjo Dair bounce
unsafe fn dair_bounce(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion(Hash40::new("attack_air_lw"))
    && fighter.motion_frame() < 46.0 {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 45.0, true, true, false);
        }
    }
}

unsafe fn dash_attack_jump_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_DASH)
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        if MotionModule::frame(fighter.module_accessor) >= 41.0 {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
        }
        else if MotionModule::frame(fighter.module_accessor) >= 27.0 {
            fighter.check_jump_cancel(false, false, true);
        }
    }
}

unsafe fn indicator_breegull_fatigue(fighter: &mut L2CFighterCommon) {
	let eggs_shot = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
    let eggs_Weak = WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_n"),hash40("bakyun_power_down_1_num"));
	if (eggs_shot >= eggs_Weak && !fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_END)) {
        let eggs_Weakest = WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_n"), hash40("bakyun_power_down_2_num"));
		let sweatRate = if eggs_shot < eggs_Weakest { 25.0 } else { 15.0 };
		let sweatSize = if eggs_shot < eggs_Weakest { 0.625 } else { 0.9 };
		let modulo = fighter.motion_frame() % sweatRate;
		if modulo < 1.0 {
			EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_sweat"), Hash40::new("top"), 0, 8.5, 7.5, 0, 0, 0, sweatSize, true);
		}
	}
}

unsafe fn beakbomb_update(fighter: &mut L2CFighterCommon) {
    // While in Beakbomb / Wonderwing
    if VarModule::is_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_ACTIVE) {
        if fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH) {
            beakbomb_control(fighter);
            //beakbomb_checkForHit(fighter,boma);
            beakbomb_checkforground(fighter);
            beakbomb_checkforcancel(fighter);
            GroundModule::set_attach_ground(fighter.module_accessor, false);
            VarModule::add_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_FRAME, 1);
        }
        else if fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL) {
            beakbomb_wall(fighter);
        }
        else if !fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH,
            *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL
        ]) {
            // If out of SideSpecial, then set BEAKBOMB_ACTIVE to false
            VarModule::off_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_ACTIVE);
        }
    }
}

// Check to see if Banjo hit a shield during beakbomb.
// unsafe fn beakbomb_checkForHit(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
//     let has_hit_shield = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
//     if (!has_hit_shield) {return;}
    
//     if (fighter.motion_frame() > 0.0) //If motion frame is 0, game crashes
//     {
//         let start_frame = 7.0;
//         let weak_frame = 21.0;
//         VarModule::set_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_BOUNCE_TYPE,
//             if (fighter.motion_frame() >= weak_frame) {1} else {2}
//         );
//         fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, false);
//         //MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, start_frame, true, true, false);
//     }
// }

unsafe fn beakbomb_control(fighter: &mut L2CFighterCommon) {
    // If past the end frame, transition into end
    if VarModule::get_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_FRAME) >= BEAKBOMB_END_FRAME {
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
        return;
    }

    // Do not update flight during hitstop
    if fighter.is_in_hitlag() { return; }

    // Movement
    let motion_factor = 0.425;
    let motion_offset = -0.125;
    let motion_vec = Vector3f::new(
        0.0,
        motion_offset + (VarModule::get_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_ANGLE) * motion_factor),
        0.0
    );
    KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
}

// Check if landed on the ground
unsafe fn beakbomb_checkforground(fighter: &mut L2CFighterCommon) {
    if !fighter.is_situation(*SITUATION_KIND_GROUND) { return; }

    let fail_safeFrames = 5;
    let fail_cutoff = 25;
    //let can_damage = fail_safeFrames < VarModule::get_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_FRAME);
    if VarModule::get_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_FRAME) < fail_cutoff {
        // Add damage
        // if can_damage { DamageModule::add_damage(fighter.module_accessor, 10.0,0); }

        KineticModule::clear_speed_all(fighter.module_accessor);
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, true);
        PLAY_SE(fighter, Hash40::new("vc_buddy_missfoot01"));
    }
    else {
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
    }
}

unsafe fn beakbomb_checkforcancel(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
    || !fighter.is_situation(*SITUATION_KIND_AIR) { return; }

    if fighter.is_button_on(Buttons::Guard) && fighter.motion_frame() >= 11.0 {
         fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
    }
}

// Recoil for bouncing off walls/shields
unsafe fn beakbomb_wall(fighter: &mut L2CFighterCommon) {
    /*
    if fighter.is_motion(Hash40::new("special_air_s_wall"))
    && fighter.motion_frame() < 7.0
    && fighter.motion_frame() > 0.0 {
        let x_bounce = match VarModule::get_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_BOUNCE_TYPE) {
            0 => -1.0,
            2 => -2.0,
            _ => -1.5
        };
        let y_bounce = if VarModule::get_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_BOUNCE_TYPE) < 1 { 0.5 } else { 1.0 };
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        SET_SPEED_EX(fighter, x_bounce, y_bounce, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        VarModule::off_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_ACTIVE);
    } 
    */
}

unsafe fn breegull_bayonet(fighter: &mut L2CFighterCommon) {
    //special_n_shoot_fire
    //special_air_n_shoot_fire
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    let motion_partial = MotionModule::motion_kind_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY);
    if fighter.is_status_one_of(&[
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_F,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_B,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_TURN,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_LANDING
    ]) && fighter.is_situation(*SITUATION_KIND_GROUND) {

        if motion_partial == hash40("special_n_shoot_upper_fire") {
            let frame_partial = MotionModule::frame_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY);
            let disable_frame = 3.0; //frame before egg fires
            let disable_bayonet = (!CancelModule::is_enable_cancel(fighter.module_accessor) && frame_partial >= disable_frame);
            VarModule::set_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_DISABLE,disable_bayonet);
        }
        else {
            VarModule::off_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_DISABLE);
        }
        let is_csticking = ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0;
        if (is_csticking && !VarModule::is_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_DISABLE)) {
            //println!("Bayonet");
            //VarModule::on_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_DISABLE);
            //VarModule::set_int(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_EGGS_FIRED,*FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
            fighter.change_status(statuses::buddy::SPECIAL_N_BAYONET.into(), false.into());
        }
    }
    /* 
    if VarModule::is_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_DISABLE) {
        if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_S3) {
            if fighter.motion_frame() < 21.0 { return; }
            fighter.change_status(statuses::buddy::SPECIAL_N_BAYONET.into(), false.into());

            let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let currentEggs = BAYONET_EGGS[entry]; // VarModule::get_int(fighter.battle_object, vars::buddy::instance::BAYONET_EGGS);
            WorkModule::set_int(fighter.module_accessor,
                currentEggs,
                *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT
            );

            VarModule::off_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_DISABLE);
        }
    }
    else if fighter.is_status_one_of(&[
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_F,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_B,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_TURN,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_LANDING
    ]) {
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let currentEggs = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
        // VarModule::set_int(fighter.battle_object, vars::buddy::instance::BAYONET_EGGS, currentEggs);
        BAYONET_EGGS[entry] = currentEggs;
    }
    */
}

unsafe fn buddy_meter_controller(fighter: &mut L2CFighterCommon) {
    let in_Air = fighter.is_situation(*SITUATION_KIND_AIR);
    let current_cool = VarModule::get_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN);
	if current_cool > 0.0 {
        // Only start cooldown if on ground
        if current_cool < FEATHERS_RED_COOLDOWN_MAX || !in_Air {
            let cool = if in_Air { 1.0 } else { FEATHERS_RED_COOLDOWN_GROUND_RATE };
            VarModule::add_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN, -cool);
    
            // If RedFeather cooldown ends...
            if VarModule::get_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN) <= 0.0 {
                VarModule::set_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN, 0.0);
                app::FighterUtil::flash_eye_info(fighter.module_accessor);
    
                // Show HUD again if already not visible
                if VarModule::get_int(fighter.battle_object, vars::buddy::instance::HUD_DISPLAY_TIME) == 0 {
                    buddy_meter_update_HUD(fighter, true);
                    VarModule::set_int(fighter.battle_object, vars::buddy::instance::HUD_DISPLAY_TIME, HUD_DISPLAY_TIME_MAX);
                }
            }
        }
    }
    // Refund cooldown if immediately caught ledge
    if fighter.is_status(*FIGHTER_STATUS_KIND_CLIFF_CATCH)
    && fighter.motion_frame() <= 3.0
    && VarModule::get_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN) > FEATHERS_RED_COOLDOWN_MAX - 5.0 {
        VarModule::set_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN, 1.0);
    }

	buddy_meter_display(fighter, in_Air);
}

// Control meter HUD display based on HUD_DISPLAY_TIME and current status
unsafe fn buddy_meter_display(fighter: &mut L2CFighterCommon, RedFeather: bool) {
	if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH,
        //*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL,
        //*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL,
		*FIGHTER_STATUS_KIND_REBIRTH
    ])
    && fighter.motion_frame() <= 3.0 {
		buddy_meter_update_HUD(fighter, RedFeather);
		VarModule::set_int(fighter.battle_object, vars::buddy::instance::HUD_DISPLAY_TIME, HUD_DISPLAY_TIME_MAX);
	}
	if VarModule::get_int(fighter.battle_object, vars::buddy::instance::HUD_DISPLAY_TIME) > 0 {
		VarModule::add_int(fighter.battle_object, vars::buddy::instance::HUD_DISPLAY_TIME, -1);
	}
	else {
		EffectModule::kill_kind(fighter.module_accessor, Hash40::new("buddy_special_s_count"), false, true);
	}
}

pub unsafe fn buddy_meter_update_HUD(fighter: &mut L2CFighterCommon, RedFeather: bool) {
	EffectModule::kill_kind(fighter.module_accessor, Hash40::new("buddy_special_s_count"), false, true);

    let FEATHERS_GOLD_COUNT = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
	//let hudZ = if (RedFeather) {25.0} else {0.0};
	let handle = EffectModule::req_follow(
		fighter.module_accessor,
		Hash40::new("buddy_special_s_count"),
		Hash40::new("top"),
		&Vector3f::new(0.0, 20.0, 0.0),
		&Vector3f::zero(),
		1.0,
		false,
		0,
		0,
		0,
		0,
		0,
		false,
		false,
	) as u32;

	let mut uv_offset_x = 0.0;
	let mut uv_offset_y = 0.2 * (FEATHERS_GOLD_COUNT as f32);
	if RedFeather {
		uv_offset_x = -1.5;
		uv_offset_y = if (VarModule::get_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN) == 0.0) { 0.2 } else { 0.0 };
		EffectModule::set_rgb(fighter.module_accessor, handle, 1.0, 0.3, 0.0);
	}
	EffectModule::set_custom_uv_offset(fighter.module_accessor, handle, &Vector2f::new(uv_offset_x, uv_offset_y), 0);
}

unsafe fn reset_vars(fighter: &mut L2CFighterCommon) {
    // Resets Red Feather cooldown in training mode after resetting
    if is_training_mode() {
        if fighter.is_status(*FIGHTER_STATUS_KIND_WAIT) || !smash::app::sv_information::is_ready_go() {
            if (VarModule::get_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN) > 0.0) {
                VarModule::set_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN, 0.0);
				VarModule::set_int(fighter.battle_object, vars::buddy::instance::HUD_DISPLAY_TIME, HUD_DISPLAY_TIME_MAX);
                buddy_meter_update_HUD(fighter, true);
            }
        }
    }
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH])
    && StatusModule::is_changing(fighter.module_accessor) {
        VarModule::off_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_ACTIVE);
        VarModule::off_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_DISABLE);
        
        VarModule::set_int(fighter.battle_object, vars::buddy::instance::HUD_DISPLAY_TIME, 60);
        VarModule::set_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_FRAME, 0);
        VarModule::set_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_BOUNCE_TYPE, 0);
        VarModule::set_int(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_EGGS_FIRED, 0);
        
        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        BAYONET_EGGS[entry] = 0;
    
        VarModule::set_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN, 0.0);
        VarModule::set_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_ANGLE, 0.0);
    }
}

// upB freefalls after one use per airtime
unsafe fn up_special_freefall(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor)
    && (fighter.is_situation(*SITUATION_KIND_GROUND)
        || fighter.is_situation(*SITUATION_KIND_CLIFF)
        || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP])) {
        VarModule::off_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_HI_ENABLE_FREEFALL);
    }
    if fighter.is_prev_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP) {
        if StatusModule::is_changing(fighter.module_accessor) {
            VarModule::on_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_HI_ENABLE_FREEFALL);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_LAG);
        }
    }
    if fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP) {
        if fighter.is_situation(*SITUATION_KIND_AIR)
        && VarModule::is_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_HI_ENABLE_FREEFALL) {
            if CancelModule::is_enable_cancel(fighter.module_accessor) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
                *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
            }
        }
    }
}

unsafe fn up_special_startup_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP) {
        // allows ledgegrab during upB startup
        if fighter.sub_transition_group_check_air_cliff().get_bool()
        && ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PAD) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PAD, ArticleOperationTarget(0));
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_AIR,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_AIR_TURN,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP_AERIAL,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_FALL,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_LW_SHOOT
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon) {
    blue_eggs_land_cancels(fighter);
    dair_bounce(fighter);
    dash_attack_jump_cancels(fighter);
    indicator_breegull_fatigue(fighter);
    beakbomb_update(fighter);
    breegull_bayonet(fighter);
    buddy_meter_controller(fighter);
    reset_vars(fighter);
    up_special_freefall(fighter);
    up_special_startup_ledgegrab(fighter);
    fastfall_specials(fighter);
}

pub unsafe extern "C" fn buddy_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    buddy_frame(fighter);
}

pub unsafe fn buddy_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, buddy_frame_wrapper);
}