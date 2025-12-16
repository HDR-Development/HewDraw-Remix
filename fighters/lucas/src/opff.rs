// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn psi_magnet_jc(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END]) {
        if boma.status_frame() > 0 {
            if !AttackModule::is_attack(boma, 0, false) { //jc if hitbox clear
                boma.check_jump_cancel(false, false);
            }
        }
    }
}

// Offense Up charge Handler
pub unsafe fn offense_charge(fighter: &mut smash::lua2cpp::L2CFighterCommon)  {
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
unsafe fn reset_flags(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_ENTRY]) || !sv_information::is_ready_go() {
        let charge_time = ParamModule::get_int(fighter.object(), ParamType::Agent, "attack_up_charge_time");
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, charge_time);
        VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
        VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
        VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
    }
}

unsafe fn offense_effect_handler(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
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

unsafe fn smash_s_angle_handler(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_START]) {
        let frame = fighter.motion_frame();
        // Up Tilted Side Smash
        if VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_UP) {
            joint_rotator(fighter, frame, Hash40::new("waist"), Vector3f{x: 0.0, y:-10.0, z:0.0}, 13.0, 15.0, 16.0, 25.0);
            joint_rotator(fighter, frame, Hash40::new("bust"), Vector3f{x: 0.0, y:-10.0, z:0.0}, 13.0, 15.0, 16.0, 25.0);
            joint_rotator(fighter, frame, Hash40::new("handl"), Vector3f{x: 0.0, y:-20.0, z:0.0}, 11.0, 15.0, 16.0, 25.0);
            joint_rotator(fighter, frame, Hash40::new("handr"), Vector3f{x: 0.0, y:-20.0, z:0.0}, 11.0, 15.0, 16.0, 25.0);
        }
        // Down Tilted Side Smash
        else if VarModule::is_flag(fighter.object(), vars::lucas::instance::ATTACK_S4_ANGLE_DOWN) {
            joint_rotator(fighter, frame, Hash40::new("waist"), Vector3f{x: 0.0, y:10.0, z:0.0}, 13.0, 15.0, 16.0, 25.0);
            joint_rotator(fighter, frame, Hash40::new("bust"), Vector3f{x: 0.0, y:10.0, z:0.0}, 13.0, 15.0, 16.0, 25.0);
            joint_rotator(fighter, frame, Hash40::new("handl"), Vector3f{x: 0.0, y:20.0, z:0.0}, 11.0, 15.0, 16.0, 25.0);
            joint_rotator(fighter, frame, Hash40::new("handr"), Vector3f{x: 0.0, y:20.0, z:0.0}, 11.0, 15.0, 16.0, 25.0);
        }
    }
}

unsafe fn dashgrab_position_fix(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_CATCH_DASH) {
        let frame = fighter.motion_frame();
        joint_rotator(fighter, frame, Hash40::new("joint1"), Vector3f{x:-10.0, y:0.0, z:0.0}, 5.0, 12.0, 30.0, 45.0);
    }
}

unsafe fn upspecialend(fighter: &mut L2CFighterCommon) {
    // I DO NOT KNOW WHY, BUT ACMD IS FUCKING NOT WORKING FOR THIS STATUS >:(
    if fighter.is_status(*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END) {
        if StatusModule::is_changing(fighter.module_accessor) {
            fighter.select_cliff_hangdata_from_name("special_air_hi_end");
            KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        else if fighter.status_frame() == 10 {
            KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE,
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HOLD,
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

unsafe fn pkt2_edgeslipoff(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END) 
    && fighter.is_situation(*SITUATION_KIND_AIR) 
    && fighter.is_prev_situation(*SITUATION_KIND_GROUND) {
        fighter.set_int(*FIGHTER_STATUS_KIND_FALL, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_NEXT_STATUS)
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    smash_s_angle_handler(fighter);
    dashgrab_position_fix(fighter);
    psi_magnet_jc(boma);
    offense_charge(fighter);
    offense_effect_handler(fighter);
    reset_flags(fighter);
    upspecialend(fighter);
    fastfall_specials(fighter);
    pkt2_edgeslipoff(fighter);
}

pub extern "C" fn lucas_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		lucas_frame(fighter)
    }
}

pub unsafe fn lucas_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, lucas_frame_wrapper);
}