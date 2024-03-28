// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 // Magnet Jump Cancel and Turnaround
unsafe fn psi_magnet_turnaround(fighter: &mut L2CFighterCommon) {
    if fighter.is_status (*FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD) {
        let facing = PostureModule::lr(fighter.module_accessor);
        let stick_x = fighter.stick_x();
        if stick_x * facing < 0.0 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
    }
}   

unsafe fn psi_magnet_jump_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[ 
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END]) {
        if fighter.status_frame() > 0 { // Allows for jump cancel on frame 8 in game (this is dictated by how long game_speciallw_start takes)
            if !fighter.is_in_hitlag() {
                fighter.check_jump_cancel(false, false);
            }
        }
    }
}

// Ness PK Fire drift
unsafe fn pk_fire_drift(boma: &mut BattleObjectModuleAccessor, stick_y: f32) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) {
        if boma.is_situation(*SITUATION_KIND_AIR) {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

unsafe fn magnet_stall_prevention(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END
        && situation_kind == *SITUATION_KIND_AIR {
        if VarModule::is_flag(boma.object(), vars::common::instance::STALL_PREVENTION) {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, true);
            // ^ just here to check if it ran at all
        }
        //VarModule::on_flag(boma.object(), vars::common::instance::STALL_PREVENTION);
    }

    if status_kind == *FIGHTER_STATUS_KIND_FALL
        && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END
        && situation_kind == *SITUATION_KIND_AIR {
        if  !VarModule::is_flag(boma.object(), vars::common::instance::STALL_PREVENTION) {
            // try to turn it on if the previous conditions were true?
            VarModule::on_flag(boma.object(), vars::common::instance::STALL_PREVENTION);
        }
    }
}

// Ness PK Thunder cancel
unsafe fn pk_thunder_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD {
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            if  !VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT) {
                VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT);
            }
            if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME) {
                VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL); // Disallow more up specials
            }
            StatusModule::change_status_request_from_script(boma, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, true);
        }
    }

    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END
        && situation_kind == *SITUATION_KIND_AIR {
        if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT) &&  !VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME) {
            VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }

    /*
    if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT) {
        println!("Up Special Interrupt flag active")
    }

    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END{
        println!("..... PKT1 COOLDOWN .....");
    }

    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END
        && (MotionModule::frame(boma) >= (MotionModule::end_frame(boma)-3.0))
        && situation_kind == *SITUATION_KIND_AIR {
        println!("PKT ending animation is over");
        if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT) &&  !VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME) {
            println!("PKT special airtime interrupt flag set");
            VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_INTERRUPT_AIRTIME);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
    */
}

// PK Thunder wall ride momentum fix
unsafe fn pk_thunder_wall_ride(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    let wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
    let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
    let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
    let touch_high = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_UP_SIDE as u32);
    let touch_low =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_DOWN_SIDE as u32);
    let touch_side =  GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32);

    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK{
        if touch_left || touch_right || touch_high || touch_low || touch_side {
            KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
    }

}

// Allow grabbing the ledge from behind while in upSpecialEnd
unsafe fn upspecialend_cliff(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END) {
        fighter.select_cliff_hangdata_from_name("special_air_hi_end");
    }
}

// Remove right arm growing during uair
unsafe fn uair_scaling(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR)
    && boma.is_motion(Hash40::new("attack_air_hi")) {
        ModelModule::set_joint_scale(boma, Hash40::new("clavicler"), &Vector3f::new(1.0, 1.0, 1.0));
        ModelModule::set_joint_scale(boma, Hash40::new("shoulderr"), &Vector3f::new(1.0, 1.0, 1.0));
        ModelModule::set_joint_scale(boma, Hash40::new("armr"), &Vector3f::new(1.0, 1.0, 1.0));
        ModelModule::set_joint_scale(boma, Hash40::new("haver"), &Vector3f::new(1.0, 1.0, 1.0));
        ModelModule::set_joint_scale(boma, Hash40::new("handr"), &Vector3f::new(1.0, 1.0, 1.0));
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD
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

unsafe fn pkt2_edgeslipoff(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END) 
    && fighter.is_situation(*SITUATION_KIND_AIR) 
    && fighter.is_prev_situation(*SITUATION_KIND_GROUND) {
        fighter.set_int(*FIGHTER_STATUS_KIND_FALL, *FIGHTER_NESS_STATUS_SPECIAL_HI_WORK_INT_NEXT_STATUS)
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    psi_magnet_turnaround(fighter);
    psi_magnet_jump_cancel(fighter);
    //pk_thunder_cancel(boma, id, status_kind, situation_kind);
    //magnet_stall_prevention(boma, id, status_kind, situation_kind);
    pk_thunder_wall_ride(boma, id, status_kind, situation_kind);
    //pk_fire_ff(boma, stick_y);
    upspecialend_cliff(fighter);
    pk_fire_drift(boma, stick_y);
    uair_scaling(boma);
    fastfall_specials(fighter);
    pkt2_edgeslipoff(fighter);
}

pub extern "C" fn ness_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		ness_frame(fighter)
    }
}

pub unsafe fn ness_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, ness_frame_wrapper);
}