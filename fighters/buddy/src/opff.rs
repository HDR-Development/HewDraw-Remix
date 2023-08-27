use super::*;
use globals::*;

const HUD_DISPLAY_TIME_MAX: i32 = 90;
const FEATHERS_RED_COOLDOWN_GROUND_RATE: f32 = 1.25;
const FEATHERS_RED_COOLDOWN_MAX: f32 = 450.0;
const BEAKBOMB_END_FRAME: i32 = 25; //Dash timer is shared between ground and air in vl.prc

static mut BAYONET_EGGS:[i32;8] = [0; 8]; //I have no idea why varmod doesn't work with this, so this will have to do
 
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn blue_eggs_land_cancels(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.is_prev_situation(*SITUATION_KIND_AIR)
    {
        // Current FAF in motion list is 50, frame is 0 indexed so subtract a frame
        let special_n_fire_cancel_frame_ground = 49.0;
        // 11F of landing lag plus one extra frame to subtract from the FAF to actually get that amount of lag
        let landing_lag = 12.0;
        if MotionModule::frame(fighter.module_accessor) < (special_n_fire_cancel_frame_ground - landing_lag) {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, special_n_fire_cancel_frame_ground - landing_lag, true, true, false);
        }
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        //fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING, false);
    }
}

// Banjo Grenade Airdodge Cancel
unsafe fn grenade_ac(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    if fighter.is_status_one_of(&[*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_LW_SHOOT, *FIGHTER_STATUS_KIND_SPECIAL_LW])
    && fighter.motion_frame() > 16.0
    {
        fighter.check_airdodge_cancel();
    }
}

// Banjo Dair bounce
unsafe fn dair_bounce(fighter: &mut L2CFighterCommon){
    if fighter.is_motion(Hash40::new("attack_air_lw"))
    && fighter.motion_frame() < 46.0
    {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 45.0, true, true, false);
        }
    }
}

// Banjo Wondering Fail on command
unsafe fn wonderwing_fail(fighter: &mut L2CFighterCommon){
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    if ((fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) && fighter.motion_frame() > 17.0)
    || (fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END) && fighter.motion_frame() < 4.0))
    && fighter.is_button_on(Buttons::Guard)
    {
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, true);
    }
}

unsafe fn dash_attack_jump_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
    && situation_kind == *SITUATION_KIND_AIR
    && MotionModule::frame(boma) >= 27.0 {
        fighter.check_jump_cancel(false, false);
    }
}

unsafe fn indicator_breegull_fatigue(fighter: &mut L2CFighterCommon){
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
	let eggs_shot = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
    let eggs_Weakest = WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_n"),hash40("bakyun_power_down_2_num"));
    let eggs_Weak = WorkModule::get_param_int(fighter.module_accessor,hash40("param_special_n"),hash40("bakyun_power_down_1_num"));
	if (eggs_shot >= eggs_Weak
	&& !fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_END))
	{
		let sweatRate = if (eggs_shot<eggs_Weakest) {25.0} else {15.0};
		let sweatSize = if (eggs_shot<eggs_Weakest) {0.625} else {0.9};
		let modulo = fighter.motion_frame() % sweatRate;
		if (modulo<1.0)
		{
			EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_sweat"), Hash40::new("top"), 0, 8.5, 7.5, 0, 0, 0, sweatSize, true);
		}
	}
}

//Banjo can airdodge cancel a bonk only after frame 15 if he did not hit a shield
unsafe fn bonk_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){ 
    if StatusModule::is_changing(boma) {
        return;
    }
    let side_special_wall = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL);
    if (!side_special_wall) {return;}
    let has_hit_shield = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
    if (has_hit_shield) {return;}
    if (VarModule::get_int(boma.object(), vars::buddy::instance::BEAKBOMB_BOUNCE)>=1) {return;}

    let cancel_frame = 16.0;
    let can_cancel = fighter.motion_frame() >= cancel_frame;
    if (can_cancel)
    {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}


unsafe fn beakbomb_checkForCancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    if StatusModule::is_changing(boma) {
        return;
    }
    let side_special = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH);
    if !side_special {return;}

    let has_hit_shield = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
    if (has_hit_shield) {return;}

    let in_Air = fighter.is_situation(*SITUATION_KIND_AIR);
    if (!in_Air) {return;}

    let is_guarding = fighter.is_button_on(Buttons::Guard);
    let cancel_frame = 11.0;
    let can_cancel = fighter.motion_frame() >= cancel_frame;
    if (is_guarding && can_cancel)
    {
         fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
    }

}

