use super::*;
use globals::*;

// This function is used to calculate the following:
//      param_2: Left ECB point's horizontal offset from Top bone (negative number)
//      param_3: Bottom ECB point's vertical offset from Top bone (positive number, 0.0 in vanilla)
//      param_4: Right ECB point's horizontal offset from Top bone (positive number)
//      param_5: Top ECB point's vertical offset from Top bone (positive number)

// All of your character's map_coll_data bones, found in vl.prc, are stored in param_1's Hash40 pointer

// Not sure what param_6 is, but when 0, it skips calculations for your ECB's bottom point and just sets it to 0.0, which "locks" your ECB's bottom point to your Top bone
// when 1, it calculates your bottom ECB point normally, like the other 3 points
// Vanilla passes 0 by default, so we have to forcibly pass in 1
#[skyline::hook(offset = 0x45f420)]
unsafe fn ground_module_ecb_point_calc_hook(ground_module: u64, param_1: *mut *mut Hash40, param_2: *mut f32, param_3: *mut f32, param_4: *mut f32, param_5: *mut f32, param_6: u32) {
    // The original function calls model_module_joint_global_position_with_offset_hook
    let boma = *((ground_module + 0x20) as *mut *mut BattleObjectModuleAccessor);
    if (*boma).is_fighter() { VarModule::on_flag((*boma).object(), vars::common::instance::IS_GETTING_POSITION_FOR_ECB); }
    call_original!(ground_module, param_1, param_2, param_3, param_4, param_5, 1);
    if (*boma).is_fighter() {
        VarModule::off_flag((*boma).object(), vars::common::instance::IS_GETTING_POSITION_FOR_ECB);
        VarModule::set_float((*boma).object(), vars::common::instance::ECB_BOTTOM_Y_OFFSET, *param_3);
        let ecb_center_y_offset = ((*param_5 - *param_3) / 2.0) + *param_3;
        VarModule::set_float((*boma).object(), vars::common::instance::ECB_CENTER_Y_OFFSET, ecb_center_y_offset);
    }
    if !(*boma).is_fighter()
    || VarModule::is_flag((*boma).object(), vars::common::status::DISABLE_ECB_SHIFT)
    || (*boma).is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEMO,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_THROWN])
    || !(*boma).is_situation(*SITUATION_KIND_AIR)
    || WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < ParamModule::get_int((*boma).object(), ParamType::Common, "ecb_shift_air_trans_frame") {
        // This check passes after 9 frames of airtime, if not in a grabbed/thrown state
        *param_3 = 0.0;
    }

    // Prevents rising platwarps during aerials and tumble
    if !StopModule::is_stop(boma) {
        if (*boma).is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY])
        && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0
        {
            // Forces character to ignore platforms, overrides ability to land
            GroundModule::set_passable_check(boma, true);
        }
        else if (*boma).is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR)
        && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) <= 0.0
        {
            GroundModule::set_passable_check(boma, false);
        }
    }
}


// This function calculates the coordinates of the passed bone relative to the Top bone (PostureModule::pos)
// It stores these x/y/z coordinates in param_3's Vector3f

// ground_module_ecb_point_calc_hook will pass each bone from your character's map_coll_data list in vl.prc, one by one, into this func
// If param_6 in ground_module_ecb_point_calc_hook is 1, it will then pass the Trans bone once all of the map_coll_data bones have been processed
// The game will use your Trans bone's distance from your Top bone to determine where to place your bottom ECB point, which will pretty much always be {0, 0, 0}
// This is why your ECB bottom point is always "locked" to your Top bone

// By returning once the Trans bone is passed into this func, we can ignore it and thus use your map_coll_data bones to calculate your bottom ECB point, like the other 3 points
#[skyline::hook(offset = 0x48fc40)]
unsafe fn model_module_joint_global_position_with_offset_hook(model_module: u64, bone: Hash40, param_3: *mut Vector3f, param_4: *mut Vector3f, param_5: bool) {
    let boma = *(model_module as *mut *mut BattleObjectModuleAccessor).add(1);
    if (*boma).is_fighter()
    && VarModule::is_flag((*boma).object(), vars::common::instance::IS_GETTING_POSITION_FOR_ECB)
    && bone == Hash40::new("trans")
    {
        return;
    }
    call_original!(model_module, bone, param_3, param_4, param_5);
}

// Unused for now
#[skyline::hook(offset = 0x523a60)]
unsafe fn groundcollision__processgroundcollisioninfo(groundcollisioninfo: *mut f32, groundcollision: *mut u64) {
    call_original!(groundcollisioninfo, groundcollision)
}

