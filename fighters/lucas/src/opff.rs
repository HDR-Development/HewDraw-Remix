// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn psi_magnet_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    if [*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
        if frame > 1.0 {
            if boma.is_input_jump() && !boma.is_in_hitlag() {
                if situation_kind == *SITUATION_KIND_AIR {
                    if boma.get_num_used_jumps() < boma.get_jump_count_max() {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                    }
                } else if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

// Lucas PK Thunder cancel
unsafe fn pk_thunder_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
   if status_kind == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD {
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if  !VarModule::is_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT) {
                VarModule::on_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT);
            }
            if VarModule::is_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT_AIRTIME) {
                VarModule::on_flag(boma.object(), vars::common::UP_SPECIAL_CANCEL); // Disallow more up specials
            }
            StatusModule::change_status_request_from_script(boma, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, true);
        }
    }

    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END
        && situation_kind == *SITUATION_KIND_AIR {
        if VarModule::is_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT) &&  !VarModule::is_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT_AIRTIME) {
            VarModule::on_flag(boma.object(), vars::common::UP_SPECIAL_INTERRUPT_AIRTIME);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

// Lucas DJC and momentum tracker
unsafe fn djc_momentum_helper(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
        VarModule::set_float(boma.object(), vars::common::DOUBLE_JUMP_FRAME, frame);
    }
    /*
    if VarModule::get_float(boma.object(), vars::common::DOUBLE_JUMP_FRAME) == 1.0 {
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
    if status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR {
        VarModule::off_flag(boma.object(), vars::common::DOUBLE_JUMP_CANCELED);
    }
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

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    psi_magnet_jc(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    pk_thunder_cancel(boma, id, status_kind, situation_kind);
    //pk_thunder_wall_ride_shorten(fighter, boma, id, status_kind, situation_kind);
    //djc_momentum_helper(boma, id, status_kind, frame);
    pk_fire_ff(boma, stick_y);
}

#[utils::macros::opff(FIGHTER_KIND_LUCAS )]
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