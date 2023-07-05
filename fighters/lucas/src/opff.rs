// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


unsafe fn psi_magnet_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    if [*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        if boma.status_frame() > 0 {
            if !boma.is_in_hitlag() {
                boma.check_jump_cancel(false);
            }
        }
    }
}

// Lucas PK Thunder cancel
// unsafe fn pk_thunder_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
//    if status_kind == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD {
//         if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
//             if  !VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT) {
//                 VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT);
//             }
//             if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME) {
//                 VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL); // Disallow more up specials
//             }
//             StatusModule::change_status_request_from_script(boma, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, true);
//         }
//     }

//     if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL
//         && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END
//         && situation_kind == *SITUATION_KIND_AIR {
//         if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT) &&  !VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME) {
//             VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME);
//             StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
//         }
//     }
// }

// Lucas DJC and momentum tracker
unsafe fn djc_momentum_helper(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
        VarModule::set_float(boma.object(), vars::common::instance::DOUBLE_JUMP_FRAME, frame);
    }
    /*
    if VarModule::get_float(boma.object(), vars::common::instance::DOUBLE_JUMP_FRAME) == 1.0 {
        VarModule::set_float(boma.object(), vars::common::DOUBLE_JUMP_TIMER, 1.0);
    }
    if VarModule::get_float(boma.object(), vars::common::DOUBLE_JUMP_TIMER) > 0.0 && (status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR) {
        VarModule::add_float(boma.object(), vars::common::DOUBLE_JUMP_TIMER, 1.0);
    }
    if VarModule::is_flag(boma.object(), vars::common::DOUBLE_JUMP_STOP) && VarModule::get_float(boma.object(), vars::common::DOUBLE_JUMP_TIMER) == 0.0 {
        VarModule::off_flag(boma.object(), vars::common::DOUBLE_JUMP_STOP);
        println!("DJ stop flag reset");
    }
    if VarModule::is_flag(boma.object(), vars::common::DOUBLE_JUMP_STOP) {
        VarModule::set_float(boma.object(), vars::common::DOUBLE_JUMP_TIMER, 0.0);
        println!("Ended!");
    }
    if VarModule::get_float(boma.object(), vars::common::DOUBLE_JUMP_TIMER) >= 15.0 {
        VarModule::on_flag(boma.object(), vars::common::DOUBLE_JUMP_STOP);
        println!("Ending DJC motion blending");
    }
    //println!("Lucas DJ timer: Frame {}", VarModule::get_float(boma.object(), vars::common::DOUBLE_JUMP_TIMER));
    */
}