unsafe fn beakbomb_control(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    //If past the end frame, transition into end
    if (VarModule::get_int(boma.object(), vars::buddy::instance::BEAKBOMB_FRAME) >= BEAKBOMB_END_FRAME)
    {
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
        return;
    }

    //Do not update flight during hitstop
    if boma.is_in_hitlag() {return;}

    //Movement
    let motion_factor = 0.425;
    let motion_offset = -0.125;
    let motion_vec = Vector3f{x: 0.0, y: motion_offset+(VarModule::get_float(boma.object(), vars::buddy::instance::BEAKBOMB_ANGLE)*motion_factor), z: 0.0};
    KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);

}
unsafe fn beakbomb_update(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,status: i32){
    let sideSpecial = [
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL
    ].contains(&StatusModule::status_kind(fighter.module_accessor));
    let side_special_dash = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH);
    let side_special_wall = fighter.is_status(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL);
    let in_Air = fighter.is_situation(*SITUATION_KIND_AIR);

    //While in Beakbomb / Wonderwing
    if (side_special_dash)
    {
        if (VarModule::is_flag(boma.object(), vars::buddy::instance::BEAKBOMB_ACTIVE))
        {
            beakbomb_control(fighter,boma);
            //beakbomb_checkForHit(fighter,boma);
            beakbomb_checkForGround(fighter,boma);
            beakbomb_checkForCancel(fighter,boma);

            GroundModule::set_attach_ground(fighter.module_accessor, false);
            VarModule::add_int(boma.object(), vars::buddy::instance::BEAKBOMB_FRAME,1);
        }
        else if (!in_Air)
        {
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
    }
    else if (side_special_wall)
    {
        beakbomb_wall(fighter,boma);
    }
    //If out of SideSpecial, then set BEAKBOMB_ACTIVE to false
    else if (!sideSpecial && VarModule::is_flag(boma.object(), vars::buddy::instance::BEAKBOMB_ACTIVE))
    {
        VarModule::off_flag(boma.object(), vars::buddy::instance::BEAKBOMB_ACTIVE);
    }

}

//Check to see if Banjo hit a shield during beakbomb.
unsafe fn beakbomb_checkForHit(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    if StatusModule::is_changing(boma) {
        return;
    }
    let has_hit_shield = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD);
    if (!has_hit_shield) {return;}
    
    if (fighter.motion_frame() > 0.0) //If motion frame is 0, game crashes
    {
        let start_frame = 7.0;
        let weak_frame = 21.0;
        VarModule::set_int(boma.object(), vars::buddy::instance::BEAKBOMB_BOUNCE,
            if (fighter.motion_frame() >= weak_frame) {1} else {2}
        );
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, false);
        //MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, start_frame, true, true, false);
    }
}

//Recoil for bouncing off walls/shields
unsafe fn beakbomb_wall(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let start_frame = 7.0;
    if fighter.is_motion(Hash40::new("special_air_s_wall"))
    && fighter.motion_frame() < start_frame
    && fighter.motion_frame() > 0.0 {
        let x_bounce = match VarModule::get_int(boma.object(), vars::buddy::instance::BEAKBOMB_BOUNCE){
            0=> -1.0,
            2=> -2.0,
            _=> -1.5
        };
        let y_bounce = if (VarModule::get_int(boma.object(), vars::buddy::instance::BEAKBOMB_BOUNCE)<1) {0.5} else {1.0};
        WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        SET_SPEED_EX(fighter, x_bounce, y_bounce, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        VarModule::off_flag(boma.object(), vars::buddy::instance::BEAKBOMB_ACTIVE);
    }
}

//Check if landed on the ground
unsafe fn beakbomb_checkForGround(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let is_grounded = fighter.is_situation(*SITUATION_KIND_GROUND);
    let fail_safeFrames = 5;
    let fail_cutoff = 25;
    let can_damage = fail_safeFrames < VarModule::get_int(boma.object(), vars::buddy::instance::BEAKBOMB_FRAME);
    let can_fail = VarModule::get_int(boma.object(), vars::buddy::instance::BEAKBOMB_FRAME) < fail_cutoff;
    if !(is_grounded) {return;}

    if (can_fail)
    {
        //Add damage
        if (can_damage)
            {DamageModule::add_damage(fighter.module_accessor, 10.0,0);}

        KineticModule::clear_speed_all(fighter.module_accessor);
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, true);
        PLAY_SE(fighter, Hash40::new("vc_buddy_missfoot01"));
    }
    else
    {
        fighter.change_status_req(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, true);
    }

}

