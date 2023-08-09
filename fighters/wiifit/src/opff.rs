// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn header_cancel(boma: &mut BattleObjectModuleAccessor) {
    let status_kind_prev = StatusModule::prev_status_kind(boma, 0);
    if boma.is_status(*FIGHTER_STATUS_KIND_FALL_SPECIAL)
        && boma.is_prev_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP,
            *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_HEADING])
        && boma.is_situation(*SITUATION_KIND_AIR) {
        if !VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL) {
            VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL);
            ControlModule::reset_trigger(boma);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
        }
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_CANCEL) {
        if boma.is_situation(*SITUATION_KIND_AIR) {
            if WorkModule::get_int(boma, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
}

unsafe fn deep_breathing_respawn_cooldown(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) > 1 {
        VarModule::dec_int(boma.object(), vars::common::instance::GIMMICK_TIMER);
    }
    else if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) == 1 {
        //println!("cooldown over");
        VarModule::dec_int(boma.object(), vars::common::instance::GIMMICK_TIMER);
        VarModule::off_flag(boma.object(), vars::wiifit::instance::DEEP_BREATHING_COOLDOWN);
    }
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) {
        //println!("starting cooldown");
        VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 180);
        VarModule::off_flag(boma.object(), vars::wiifit::instance::DEEP_BREATHING_COOLDOWN);
    }
}

/// Starts ring effect for hitboxes
pub unsafe fn start_ring(fighter: &mut L2CFighterCommon, duration: f32, start_size: f32, end_size: f32, bone: Hash40, mut offset: Vector3f, mut color: Vector3f, mut color2: Vector3f, follow: bool) {
    VarModule::on_flag(fighter.object(), vars::wiifit::instance::IS_RING_VISIBLE);
    VarModule::set_float(fighter.object(), vars::wiifit::instance::RING_END_FRAME, duration);
    VarModule::set_float(fighter.object(), vars::wiifit::instance::RING_CURRENT_FRAME, 0.0);
    VarModule::set_float(fighter.object(), vars::wiifit::instance::RING_START_SIZE, start_size);
    VarModule::set_float(fighter.object(), vars::wiifit::instance::RING_END_SIZE, end_size);
    VarModule::set_int64(fighter.object(), vars::wiifit::instance::SHOW_RING_MOTION, MotionModule::motion_kind(fighter.module_accessor));
    
    // Make sure that no color alpha is zero
    color.x = if color.x == 0.0 { 0.1 } else { color.x };
    color.y = if color.y == 0.0 { 0.1 } else { color.y };
    color.z = if color.z == 0.0 { 0.1 } else { color.z };
    VarModule::set_vec3(fighter.object(), vars::wiifit::instance::RING_COLOR, color);
    color2.x = if color2.x == 0.0 { 0.1 } else { color2.x };
    color2.y = if color2.y == 0.0 { 0.1 } else { color2.y };
    color2.z = if color2.z == 0.0 { 0.1 } else { color2.z };
    VarModule::set_vec3(fighter.object(), vars::wiifit::instance::RING_SECOND_COLOR, color2);
   
    // Attach effect to bone if 'follow' arg is true, otherwise place effect at bone's current position with offset 
    //let mut calc_offset = ModelModule::joint_global_offset_from_top(fighter.module_accessor, bone, &mut offset);
    //offset.x *= PostureModule::lr(fighter.module_accessor);
    let handle = if follow {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("wiifit_fukushiki_ring"), bone, &offset, &Vector3f::zero(), start_size, false, 0, 0, 0, 0, 0, false, false)
    } else {
        EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("wiifit_fukushiki_ring"), bone, &offset, &Vector3f::zero(), start_size, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0)
    };
    let dark_handle = if follow {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("wiifit_fukushiki_ring"), bone, &offset, &Vector3f::zero(), start_size, false, 0, 0, 0, 0, 0, false, false)
    } else {
        EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("wiifit_fukushiki_ring"), bone, &offset, &Vector3f::zero(),  start_size, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0)
    };
    let light_handle = if follow {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("wiifit_fukushiki_ring"), bone, &offset, &Vector3f::zero(), start_size, false, 0, 0, 0, 0, 0, false, false)
    } else {
        EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("wiifit_fukushiki_ring"), bone, &offset, &Vector3f::zero(), start_size, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0)
    };

    // Set effect color and store handle
    EffectModule::set_rgb(fighter.module_accessor, handle as u32, color.x, color.y, color.z);
    EffectModule::set_rgb(fighter.module_accessor, dark_handle as u32, color.x, color.y, color.z);
    EffectModule::set_rgb(fighter.module_accessor, light_handle as u32, color.x, color.y, color.z);
    VarModule::set_int(fighter.object(), vars::wiifit::instance::RING_EFF_HANDLE, handle as i32);
    VarModule::set_int(fighter.object(), vars::wiifit::instance::RING_SECOND_EFF_HANDLE, dark_handle as i32);
    VarModule::set_int(fighter.object(), vars::wiifit::instance::RING_THIRD_EFF_HANDLE, light_handle as i32);
}

