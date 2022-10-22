// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn header_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    let status_kind_prev = StatusModule::prev_status_kind(boma, 0);
    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && [*FIGHTER_STATUS_KIND_SPECIAL_S,*FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_HEADING].contains(&status_kind_prev)
        && situation_kind == *SITUATION_KIND_AIR {
        if  !VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL) {
            VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                //ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

unsafe fn set_zen_mode(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_SUCCESS {
        WorkModule::on_flag(boma, vars::wiifit::instance::IS_ZEN_MODE);
    }
    else if status_kind == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_LW_FAILURE {
        WorkModule::off_flag(boma, vars::wiifit::instance::IS_ZEN_MODE);
    }
}

/// Starts ring effect for hitboxes
pub unsafe fn start_ring(fighter: &mut L2CFighterCommon, duration: f32, start_size: f32, end_size: f32, mut offset: Vector3f, bone: Hash40, mut color: Vector3f, mut color2: Vector3f) {
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
   
    // Attach effect to bone
    ModelModule::joint_global_offset_from_top(fighter.module_accessor, bone, &mut offset);
    offset.x *= PostureModule::lr(fighter.module_accessor);
    let light_handle = EffectModule::req_on_joint(
        fighter.module_accessor,
        Hash40::new("wiifit_fukushiki_ring"),
        Hash40::new("top"),
        &offset,
        &Vector3f::zero(),
        start_size,
        &Vector3f::zero(),
        &Vector3f::zero(),
        false,
        0,
        0,
        0
    );
    let dark_handle = EffectModule::req_on_joint(
        fighter.module_accessor,
        Hash40::new("wiifit_fukushiki_ring"),
        Hash40::new("top"),
        &offset,
        &Vector3f::zero(),
        start_size,
        &Vector3f::zero(),
        &Vector3f::zero(),
        false,
        0,
        0,
        0
    );
    let handle = EffectModule::req_on_joint(
        fighter.module_accessor,
        Hash40::new("wiifit_fukushiki_ring"),
        Hash40::new("top"),
        &offset,
        &Vector3f::zero(),
        start_size,
        &Vector3f::zero(),
        &Vector3f::zero(),
        false,
        0,
        0,
        0
    );

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
    let current_frame = fighter.motion_frame() - VarModule::get_float(fighter.object(), vars::wiifit::instance::RING_START_FRAME);
    let end_frame = VarModule::get_float(fighter.object(), vars::wiifit::instance::RING_END_FRAME);
    let handle = VarModule::get_int(fighter.object(), vars::wiifit::instance::RING_EFF_HANDLE);
    let handle2 = VarModule::get_int(fighter.object(), vars::wiifit::instance::RING_SECOND_EFF_HANDLE);
    let handle3 = VarModule::get_int(fighter.object(), vars::wiifit::instance::RING_THIRD_EFF_HANDLE);
    let start_size = VarModule::get_float(fighter.object(), vars::wiifit::instance::RING_START_SIZE);
    let end_size = VarModule::get_float(fighter.object(), vars::wiifit::instance::RING_END_SIZE);
    let progress = (current_frame as f32/end_frame as f32);

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

    // Linearly interpolate the size and color each frame
    use interpolation::Lerp;
    let lerp_size = Lerp::lerp(&start_size, &end_size, &progress);
    let lerp_color = Lerp::lerp(&1.0, &0.25, &progress);
    EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f{ x: lerp_size, y: lerp_size, z: lerp_size });
    EffectModule::set_scale(fighter.module_accessor, handle2 as u32, &Vector3f{ x: lerp_size - 0.05, y: lerp_size - 0.05, z: lerp_size - 0.05 });
    EffectModule::set_scale(fighter.module_accessor, handle3 as u32, &Vector3f{ x: lerp_size + 0.05, y: lerp_size + 0.05, z: lerp_size + 0.05 });
    EffectModule::set_alpha(fighter.module_accessor, handle as u32, lerp_color);
    EffectModule::set_alpha(fighter.module_accessor, handle2 as u32, lerp_color);
    EffectModule::set_alpha(fighter.module_accessor, handle3 as u32, lerp_color);
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    nspecial_cancels(boma, status_kind, situation_kind);
    header_cancel(boma, id, status_kind, situation_kind);
    set_zen_mode(boma, status_kind);
    update_ring(utils::util::get_fighter_common_from_accessor(boma));
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
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}