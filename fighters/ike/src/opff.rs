// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn aether_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, stick_x: f32, facing: f32) {
    if situation_kind != *SITUATION_KIND_AIR {
        return;
    }

    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2].contains(&status_kind) {
        if stick_x != 0.0 {
            let motion_vec = x_motion_vec(0.3, stick_x);
            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }
}

// Ike Quick Draw Jump, Wall Jump, and Attack Cancels
unsafe fn quickdraw_jump_attack_cancels(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32) {
    if status_kind != *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH {
        return;
    }
    
    // Wall Jump & ECB correction
    if situation_kind == *SITUATION_KIND_AIR {
        //GroundModule::set_rhombus_offset(boma, &Vector2f::new(0.0, 0.05));
        if  !VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_WALL_JUMP) {
            let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
            let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
            if touch_left || touch_right {
                if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) {
                    VarModule::on_flag(boma.object(), vars::common::instance::SPECIAL_WALL_JUMP);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                }
            }
        }
    }

    // Jump and Attack cancels
    let pad_flag = ControlModule::get_pad_flag(boma);
    
    if compare_mask(pad_flag, *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) || compare_mask(pad_flag, *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK, true);
    }
    if !VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL) {
        if situation_kind == *SITUATION_KIND_GROUND {
            VarModule::set_float(boma.object(), vars::common::instance::JUMP_SPEED_MAX_MUL, 1.346);  // 1.75 max jump speed out of Quick Draw
            boma.check_jump_cancel(true) || boma.check_attack_hi4_cancel(false);
        }
    }
}

unsafe fn quickdraw_instakill(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    if StatusModule::is_changing(boma) {
        return;
    }
    // Glow blue when attack is charged enough
    let cbm_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2}; // Brightness vector
    let cbm_vec2 = Vector4f{ /* Red */ x: 0.125, /* Green */ y: 0.4, /* Blue */ z: 1.0, /* Alpha */ w: 0.45}; // Diffuse vector

    if fighter.is_status(*FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD) && fighter.is_situation(*SITUATION_KIND_GROUND){
        if WorkModule::get_int(boma, *FIGHTER_IKE_STATUS_SPECIAL_S_WORK_INT_CHARGE_COUNT) > 160 {
            if !VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
                VarModule::on_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL);
                EFFECT_FOLLOW(fighter, Hash40::new("ike_volcano_hold"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1.0, false);
                ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_vec1, /* Diffuse */ &cbm_vec2, 0.21, 2.2, /*Fadein time*/ 30, /* Display Color */ true);
                //FLASH(fighter, 0.125, 0.4, 1, 0.45);
                //BURN_COLOR_FRAME(fighter, 30, 0.125, 0.4, 1, 0.45);
            }
        }
    }
    if fighter.is_status(*FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK) && fighter.is_situation(*SITUATION_KIND_GROUND){
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL) && MotionModule::frame(boma) >= 30.0 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
                if PostureModule::lr(boma) > 0.0{
                    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_APPEAL, false);
                    MotionModule::change_motion(boma, Hash40::new("appeal_lw_r"), -1.0, 1.0, false, 0.0, false, false);
                }
                else{
                    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_APPEAL, false);
                    MotionModule::change_motion(boma, Hash40::new("appeal_lw_l"), -1.0, 1.0, false, 0.0, false, false);
                }
            }
        }
    }
    if !fighter.is_status_one_of(&[*FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_STATUS_KIND_APPEAL]){
        if VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL){
            EFFECT_OFF_KIND(fighter, Hash40::new("ike_volcano_hold"), false, false);
            ColorBlendModule::cancel_main_color(boma, 0);
            VarModule::off_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL);
        }
    }
}


// Need to consolidate the following bone manipulation functions later