// Performs ground correct/sets situation kind
#[skyline::hook(offset = 0x53fe30)]
unsafe fn groundcollision__processgroundcollisioninfo_check_landing(groundcollisioninfo: *mut f32, groundcollision: u64) {
    let groundcollisionline = *((groundcollision + 0x320) as *mut u64) as *mut GroundCollisionLine;
    let groundcollisionline_next = *(groundcollisionline as *mut *mut GroundCollisionLine);
    let vertex_1_y = *(((groundcollisionline_next as u64) + 0x24) as *mut f32);
    let vertex_2_y = *(((groundcollisionline_next as u64) + 0x34) as *mut f32);
    let touch_pos_y = (vertex_1_y + vertex_2_y) / 2.0;
    
    let flags = *(groundcollisioninfo.add(0x5d8 / 4) as *mut u32);
    let is_fighter = flags >> 0x1b & 1 == 0;
    let is_item = flags >> 0xa & 1 == 0;
    let situation_kind = *(groundcollisioninfo.add(0x5a0 / 4) as *mut i32);  // 1 = ground, 2 = air, 3 = cliff...
    let prev_pos_y = *groundcollisioninfo.add(0x4c4 / 4);
    let pos_y = *groundcollisioninfo.add(0x634 / 4);
    let prev_ecb_offset_y = *groundcollisioninfo.add(0x424 / 4);
    let ecb_offset_y = *groundcollisioninfo.add(0x3d4 / 4);

    if !is_fighter
    && !is_item
    && situation_kind == 2
    && (prev_ecb_offset_y == 0.0 && ecb_offset_y != 0.0)  // this only passes on the frame a projectile spawns
    && (prev_pos_y + prev_ecb_offset_y) >= (pos_y + ecb_offset_y)  // checks if the projectile is descending
    && (pos_y + ecb_offset_y) <= touch_pos_y  // checks if the projectile's ECB bottom position is underneath the nearest surface
    {
        *groundcollisioninfo.add(0x420 / 4) = *groundcollisioninfo.add(0x3d0 / 4);  // prev_ecb_offset_x = ecb_offset_x
        *groundcollisioninfo.add(0x424 / 4) = *groundcollisioninfo.add(0x3d4 / 4);  // prev_ecb_offset_y = ecb_offset_y
        *((groundcollision + 0x39f) as *mut bool) = true;  // sets a flag in GroundCollision which tells subsequent functions to ignore ground collision
    } else {
        *((groundcollision + 0x39f) as *mut bool) = false;
    };

    call_original!(groundcollisioninfo, groundcollision);

    let prev_touch_pos_y = *groundcollisioninfo.add(0x1A4 / 4);
    let touch_pos_y = *groundcollisioninfo.add(0xB4 / 4);
    let ecb_offset_y = *groundcollisioninfo.add(0x3d4 / 4);

    if is_fighter
    && prev_touch_pos_y == 0.0
    && touch_pos_y != 0.0
    && ecb_offset_y != 0.0
    && lua_bind::BattleObjectSlow::is_adjust(utils::singletons::BattleObjectSlow())
    {
        // When landing, sets your position to the coordinates of the surface you are landing on
        *groundcollisioninfo.add(0x634 / 4) = touch_pos_y;
        // Reset ECB offset to 0.0
        *groundcollisioninfo.add(0x3d4 / 4) = 0.0;
    }
}
// Sets GroundCollisionLine
#[skyline::hook(offset = 0x52d900)]
unsafe fn groundcollision__processgroundcollisioninfo_check_landing_sub(groundcollision: u64, arg2: *mut u64, prev_ecb_bottom_pos: *mut smash::phx::Vector2f, ecb_bottom_translation: *mut smash::phx::Vector2f, arg5: u64, arg6: u64, arg7: *mut u64) -> *mut GroundCollisionLine {
    if *((groundcollision + 0x39f) as *mut bool) {
        // Ignore ground collision
        return 0 as *mut GroundCollisionLine;
    }
    call_original!(groundcollision, arg2, prev_ecb_bottom_pos, ecb_bottom_translation, arg5, arg6, arg7)
}

pub fn install() {
    unsafe {
        // Removes 0.3 unit leniency above ECB bottom when deciding whether to land
        // which reduces frequency of platform cancels
        skyline::patching::Patch::in_text(0x540dd8).data(0x529ae148);
        skyline::patching::Patch::in_text(0x540ddc).data(0x72a78468);
    }
    skyline::install_hooks!(
        groundcollision__processgroundcollisioninfo_check_landing,
        groundcollision__processgroundcollisioninfo_check_landing_sub,
        ground_module_ecb_point_calc_hook,
        model_module_joint_global_position_with_offset_hook
    );
}