// PK Thunder wall ride momentum "fix"
// this move can go die in a hole
unsafe fn pk_thunder_wall_ride_shorten(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    let wall_ride = Vector3f{x: 1.0, y: -1.0, z: 1.0};
    let wall_leave = Vector3f{x: -1.0, y: 1.0, z: 1.0};
    let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT as u32);
    let touch_right_side = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
    let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT as u32);
    let touch_left_side = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
    let touch_high = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_UP as u32);
    let touch_high_side = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_UP_SIDE as u32);
    let touch_low =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_DOWN as u32);
    let touch_low_side =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_DOWN_SIDE as u32);
    let touch_side =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32);
    let touch_all =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_ID_MAX_LRUD as u32);
    let touch_right =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_ID_RIGHT as u32);
    let touch_left =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_ID_LEFT as u32);
    let touch_none =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_ID_NONE as u32);

    let touch_grass = GroundModule::is_touch(boma, *GROUND_TOUCH_ID_MAX as u32);

    if boma.is_status_one_of(&[*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK/*, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END*/]){
        let shorten_end_frame = MotionModule::end_frame(boma) - 5.0;
        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) && MotionModule::frame(boma) < shorten_end_frame {
            MotionModule::set_frame_sync_anim_cmd(boma, shorten_end_frame, true, true, false);
            //boma.change_status_req(*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, false);
        }
        // Get the current x momentum for the status
        let x_momentum = KineticModule::get_sum_speed_x(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        // Get the current y momentum for the status
        let y_momentum = KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        // Get the deceleration from Lucas's vl.prc
        let special_hi_attack_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("attack_brake"));
        let special_hi_attack_init_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("attack_speed"));

        // Get the initial x momentum if it hasn't been grabbed yet for the status
        if VarModule::get_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_X_INIT_MOMENTUM) > 10000.0{
            VarModule::set_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_X_INIT_MOMENTUM, x_momentum);
        }
        // Get the initial y momentum if it hasn't been grabbed yet for the status
        if VarModule::get_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_Y_INIT_MOMENTUM) > 10000.0{
            VarModule::set_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_Y_INIT_MOMENTUM, y_momentum);
        }

        // If touching a wall...
        if touch_left || touch_left_side || touch_right || touch_right_side || touch_high || touch_high_side || touch_low || touch_low_side || touch_side {
        //if touch_right || touch_left {
        //if touch_grass {
            //DamageModule::add_damage(boma, 1.0, 0);
            
            // If the vertical momentum you have while touching the wall is opposite your initial up b momentum, multiply the vertical momentum by -1.0 to get back in the right direction
            if y_momentum * VarModule::get_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_Y_INIT_MOMENTUM) < 0.0{
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
            // If you haven't touched a wall yet, set the wall touch flag to true and store the frame you touched the wall
            if !VarModule::is_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_TOUCH_WALL){
                VarModule::on_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_TOUCH_WALL);
                VarModule::set_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_WALL_TOUCH_FRAME, MotionModule::frame(boma));
            }
            
        }
        // If not touching a wall...
        else {
            // If the horizontal momentum you have after touching the wall is opposite your initial up b momentum, multiply the horizontal momentum by -1.0 to get back in the right direction
            
            if VarModule::is_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_TOUCH_WALL)
               && !VarModule::is_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_FLIPPED_MOMENTUM_AFTER_WALLTOUCH)
               && x_momentum * VarModule::get_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_X_INIT_MOMENTUM) < 0.0{
                KineticModule::mul_speed(boma, &wall_leave, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                VarModule::on_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_FLIPPED_MOMENTUM_AFTER_WALLTOUCH);
            }
            
            
            // Get the x momentum while not touching the wall so we can grab it later for if we touch then leave the wall
            VarModule::set_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_X_MOMENTUM, x_momentum);
            // If you've touched a wall and are no longer touching a wall...
            if VarModule::is_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_TOUCH_WALL) && !VarModule::is_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_LEAVE_WALL){
                // No longer touching a wall, set the wall touch flag to off
                VarModule::off_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_TOUCH_WALL);
                // Notify us that we've left the wall, set the wall leave flag to on
                VarModule::on_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_LEAVE_WALL);
                // Grab the frame we left the wall so we can calculate our deceleration
                VarModule::set_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_WALL_LEAVE_FRAME, MotionModule::frame(boma));  
            }
            
            /*
            if KineticModule::get_sum_speed_x(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP).abs() < 0.01 {
                VarModule::off_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_SET_WALL_LEAVE_MOMENTUM);
            }
            */
            // If we've left the wall after touching it and haven't set our momentum yet...
            /*
            if VarModule::is_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_LEAVE_WALL) && !VarModule::is_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_SET_WALL_LEAVE_MOMENTUM){
                //DamageModule::add_damage(boma, 1.0, 0);
                let new_y_momentum = KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                let wall_touch_frame = VarModule::get_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_WALL_TOUCH_FRAME);
                let wall_leave_frame = VarModule::get_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_WALL_LEAVE_FRAME);
                let x_speed_before_walltouch = VarModule::get_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_X_MOMENTUM);
                // Calculate the travel speed...
                // (speed right before walltouch) - (up b deceleration) * ((wall touch frame) - (wall leave frame))
                //let wall_leave_x_travel_speed = x_speed_before_walltouch + (-1.0)*(special_hi_attack_brake)*(wall_touch_frame - wall_leave_frame);

                let wall_leave_x_travel_speed = special_hi_attack_init_speed + (-1.0)*(special_hi_attack_brake)*(MotionModule::frame(boma) + 2.0);
                fighter.clear_lua_stack();
                //lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, wall_leave_x_travel_speed, new_y_momentum);
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, wall_leave_x_travel_speed, new_y_momentum);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                VarModule::on_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_SET_WALL_LEAVE_MOMENTUM);
            }
            */
            
        }
    }
    else{
        // Reset all the related flags and floats if not in the up b statuses
        VarModule::set_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_X_INIT_MOMENTUM, 10001.0);
        VarModule::set_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_Y_INIT_MOMENTUM, 10001.0);
        VarModule::set_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_WALL_TOUCH_FRAME, 10001.0);
        VarModule::set_float(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_WALL_LEAVE_FRAME, 10001.0);
        VarModule::off_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_TOUCH_WALL);
        VarModule::off_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_LEAVE_WALL);
        VarModule::off_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_FLIPPED_MOMENTUM_AFTER_WALLTOUCH);
        VarModule::off_flag(boma.object(), vars::lucas::status::SPECIAL_HI_ATTACK_IS_SET_WALL_LEAVE_MOMENTUM);
    }

}