unsafe fn breegull_bayonet(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,status: i32){
    if StatusModule::is_changing(boma) {
        return;
    }
    let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if (VarModule::is_flag(boma.object(), vars::buddy::instance::BAYONET_ACTIVE))
    {
        if (status == *FIGHTER_STATUS_KIND_ATTACK_S3 )
        {
            let transition_frame = 21.0;
            let can_cancel = fighter.motion_frame() >= transition_frame;
            if (!can_cancel) {return;}

            fighter.change_to_custom_status(statuses::buddy::BUDDY_BAYONET_END, false, false);

            let currentEggs=
            //VarModule::get_int(boma.object(), vars::buddy::instance::BAYONET_EGGS);
            BAYONET_EGGS[entry];
            WorkModule::set_int(fighter.module_accessor,
                currentEggs,
                *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT
            );

            VarModule::off_flag(boma.object(), vars::buddy::instance::BAYONET_ACTIVE);
        }
    }
    else if [
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_F,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_B,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_TURN,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_LANDING
    ].contains(&status)
    {
        let currentEggs = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
        //VarModule::set_int(boma.object(), vars::buddy::instance::BAYONET_EGGS,currentEggs);
        BAYONET_EGGS[entry] = currentEggs;
    }
}

unsafe fn buddy_meter_update_HUD(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,RedFeather: bool) {
	EffectModule::kill_kind(boma, Hash40::new("buddy_special_s_count"), false, true);

    let FEATHERS_GOLD_COUNT = WorkModule::get_int(boma,  *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
	//let hudZ = if (RedFeather) {25.0} else {0.0};
	let position = Vector3f::new(0.0,20.0,0.0);
	let handle = EffectModule::req_follow(
		boma,
		Hash40::new("buddy_special_s_count"),
		Hash40::new("top"),
		&position,
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
	let mut uv_offset_y = 0.2*(FEATHERS_GOLD_COUNT as f32);
	if (RedFeather)
	{
		uv_offset_x = -1.5;
		uv_offset_y = if (VarModule::get_float(boma.object(), vars::buddy::instance::FEATHERS_RED_COOLDOWN) == 0.0) {0.2} else {0.0};
		EffectModule::set_rgb(boma,handle, 1.0, 0.3, 0.0);
	}
	EffectModule::set_custom_uv_offset(boma, handle, &Vector2f::new(uv_offset_x,uv_offset_y), 0);
}

//Control meter HUD display based on HUD_DISPLAY_TIME and current status
unsafe fn buddy_meter_display(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,RedFeather: bool){
    if StatusModule::is_changing(boma) {
        return;
    }
    let status = StatusModule::status_kind(fighter.module_accessor);
    let side_special = [
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL,
        //*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL,
		*FIGHTER_STATUS_KIND_REBIRTH
    ].contains(&status);
	if (side_special && fighter.motion_frame()<=3.0)
	{
		buddy_meter_update_HUD(fighter,boma,RedFeather);
		VarModule::set_int(boma.object(), vars::buddy::instance::HUD_DISPLAY_TIME,HUD_DISPLAY_TIME_MAX);
	}
	if (VarModule::get_int(boma.object(), vars::buddy::instance::HUD_DISPLAY_TIME)>0)
	{
		VarModule::add_int(boma.object(), vars::buddy::instance::HUD_DISPLAY_TIME,-1);
	}
	else
	{
		EffectModule::kill_kind(boma, Hash40::new("buddy_special_s_count"), false, true);
	}
}
//This causes vectoring?
unsafe fn buddy_meter_controller(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor,status: i32){
    if StatusModule::is_changing(boma) {
        return;
    }
    if (status == *FIGHTER_STATUS_KIND_WIN) { 
		EffectModule::kill_kind(boma, Hash40::new("buddy_special_s_count"), false, true);
        return;
    }
    let in_Air = fighter.is_situation(*SITUATION_KIND_AIR);
	if (VarModule::get_float(boma.object(), vars::buddy::instance::FEATHERS_RED_COOLDOWN)>0.0)
	{
		let cool = if (in_Air) {1.0} else {FEATHERS_RED_COOLDOWN_GROUND_RATE};
		VarModule::add_float(boma.object(), vars::buddy::instance::FEATHERS_RED_COOLDOWN,-cool);

        //If RedFeather cooldown ends...
		if (VarModule::get_float(boma.object(), vars::buddy::instance::FEATHERS_RED_COOLDOWN)<=0.0)
		{
			VarModule::set_float(boma.object(), vars::buddy::instance::FEATHERS_RED_COOLDOWN,0.0);
			app::FighterUtil::flash_eye_info(fighter.module_accessor);

            //Show HUD again if already not visible
			if (VarModule::get_int(boma.object(), vars::buddy::instance::HUD_DISPLAY_TIME)==0)
			{
				buddy_meter_update_HUD(fighter,boma,true);
				VarModule::set_int(boma.object(), vars::buddy::instance::HUD_DISPLAY_TIME,HUD_DISPLAY_TIME_MAX);
			}
		}
    }
    //Refund cooldown if immediately caught ledge
    if (fighter.motion_frame() <= 3.0 && in_Air)
    {
        if (status == *FIGHTER_STATUS_KIND_CLIFF_CATCH
        && VarModule::get_float(boma.object(), vars::buddy::instance::FEATHERS_RED_COOLDOWN) > FEATHERS_RED_COOLDOWN_MAX-5.0)
        {
            VarModule::set_float(boma.object(), vars::buddy::instance::FEATHERS_RED_COOLDOWN,1.0);
        }
        else if (status == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH)
        {
            VarModule::set_float(boma.object(), vars::buddy::instance::FEATHERS_RED_COOLDOWN,FEATHERS_RED_COOLDOWN_MAX);
        }
	}

	buddy_meter_display(fighter,boma,in_Air);
}

//Called on rebirth and entry
unsafe fn on_rebirth(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor)
{
    let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    VarModule::off_flag(boma.object(), vars::buddy::instance::BEAKBOMB_ACTIVE);
    VarModule::off_flag(boma.object(), vars::buddy::instance::BAYONET_ACTIVE);
    
    VarModule::set_int(boma.object(), vars::buddy::instance::HUD_DISPLAY_TIME,60);
    VarModule::set_int(boma.object(), vars::buddy::instance::BEAKBOMB_FRAME,0);
    VarModule::set_int(boma.object(), vars::buddy::instance::BEAKBOMB_BOUNCE,0);
    VarModule::set_int(boma.object(), vars::buddy::instance::BAYONET_EGGS,0);
    BAYONET_EGGS[entry]=0;

    VarModule::set_float(boma.object(), vars::buddy::instance::FEATHERS_RED_COOLDOWN,0.0);
    VarModule::set_float(boma.object(), vars::buddy::instance::BEAKBOMB_ANGLE,0.0);
}

//Resets Red Feather cooldown in training mode after resetting
unsafe fn training_reset(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor)
{
    if is_training_mode() {
        if fighter.is_status(*FIGHTER_STATUS_KIND_WAIT) || !smash::app::sv_information::is_ready_go() {
            if (VarModule::get_float(boma.object(), vars::buddy::instance::FEATHERS_RED_COOLDOWN)>0.0)
            {
                VarModule::set_float(boma.object(), vars::buddy::instance::FEATHERS_RED_COOLDOWN,0.0);
				VarModule::set_int(boma.object(), vars::buddy::instance::HUD_DISPLAY_TIME,HUD_DISPLAY_TIME_MAX);
                buddy_meter_update_HUD(fighter,boma,true);
            }
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
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    dair_bounce(fighter);
    //wonderwing_fail(fighter);
    blue_eggs_land_cancels(fighter);
    grenade_ac(fighter);
    dash_attack_jump_cancels(fighter, boma, status_kind, situation_kind);
    indicator_breegull_fatigue(fighter);

    beakbomb_update(fighter,boma,status_kind);
    breegull_bayonet(fighter,boma,status_kind);
    buddy_meter_controller(fighter,boma,status_kind);
    training_reset(fighter,boma);
    fastfall_specials(fighter);

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) {
        on_rebirth(fighter,boma);
    }
}


#[fighter_reset]
fn buddy_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;    
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
        if fighter.kind() == *FIGHTER_KIND_BUDDY {
            on_rebirth(fighter,boma);
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_BUDDY)]
pub unsafe fn buddy_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    buddy_frame(fighter);
}

pub unsafe fn buddy_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