// boma: its a boma
// return_frame: frame to start interpolating back to regular angle
// straight_frame: frame the arm bones should be at the regular angle again
unsafe fn quickdraw_attack_arm_bend(boma: &mut BattleObjectModuleAccessor) {
    let frame = MotionModule::frame(boma);
    let return_frame = 0.0;
    let straight_frame = 0.1;
    let end_frame = MotionModule::end_frame(boma);
    let max_x_rotation = 0.0;
    let max_y_rotation = 0.0;
    let max_z_rotation = 75.0;
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        
    if boma.is_motion_one_of(&[Hash40::new("special_s_attack"), Hash40::new("special_air_s_attack")]) && !VarModule::is_flag(boma.object(), vars::ike::status::IS_QUICK_DRAW_INSTAKILL) && frame <= straight_frame {
        // linear interpolate back to normal
        let calc_x_rotate = max_x_rotation *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let x_rotation = calc_x_rotate.clamp(0.0, max_x_rotation);
        let calc_y_rotate = max_y_rotation *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let y_rotation = calc_y_rotate.clamp(0.0, max_y_rotation);
        let calc_z_rotate = max_z_rotation *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let z_rotation = calc_z_rotate.clamp(0.0, max_z_rotation);
        rotation = Vector3f{x: x_rotation, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(boma, Hash40::new("armr"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        ModelModule::set_joint_rotate(boma, Hash40::new("handr"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
}

// boma: its a boma
// start_frame: frame to start interpolating the bnoe rotation
// bend_frame: frame to interpolate to the intended angle amount until
// return_frame: frame to start interpolating back to regular angle
// straight_frame: frame the bones should be at the regular angle again
unsafe fn jab_lean(boma: &mut BattleObjectModuleAccessor) {
    let start_frame = 0.0;
    let bend_frame = 3.0;
    let return_frame = 10.0;
    let straight_frame = 21.0;
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let max_x_rotation_torso = 0.0;
    let max_y_rotation_torso = -25.0;
    let max_z_rotation_torso = 0.0;
    let mut rotation_torso = Vector3f{x: 0.0, y: 0.0, z: 0.0};

    let max_x_rotation_shoulder = 0.0;
    let max_y_rotation_shoulder = 25.0;
    let max_z_rotation_shoulder = 0.0;
    let mut rotation_shoulder = Vector3f{x: 0.0, y: 0.0, z: 0.0};

    let max_x_rotation_neck = 0.0;
    let max_y_rotation_neck = 0.0;
    let max_z_rotation_neck = -40.0;
    let mut rotation_neck = Vector3f{x: 0.0, y: 0.0, z: 0.0};

    let max_x_rotation_clavicle = 0.0;
    let max_y_rotation_clavicle = 0.0;
    let max_z_rotation_clavicle = -25.0;
    let mut rotation_clavicle = Vector3f{x: 0.0, y: 0.0, z: 0.0};

    if boma.is_motion(Hash40::new("attack_11")){
        if frame >= start_frame && frame < return_frame {
            // this has to be called every frame, or you snap back to the normal joint angle
            // interpolate to the respective bone bend angle
            // Torso bend
            let calc_x_rotate_torso = max_x_rotation_torso * (frame / (bend_frame - start_frame));
            let x_rotation_torso = calc_x_rotate_torso.clamp(0.0, max_x_rotation_torso);
            let calc_y_rotate_torso = max_y_rotation_torso * (frame / (bend_frame - start_frame));
            let y_rotation_torso = calc_y_rotate_torso.clamp(max_y_rotation_torso, 0.0);
            let calc_z_rotate_torso = max_z_rotation_torso * (frame / (bend_frame - start_frame));
            let z_rotation_torso = calc_z_rotate_torso.clamp(0.0, max_z_rotation_torso);
            rotation_torso = Vector3f{x: x_rotation_torso, y: y_rotation_torso, z: z_rotation_torso};
            ModelModule::set_joint_rotate(boma, Hash40::new("waist"), &rotation_torso, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            // Shoulder bend
            let calc_x_rotate_shoulder = max_x_rotation_shoulder * (frame / (bend_frame - start_frame));
            let x_rotation_shoulder = calc_x_rotate_shoulder.clamp(0.0, max_x_rotation_shoulder);
            let calc_y_rotate_shoulder = max_y_rotation_shoulder * (frame / (bend_frame - start_frame));
            let y_rotation_shoulder = calc_y_rotate_shoulder.clamp(0.0, max_y_rotation_shoulder);
            let calc_z_rotate_shoulder = max_z_rotation_shoulder * (frame / (bend_frame - start_frame));
            let z_rotation_shoulder = calc_z_rotate_shoulder.clamp(0.0, max_z_rotation_shoulder);
            rotation_shoulder = Vector3f{x: x_rotation_shoulder, y: y_rotation_shoulder, z: z_rotation_shoulder};
            ModelModule::set_joint_rotate(boma, Hash40::new("shoulderl"), &rotation_shoulder, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            // Neck bend
            let calc_x_rotate_neck = max_x_rotation_neck * (frame / (bend_frame - start_frame));
            let x_rotation_neck = calc_x_rotate_neck.clamp(0.0, max_x_rotation_neck);
            let calc_y_rotate_neck = max_y_rotation_neck * (frame / (bend_frame - start_frame));
            let y_rotation_neck = calc_y_rotate_neck.clamp(0.0, max_y_rotation_neck);
            let calc_z_rotate_neck = max_z_rotation_neck * (frame / (bend_frame - start_frame));
            let z_rotation_neck = calc_z_rotate_neck.clamp(max_z_rotation_neck, 0.0);
            rotation_neck = Vector3f{x: x_rotation_neck, y: y_rotation_neck, z: z_rotation_neck};
            ModelModule::set_joint_rotate(boma, Hash40::new("neck"), &rotation_neck, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            // Clavicle bend
            let calc_x_rotate_clavicle = max_x_rotation_clavicle * (frame / (bend_frame - start_frame));
            let x_rotation_clavicle = calc_x_rotate_clavicle.clamp(0.0, max_x_rotation_clavicle);
            let calc_y_rotate_clavicle = max_y_rotation_clavicle * (frame / (bend_frame - start_frame));
            let y_rotation_clavicle = calc_y_rotate_clavicle.clamp(0.0, max_y_rotation_clavicle);
            let calc_z_rotate_clavicle = max_z_rotation_clavicle * (frame / (bend_frame - start_frame));
            let z_rotation_clavicle = calc_z_rotate_clavicle.clamp(max_z_rotation_clavicle, 0.0);
            rotation_clavicle = Vector3f{x: x_rotation_clavicle, y: y_rotation_clavicle, z: z_rotation_clavicle};
            ModelModule::set_joint_rotate(boma, Hash40::new("claviclel"), &rotation_clavicle, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        } else if frame >= return_frame && frame < straight_frame {
            // linear interpolate back to normal
            // Torso bend
            let calc_x_rotate_torso = max_x_rotation_torso  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let x_rotation_torso  = calc_x_rotate_torso.clamp(0.0, max_x_rotation_torso);
            let calc_y_rotate_torso = max_y_rotation_torso  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let y_rotation_torso  = calc_y_rotate_torso.clamp(max_y_rotation_torso, 0.0);
            let calc_z_rotate_torso = max_z_rotation_torso  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let z_rotation_torso  = calc_z_rotate_torso.clamp(0.0, max_z_rotation_torso);
            rotation_torso  = Vector3f{x: x_rotation_torso, y: y_rotation_torso, z: z_rotation_torso };
            ModelModule::set_joint_rotate(boma, Hash40::new("waist"), &rotation_torso, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            // Shoulder bend
            let calc_x_rotate_shoulder = max_x_rotation_shoulder  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let x_rotation_shoulder  = calc_x_rotate_shoulder.clamp(0.0, max_x_rotation_shoulder);
            let calc_y_rotate_shoulder = max_y_rotation_shoulder  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let y_rotation_shoulder  = calc_y_rotate_shoulder.clamp(0.0, max_y_rotation_shoulder);
            let calc_z_rotate_shoulder = max_z_rotation_shoulder  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let z_rotation_shoulder  = calc_z_rotate_shoulder.clamp(0.0, max_z_rotation_shoulder);
            rotation_shoulder  = Vector3f{x: x_rotation_shoulder, y: y_rotation_shoulder, z: z_rotation_shoulder };
            ModelModule::set_joint_rotate(boma, Hash40::new("shoulderl"), &rotation_shoulder, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            // Neck bend
            let calc_x_rotate_neck = max_x_rotation_neck  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let x_rotation_neck  = calc_x_rotate_neck.clamp(0.0, max_x_rotation_neck);
            let calc_y_rotate_neck = max_y_rotation_neck  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let y_rotation_neck  = calc_y_rotate_neck.clamp(0.0, max_y_rotation_neck);
            let calc_z_rotate_neck = max_z_rotation_neck  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let z_rotation_neck  = calc_z_rotate_neck.clamp(max_z_rotation_neck, 0.0);
            rotation_neck  = Vector3f{x: x_rotation_neck, y: y_rotation_neck, z: z_rotation_neck };
            ModelModule::set_joint_rotate(boma, Hash40::new("neck"), &rotation_neck, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            // Clavicle bend
            let calc_x_rotate_clavicle = max_x_rotation_clavicle  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let x_rotation_clavicle  = calc_x_rotate_clavicle.clamp(0.0, max_x_rotation_clavicle);
            let calc_y_rotate_clavicle = max_y_rotation_clavicle  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let y_rotation_clavicle  = calc_y_rotate_clavicle.clamp(0.0, max_y_rotation_clavicle);
            let calc_z_rotate_clavicle = max_z_rotation_clavicle  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let z_rotation_clavicle  = calc_z_rotate_clavicle.clamp(max_z_rotation_clavicle, 0.0);
            rotation_clavicle  = Vector3f{x: x_rotation_clavicle, y: y_rotation_clavicle, z: z_rotation_clavicle };
            ModelModule::set_joint_rotate(boma, Hash40::new("claviclel"), &rotation_clavicle, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
    }
}

unsafe fn grab_lean(boma: &mut BattleObjectModuleAccessor) {
    let start_frame = 0.0;
    let bend_frame = 6.0;
    let return_frame = 13.0;
    let straight_frame = 36.0;
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let max_x_rotation_torso = 0.0;
    let max_y_rotation_torso = -25.0;
    let max_z_rotation_torso = 0.0;
    let mut rotation_torso = Vector3f{x: 0.0, y: 0.0, z: 0.0};

    let max_x_rotation_shoulder = 0.0;
    let max_y_rotation_shoulder = 30.0;
    let max_z_rotation_shoulder = 0.0;
    let mut rotation_shoulder = Vector3f{x: 0.0, y: 0.0, z: 0.0};

    let max_x_rotation_neck = 0.0;
    let max_y_rotation_neck = 0.0;
    let max_z_rotation_neck = -45.0;
    let mut rotation_neck = Vector3f{x: 0.0, y: 0.0, z: 0.0};

    let max_pos_shift = 1.0;
    let mut trans_offset = Vector3f{x: 0.0, y: 0.0, z: 0.0};

    if boma.is_motion(Hash40::new("catch")){
        if frame >= start_frame && frame < return_frame {
            // this has to be called every frame, or you snap back to the normal joint angle
            // interpolate to the respective bone bend angle
            // Torso bend
            let calc_x_rotate_torso = max_x_rotation_torso * (frame / (bend_frame - start_frame));
            let x_rotation_torso = calc_x_rotate_torso.clamp(0.0, max_x_rotation_torso);
            let calc_y_rotate_torso = max_y_rotation_torso * (frame / (bend_frame - start_frame));
            let y_rotation_torso = calc_y_rotate_torso.clamp(max_y_rotation_torso, 0.0);
            let calc_z_rotate_torso = max_z_rotation_torso * (frame / (bend_frame - start_frame));
            let z_rotation_torso = calc_z_rotate_torso.clamp(0.0, max_z_rotation_torso);
            rotation_torso = Vector3f{x: x_rotation_torso, y: y_rotation_torso, z: z_rotation_torso};
            ModelModule::set_joint_rotate(boma, Hash40::new("waist"), &rotation_torso, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            // Shoulder bend
            let calc_x_rotate_shoulder = max_x_rotation_shoulder * (frame / (bend_frame - start_frame));
            let x_rotation_shoulder = calc_x_rotate_shoulder.clamp(0.0, max_x_rotation_shoulder);
            let calc_y_rotate_shoulder = max_y_rotation_shoulder * (frame / (bend_frame - start_frame));
            let y_rotation_shoulder = calc_y_rotate_shoulder.clamp(0.0, max_y_rotation_shoulder);
            let calc_z_rotate_shoulder = max_z_rotation_shoulder * (frame / (bend_frame - start_frame));
            let z_rotation_shoulder = calc_z_rotate_shoulder.clamp(0.0, max_z_rotation_shoulder);
            rotation_shoulder = Vector3f{x: x_rotation_shoulder, y: y_rotation_shoulder, z: z_rotation_shoulder};
            ModelModule::set_joint_rotate(boma, Hash40::new("shoulderl"), &rotation_shoulder, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            // Neck bend
            let calc_x_rotate_neck = max_x_rotation_neck * (frame / (bend_frame - start_frame));
            let x_rotation_neck = calc_x_rotate_neck.clamp(0.0, max_x_rotation_neck);
            let calc_y_rotate_neck = max_y_rotation_neck * (frame / (bend_frame - start_frame));
            let y_rotation_neck = calc_y_rotate_neck.clamp(0.0, max_y_rotation_neck);
            let calc_z_rotate_neck = max_z_rotation_neck * (frame / (bend_frame - start_frame));
            let z_rotation_neck = calc_z_rotate_neck.clamp(max_z_rotation_neck, 0.0);
            rotation_neck = Vector3f{x: x_rotation_neck, y: y_rotation_neck, z: z_rotation_neck};
            ModelModule::set_joint_rotate(boma, Hash40::new("neck"), &rotation_neck, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(boma, Hash40::new("claviclel"), &rotation_neck, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

            // Model forward shift for stepping forwards
            let calc_trans_shift = max_pos_shift * (frame / (bend_frame - start_frame));
            let trans_shift = calc_trans_shift.clamp(0.0, max_pos_shift);
            trans_offset = Vector3f{x: 0.0, y: 0.0, z: trans_shift};
            ModelModule::set_joint_translate(boma, Hash40::new("trans"), &trans_offset, false, false);
        } else if frame >= return_frame && frame < straight_frame {
            // linear interpolate back to normal
            // Torso bend
            let calc_x_rotate_torso = max_x_rotation_torso  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let x_rotation_torso  = calc_x_rotate_torso.clamp(0.0, max_x_rotation_torso);
            let calc_y_rotate_torso = max_y_rotation_torso  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let y_rotation_torso  = calc_y_rotate_torso.clamp(max_y_rotation_torso, 0.0);
            let calc_z_rotate_torso = max_z_rotation_torso  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let z_rotation_torso  = calc_z_rotate_torso.clamp(0.0, max_z_rotation_torso);
            rotation_torso  = Vector3f{x: x_rotation_torso, y: y_rotation_torso, z: z_rotation_torso };
            ModelModule::set_joint_rotate(boma, Hash40::new("waist"), &rotation_torso, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            // Shoulder bend
            let calc_x_rotate_shoulder = max_x_rotation_shoulder  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let x_rotation_shoulder  = calc_x_rotate_shoulder.clamp(0.0, max_x_rotation_shoulder);
            let calc_y_rotate_shoulder = max_y_rotation_shoulder  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let y_rotation_shoulder  = calc_y_rotate_shoulder.clamp(0.0, max_y_rotation_shoulder);
            let calc_z_rotate_shoulder = max_z_rotation_shoulder  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let z_rotation_shoulder  = calc_z_rotate_shoulder.clamp(0.0, max_z_rotation_shoulder);
            rotation_shoulder  = Vector3f{x: x_rotation_shoulder, y: y_rotation_shoulder, z: z_rotation_shoulder };
            ModelModule::set_joint_rotate(boma, Hash40::new("shoulderl"), &rotation_shoulder, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            // Neck bend
            let calc_x_rotate_neck = max_x_rotation_neck  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let x_rotation_neck  = calc_x_rotate_neck.clamp(0.0, max_x_rotation_neck);
            let calc_y_rotate_neck = max_y_rotation_neck  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let y_rotation_neck  = calc_y_rotate_neck.clamp(0.0, max_y_rotation_neck);
            let calc_z_rotate_neck = max_z_rotation_neck  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let z_rotation_neck  = calc_z_rotate_neck.clamp(max_z_rotation_neck, 0.0);
            rotation_neck  = Vector3f{x: x_rotation_neck, y: y_rotation_neck, z: z_rotation_neck };
            ModelModule::set_joint_rotate(boma, Hash40::new("neck"), &rotation_neck, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            ModelModule::set_joint_rotate(boma, Hash40::new("claviclel"), &rotation_neck, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

            // Shift model back again
            let calc_trans_shift = max_pos_shift * (1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let trans_shift = calc_trans_shift.clamp(0.0, max_pos_shift);
            trans_offset = Vector3f{x: 0.0, y: 0.0, z: trans_shift};
            ModelModule::set_joint_translate(boma, Hash40::new("trans"), &trans_offset, false, false);
        }
    }
}

unsafe fn fair_wrist_bend(boma: &mut BattleObjectModuleAccessor) {
    //let start_frame = 0.0;
    //let bend_frame = 0.3;
    //let return_frame = 100.0;
    //let straight_frame = 105.0;
    let start_frame = 7.0;
    let bend_frame = 13.0;
    let return_frame = 14.0;
    let straight_frame = 26.0;
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let max_x_rotation_wrist = 0.0;
    let max_y_rotation_wrist = 35.0;
    let max_z_rotation_wrist = 0.0;
    let mut rotation_wrist = Vector3f{x: 0.0, y: 0.0, z: 0.0};

    if boma.is_motion(Hash40::new("attack_air_f")){
        if frame >= start_frame && frame < return_frame {
            // this has to be called every frame, or you snap back to the normal joint angle
            // interpolate to the respective bone bend angle
            // wrist bend
            let calc_x_rotate_wrist = max_x_rotation_wrist * (frame / (bend_frame - start_frame));
            let x_rotation_wrist = calc_x_rotate_wrist.clamp(0.0, max_x_rotation_wrist);
            let calc_y_rotate_wrist = max_y_rotation_wrist * (frame / (bend_frame - start_frame));
            let y_rotation_wrist = calc_y_rotate_wrist.clamp(0.0, max_y_rotation_wrist);
            let calc_z_rotate_wrist = max_z_rotation_wrist * (frame / (bend_frame - start_frame));
            let z_rotation_wrist = calc_z_rotate_wrist.clamp(max_z_rotation_wrist, 0.0);
            rotation_wrist = Vector3f{x: x_rotation_wrist, y: y_rotation_wrist, z: z_rotation_wrist};
            ModelModule::set_joint_rotate(boma, Hash40::new("handr"), &rotation_wrist, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        } else if frame >= return_frame && frame < straight_frame {
            // linear interpolate back to normal
            // wrist bend
            let calc_x_rotate_wrist = max_x_rotation_wrist  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let x_rotation_wrist  = calc_x_rotate_wrist.clamp(0.0, max_x_rotation_wrist);
            let calc_y_rotate_wrist = max_y_rotation_wrist  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let y_rotation_wrist  = calc_y_rotate_wrist.clamp(0.0, max_y_rotation_wrist);
            let calc_z_rotate_wrist = max_z_rotation_wrist  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let z_rotation_wrist  = calc_z_rotate_wrist.clamp(max_z_rotation_wrist, 0.0);
            rotation_wrist  = Vector3f{x: x_rotation_wrist, y: y_rotation_wrist, z: z_rotation_wrist };
            ModelModule::set_joint_rotate(boma, Hash40::new("handr"), &rotation_wrist, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
    }
}

unsafe fn quickdraw_attack_whiff_freefall(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && !StatusModule::is_changing(fighter.module_accessor)
    && CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
        let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
        *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
    }
}


pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    aether_drift(boma, status_kind, situation_kind, stick_x, facing);
    quickdraw_jump_attack_cancels(boma, id, status_kind, situation_kind, cat[0], stick_x, facing);
    quickdraw_instakill(fighter, boma);
    quickdraw_attack_arm_bend(boma);
    jab_lean(boma);
    grab_lean(boma);
    fair_wrist_bend(boma);
    quickdraw_attack_whiff_freefall(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_IKE )]
pub fn ike_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		ike_frame(fighter)
    }
}

pub unsafe fn ike_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}