/// Updates ring color to second defined color
unsafe fn set_ring_color(fighter: &mut L2CFighterCommon, mut color: Vector3f) {
    let handle = VarModule::get_int(fighter.object(), vars::wiifit::instance::RING_EFF_HANDLE);
    let dark_handle = VarModule::get_int(fighter.object(), vars::wiifit::instance::RING_SECOND_EFF_HANDLE);
    let light_handle = VarModule::get_int(fighter.object(), vars::wiifit::instance::RING_THIRD_EFF_HANDLE);

    // Make sure no color alpha is 0.0
    color.x = if color.x == 0.0 { 0.1 } else { color.x };
    color.y = if color.y == 0.0 { 0.1 } else { color.y };
    color.z = if color.z == 0.0 { 0.1 } else { color.z };
    
    // Apply color to ring
    EffectModule::set_rgb(fighter.module_accessor, handle as u32, color.x, color.y, color.z);
    EffectModule::set_rgb(fighter.module_accessor, dark_handle as u32, color.x, color.y, color.z);
    EffectModule::set_rgb(fighter.module_accessor, light_handle as u32, color.x, color.y, color.z);
    
    // Store color vars
    VarModule::set_vec3(fighter.object(), vars::wiifit::instance::RING_COLOR, color);
    VarModule::set_vec3(fighter.object(), vars::wiifit::instance::RING_SECOND_COLOR, color);
}

/// Updates size and color of ring
pub unsafe fn update_ring(fighter: &mut L2CFighterCommon) {
    if !VarModule::is_flag(fighter.object(), vars::wiifit::instance::IS_RING_VISIBLE) { return; }
    let motion_kind = VarModule::get_int64(fighter.object(), vars::wiifit::instance::SHOW_RING_MOTION);
    if !fighter.is_motion(Hash40::new_raw(motion_kind)) {
        EFFECT_OFF_KIND(fighter, Hash40::new("wiifit_fukushiki_ring"), false, true);
        VarModule::off_flag(fighter.object(), vars::wiifit::instance::IS_RING_VISIBLE);
        return;
    }

    // Update ring size
    let current_frame = VarModule::get_float(fighter.object(), vars::wiifit::instance::RING_CURRENT_FRAME);
    let end_frame = VarModule::get_float(fighter.object(), vars::wiifit::instance::RING_END_FRAME);

    // Kill effect if beyond end frame
    if current_frame > end_frame {
        EFFECT_OFF_KIND(fighter, Hash40::new("wiifit_fukushiki_ring"), false, true);
        VarModule::off_flag(fighter.object(), vars::wiifit::instance::IS_RING_VISIBLE);
        return;
    }

    // Change to second color if attack hits
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        let color = VarModule::get_vec3(fighter.object(), vars::wiifit::instance::RING_SECOND_COLOR);
        set_ring_color(fighter, color);
    }
    
    let handle = VarModule::get_int(fighter.object(), vars::wiifit::instance::RING_EFF_HANDLE);
    let handle2 = VarModule::get_int(fighter.object(), vars::wiifit::instance::RING_SECOND_EFF_HANDLE);
    let handle3 = VarModule::get_int(fighter.object(), vars::wiifit::instance::RING_THIRD_EFF_HANDLE);
    let start_size = VarModule::get_float(fighter.object(), vars::wiifit::instance::RING_START_SIZE);
    let end_size = VarModule::get_float(fighter.object(), vars::wiifit::instance::RING_END_SIZE);
    let lerp = (current_frame as f32/end_frame as f32);

    // Linearly interpolate the size and color each frame
    use interpolation::Lerp;
    let lerp_size = Lerp::lerp(&start_size, &end_size, &lerp);
    let lerp_color = Lerp::lerp(&1.0, &0.25, &lerp);
    EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f{ x: lerp_size, y: lerp_size, z: lerp_size });
    EffectModule::set_scale(fighter.module_accessor, handle2 as u32, &Vector3f{ x: lerp_size - 0.05, y: lerp_size - 0.05, z: lerp_size - 0.05 });
    EffectModule::set_scale(fighter.module_accessor, handle3 as u32, &Vector3f{ x: lerp_size + 0.05, y: lerp_size + 0.05, z: lerp_size + 0.05 });
    EffectModule::set_alpha(fighter.module_accessor, handle as u32, lerp_color);
    EffectModule::set_alpha(fighter.module_accessor, handle2 as u32, lerp_color);
    EffectModule::set_alpha(fighter.module_accessor, handle3 as u32, lerp_color);
    VarModule::set_float(fighter.object(), vars::wiifit::instance::RING_CURRENT_FRAME, current_frame + 1.0);
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_HEADING,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_BREATH,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS,
        *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_FAILURE
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
    header_cancel(boma);
    nspecial_cancels(boma);
    deep_breathing_respawn_cooldown(boma);
    fastfall_specials(fighter);
    //update_ring(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_WIIFIT )]
pub fn wiifit_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		wiifit_frame(fighter)
    }
}

pub unsafe fn wiifit_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}