// Lucas PK Fire Fast Fall
unsafe fn pk_fire_ff(boma: &mut BattleObjectModuleAccessor, stick_y: f32) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) {
        if boma.is_situation(*SITUATION_KIND_AIR) {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Offense Up charge Handler
pub unsafe fn offense_charge(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, situation_kind: i32)  {
    if(VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE)) {
        if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, 
            *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE]
        ) {
            //println!("In swing! Status of release: {} Reflective: {}", VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF));
            if(AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            }
        }
        else if !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, 
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_START, 
            *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_END]) && VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF
        ) {
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
        }
    } 
}

// We run this check on pre-handle to ensure the flag is not improperly edited. //
unsafe fn reset_flags(fighter: &mut smash::lua2cpp::L2CFighterCommon, status_kind: i32, situation_kind: i32) {
    if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind)  || !sv_information::is_ready_go() {
        let charge_time = ParamModule::get_int(fighter.object(), ParamType::Agent, "attack_up_charge_time");
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, charge_time);
        VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
        VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
        VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
    }
}

unsafe fn offense_effct_handler(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) && !VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT) 
    && (VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1) == -1 || VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2) == -1) {
        // The case is that Lucas is in Offense Up, has cleared past `pkfr_hold` effects, yet he does not have his hand effects. //
        let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("lucas_pkfr_hold"), Hash40::new("handl"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1, handle as i32);
        let handle2 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("lucas_pkfr_hold"), Hash40::new("handr"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2, handle2 as i32);
        let handle3 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_status_defense_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3, handle3 as i32);
    }
    else if !VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) 
    && (VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1) != -1 || VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2) != -1) {
        // The case is that Lucas is no longer in Offence Up, and his hand effects NEED TO BE CLEARED. //
        let handle = VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1) as u32;
        EffectModule::kill(fighter.module_accessor, handle, false, false);
        let handle2 = VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2) as u32;
        EffectModule::kill(fighter.module_accessor, handle2, false, false);
        let handle3 = VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3) as u32;
        EffectModule::kill(fighter.module_accessor, handle3, false, false);
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1, -1);
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2, -1);
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3, -1);
    }
}

// fighter: its a fighter
// frame: its frames
// joint: the hash of the joint you are interpolating the bend to
// rotation_anount: the amount of rotation you want to preform in Vector3
// start_frame: frame to start interpolating the joint rotation
// bend_frame: frame to interpolate to the intended angle amount until
// return_frame: frame to start interpolating back to regular angle
// straight_frame: frame the waist should be at the regular angle again
unsafe fn joint_rotator(fighter: &mut L2CFighterCommon, frame: f32, joint: Hash40, rotation_amount: Vector3f, start_frame: f32, bend_frame: f32, return_frame: f32, straight_frame: f32) {
    let lua_state = fighter.lua_state_agent;
    let end_frame = MotionModule::end_frame(fighter.boma());
    let max_rotation = rotation_amount;
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    //println!("Frame is: {}", frame);
    if frame >= start_frame && frame < return_frame {
        // this has to be called every frame, or you snap back to the normal joint angle
        // interpolate to the respective waist bend angle
        let calc_x_rotate = max_rotation.x * (frame / (bend_frame - start_frame));
        let calc_y_rotate = max_rotation.y * (frame / (bend_frame - start_frame));
        let calc_z_rotate = max_rotation.z * (frame / (bend_frame - start_frame));
        let mut x_rotation = 0.0;
        let mut y_rotation = 0.0;
        let mut z_rotation = 0.0;
        if max_rotation.x < 0.0 {
            x_rotation = calc_x_rotate.clamp(max_rotation.x, 0.0);
        }
        else {
            x_rotation = calc_x_rotate.clamp(0.0, max_rotation.x);
        }
        if max_rotation.y < 0.0 {
            y_rotation = calc_y_rotate.clamp(max_rotation.y, 0.0);
        }
        else {
            y_rotation = calc_y_rotate.clamp(0.0, max_rotation.y);
        }
        if max_rotation.z < 0.0 { 
            z_rotation = calc_z_rotate.clamp(max_rotation.z, 0.0);
        }
        else{
            z_rotation = calc_z_rotate.clamp(0.0, max_rotation.z);
        }
        //println!("Rotation: {}, {}, {}", x_rotation, y_rotation, z_rotation);
        rotation = Vector3f{x: x_rotation, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(fighter.boma(), joint, &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    } else if frame >= return_frame && frame < straight_frame {
        // linear interpolate back to normal
        let calc_x_rotate = max_rotation.x *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let calc_y_rotate = max_rotation.y *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let calc_z_rotate = max_rotation.z *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let mut x_rotation = 0.0;
        let mut y_rotation = 0.0;
        let mut z_rotation = 0.0;
        if max_rotation.x < 0.0 {
            x_rotation = calc_x_rotate.clamp(max_rotation.x, 0.0);
        }
        else {
            x_rotation = calc_x_rotate.clamp(0.0, max_rotation.x);
        }
        if max_rotation.y < 0.0 {
            y_rotation = calc_y_rotate.clamp(max_rotation.y, 0.0);
        }
        else {
            y_rotation = calc_y_rotate.clamp(0.0, max_rotation.y);
        }
        if max_rotation.z < 0.0 { 
            z_rotation = calc_z_rotate.clamp(max_rotation.z, 0.0);
        }
        else{
            z_rotation = calc_z_rotate.clamp(0.0, max_rotation.z);
        }
        //println!("Rotation: {}, {}, {}", x_rotation, y_rotation, z_rotation);
        rotation = Vector3f{x: x_rotation, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(fighter.boma(), joint, &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    }
}

unsafe fn smash_s_angle_handler(fighter: &mut L2CFighterCommon, frame: f32) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_START]) {
        // Up Tilted Side Smash
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP) {
            joint_rotator(fighter, frame, Hash40::new("waist"), Vector3f{x: 0.0, y:-30.0, z:0.0}, 11.0, 15.0, 17.0, 25.0);
            joint_rotator(fighter, frame, Hash40::new("bust"), Vector3f{x: 0.0, y:-20.0, z:0.0}, 11.0, 15.0, 17.0, 25.0);
        }
        // Down Tilted Side Smash
        else if VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN) {
            joint_rotator(fighter, frame, Hash40::new("waist"), Vector3f{x: 0.0, y:10.0, z:0.0}, 11.0, 15.0, 17.0, 25.0);
            joint_rotator(fighter, frame, Hash40::new("bust"), Vector3f{x: 0.0, y:10.0, z:0.0}, 11.0, 15.0, 17.0, 25.0);
            joint_rotator(fighter, frame, Hash40::new("handl"), Vector3f{x: 0.0, y:20.0, z:0.0}, 11.0, 15.0, 17.0, 25.0);
            joint_rotator(fighter, frame, Hash40::new("handr"), Vector3f{x: 0.0, y:20.0, z:0.0}, 11.0, 15.0, 17.0, 25.0);
        }
    }
}

unsafe fn dashgrab_position_fix(fighter: &mut L2CFighterCommon, frame: f32) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_CATCH_DASH) {
        joint_rotator(fighter, frame, Hash40::new("joint1"), Vector3f{x:-10.0, y:0.0, z:0.0}, 5.0, 12.0, 30.0, 45.0);
    }
}

unsafe fn upspecialend(fighter: &mut L2CFighterCommon) {
    // I DO NOT KNOW WHY, BUT ACMD IS FUCKING NOT WORKING FOR THIS STATUS >:(
    if fighter.is_status(*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END) {
        fighter.select_cliff_hangdata_from_name("special_air_hi_end");
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    smash_s_angle_handler(fighter, frame);
    dashgrab_position_fix(fighter, frame);
    psi_magnet_jc(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    //pk_thunder_cancel(boma, id, status_kind, situation_kind);
    //pk_thunder_wall_ride_shorten(fighter, boma, id, status_kind, situation_kind);
    //djc_momentum_helper(boma, id, status_kind, frame);
    pk_fire_ff(boma, stick_y);
    offense_charge(fighter, boma, situation_kind);
    offense_effct_handler(fighter);
    reset_flags(fighter, status_kind, situation_kind);
    upspecialend(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_LUCAS)]
pub fn lucas_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		lucas_frame(fighter)
    }
}

pub unsafe fn lucas_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame_callback]
pub fn pkthunder_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.kind() != WEAPON_KIND_LUCAS_PK_THUNDER {
            return
        }
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    }
}