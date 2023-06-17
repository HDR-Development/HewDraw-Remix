use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn aerial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F]) {
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && VarModule::get_int(fighter.battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID) < 6 { 
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::DIRECT_HIT);
        }//enable hit flag on direct hit
        if VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::DIRECT_HIT) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        && !fighter.is_motion_one_of(&[Hash40::new("attack_air_n_hold"), Hash40::new("attack_air_f"), Hash40::new("attack_air_f_hold"), Hash40::new("attack_air_f2_hold"), Hash40::new("attack_air_f3_hold"), Hash40::new("attack_air_hi_hold"), Hash40::new("attack_air_lw_hold")])    
        && !fighter.is_in_hitlag() {
            let mut new_status = 0;
            let mut is_input_cancel = false;
            if fighter.is_input_jump()
            && fighter.get_num_used_jumps() < fighter.get_jump_count_max()
            {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_JUMP_AERIAL;
            }
            if fighter.is_cat_flag(Cat1::SpecialN) {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_N;
            }
            if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_CANCEL_THIS_AIRTIME) <= 2 {
                if fighter.is_cat_flag(Cat1::SpecialS) {
                    if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) <= 2 {
                        is_input_cancel = true;
                        new_status = *FIGHTER_STATUS_KIND_SPECIAL_S;
                    }
                }
                if fighter.is_cat_flag(Cat1::SpecialHi) {
                    if !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) {
                        is_input_cancel = true;
                        new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
                    }
                }
            }
            if is_input_cancel == false && fighter.is_motion(Hash40::new("attack_air_f2")) { //aerial cancel
                match fighter.get_aerial() {
                    Some(AerialKind::Fair) | None => return,
                    _ => {
                        is_input_cancel = true;
                        new_status = *FIGHTER_STATUS_KIND_ATTACK_AIR;
                    }
                }
            }
            if is_input_cancel && VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::IS_ACTIVATE) { //flag set at cancel enable frame
                if fighter.is_motion(Hash40::new("attack_air_lw")) && new_status == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                    return;
                }
                if new_status == *FIGHTER_STATUS_KIND_SPECIAL_S || new_status == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                    VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_CANCEL_THIS_AIRTIME);
                }
                fighter.change_status_req(new_status, false);
                return;
            }
        }
    }
}

unsafe fn jump_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) { //jump cancel fx/logic
    if fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == 2 { 
        if boma.status_frame() <= 5 {
            if (fighter.is_cat_flag(Cat1::SpecialS) && !VarModule::is_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL)) 
            || (fighter.is_cat_flag(Cat1::SpecialHi) && !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) &&!VarModule::is_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL)) 
            && VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) < 2 {
                let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma) - 6.3, z: 0.0 };
                PostureModule::set_pos(boma, &pos);
                EFFECT(fighter, Hash40::new("bayonetta_witchtime_flash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
               fighter.set_int(1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
        } else if fighter.is_cat_flag(Cat1::SpecialAny) {
            if (fighter.is_cat_flag(Cat1::SpecialS) && VarModule::is_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL)) 
            || (fighter.is_cat_flag(Cat1::SpecialHi) && (fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) || VarModule::is_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL)))
            {
                return;
            }
            EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("bayonetta_feather_twinkle"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.8, true, 0, 0, 0, 0, 0, false, false);
            PLAY_SE(fighter, Hash40::new("vc_bayonetta_ottotto"));
        }
    }
}

unsafe fn nspecial_mechanics(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE) { //PM-like neutral-b canceling
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.is_cat_flag(Cat1::AirEscape)  {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
                ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }//drift
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4);
            app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
            let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
            let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
            let facing = PostureModule::lr(boma);
            let stick_x = ControlModule::get_stick_x(boma);
            let stick_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x"));
            if stick_x.abs() > stick_threshold { //slows drift down
                KineticModule::add_speed(boma, &Vector3f::new(-0.2 * (air_accel_x_add * stick_x.signum() + air_accel_x_mul * stick_x) * facing, 0.0, 0.0));
            }
        }//platdrop
        if ControlModule::get_stick_y(fighter.module_accessor) < -0.66 && GroundModule::is_passable_ground(fighter.module_accessor) {
            GroundModule::pass_floor(fighter.module_accessor);
            ControlModule::clear_command;
        }
    }
}

unsafe fn reset_flags_resources(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let boma = fighter.boma();
    if !fighter.is_situation(*SITUATION_KIND_AIR) && !fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) //checks for (re)spawn or grounded state
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD,
                                  *FIGHTER_STATUS_KIND_REBIRTH,
                                  *FIGHTER_STATUS_KIND_WIN,
                                  *FIGHTER_STATUS_KIND_LOSE,
                                  *FIGHTER_STATUS_KIND_ENTRY]){
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 0);
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_CANCEL_THIS_AIRTIME, 0);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    //resets flags if hit
    if StopModule::is_damage(boma) || fighter.is_status(*FIGHTER_STATUS_KIND_DAMAGE_AIR) {
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_CANCEL_THIS_AIRTIME, 0);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) > 1 {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 1);
        }
    } 
    //resource limit
    if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) >= 2 {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    //manages hit-flag, needed bc specials switch statuses
    if fighter.is_status_one_of(&[*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP]) {
    //disables on startup
        if boma.status_frame() == 1 && !fighter.is_status_one_of(&[*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT]) {
            VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT);
            return;
        }
        //enables hit flag after filtering bullet arts hits
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && VarModule::get_int(fighter.battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID) < 6 {
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT); 
        } 
        // restores resource once on-hit
        if !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D])
        && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::IS_ACTIVATE) 
        && VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) {
            if boma.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) && fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) < 2 {
                VarModule::off_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
            }
            VarModule::dec_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED);
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::IS_ACTIVATE)
        }
    } else { //effect showing depletion of 1+ resource
        if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) > 0 && fighter.is_situation(*SITUATION_KIND_AIR) {
            let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_falling_smoke"), Hash40::new("bust"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.5, true, 0, 0, 0, 0, 0, false, false) as u32;
            LAST_EFFECT_SET_ALPHA(fighter, 0.45);
            LAST_EFFECT_SET_RATE(fighter, 2.0);
            EffectModule::set_rgb(boma, effect, 2.0, 0.5, 2.0);
        } 
    }
    if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) == 0 || !fighter.is_situation(*SITUATION_KIND_AIR) {
        EffectModule::kill_kind(boma, Hash40::new("sys_falling_smoke"), false, true);
    } //clear effect
}

unsafe fn fair_momentum_handling(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    // Stolen from sora
    let control_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) as *mut smash::app::KineticEnergy;
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let facing = PostureModule::lr(boma);
    let stick_x = ControlModule::get_stick_x(boma);
    let stick_y = ControlModule::get_stick_y(boma);
    let stick_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x"));
    if boma.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F){
        if boma.is_motion(Hash40::new("attack_air_f")) || boma.is_motion(Hash40::new("attack_air_f2")) {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && VarModule::get_int(fighter.battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID) < 6 {
                if boma.is_motion(Hash40::new("attack_air_f")) {
                    smash::app::lua_bind::KineticEnergy::mul_speed(control_energy, &Vector3f::new(0.55, 1.0, 1.0));
                } else if boma.is_motion(Hash40::new("attack_air_f2")) {
                    smash::app::lua_bind::KineticEnergy::mul_speed(control_energy, &Vector3f::new(0.75, 1.0, 1.0)); 
                }
            } //cut speed on hit
            if stick_x.abs() > stick_threshold {
                // air drift resistance
                KineticModule::add_speed(boma, &Vector3f::new(-0.14* (air_accel_x_add * stick_x.signum() + air_accel_x_mul * stick_x) * facing, 0.0, 0.0));
            }
        }
    }
}

unsafe fn abk(fighter: &mut smash::lua2cpp::L2CFighterCommon, frame: f32) {
    let boma = fighter.boma();
    let pos = PostureModule::lr(boma);
    if StatusModule::is_changing(boma) {
        return;
    }
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) {
        //set rotation
        let angle = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::ABK_ANGLE_ROUNDED) as f32;
        joint_rotator(fighter, frame, Hash40::new("top"), Vector3f{x: -17.0*angle, y:90.0*pos, z:0.0}, 10.0, 13.0, 28.0, 32.0);
        if boma.status_frame() <= 7 { 
            let stick_y =  ControlModule::get_stick_y(fighter.module_accessor);
            //dabk input
            if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW) || fighter.is_button_on(Buttons::Catch)) && boma.status_frame() < 6 {
                VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            // angle assignment
            } else if stick_y > 0.1 {
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::ABK_ANGLE_ROUNDED, 1);
            } else if stick_y < -0.1 {
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::ABK_ANGLE_ROUNDED, -1);   
            } else if stick_y >= -0.1 && stick_y <= 0.1 {
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::ABK_ANGLE_ROUNDED, 0);
            }
        }
        //trajectory change
        if boma.status_frame() >= 6 && boma.status_frame() <= 22 {
            if !fighter.is_in_hitlag() {
            KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f::new( -0.4 * angle * pos, angle*0.667, 0.0));
            }
        }
    }
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D) { //standardize dabk startup to 9 frames total, make up for movement in animation skipped so that both dabk inputs are comparable 
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            MotionModule::set_frame_sync_anim_cmd(boma, 6.0, true, false, false);
            VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma) - 0.5 * pos, y: PostureModule::pos_y(boma) + 7.0, z: 0.0 };
            PostureModule::set_pos(boma, &pos);
        }
    }
}

unsafe fn heel_slide_off(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor) {
    let boma = fighter.boma();
    let pos = PostureModule::lr(boma);
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
        if boma.status_frame() <= 40 && boma.status_frame() >= 15 && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        } else {
            GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
    }
}

unsafe fn branching_ftilt_jab(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    let b_press = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL);
    let a_press = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    if MotionModule::motion_kind(boma) == hash40("attack_s3_s") && fighter.motion_frame() > 25.0 {
        if b_press {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false); 
        }
    }
    if MotionModule::motion_kind(boma) == hash40("attack_s3_s2") {
        if fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
            AttackModule::clear_all(boma); //quickfix for ba not clearing hitbox
        }
        if stick_y > 0.66 { //hold up kick
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        }
        if fighter.motion_frame() <= 5.0 {
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
                StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            }
        }
    }
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        if MotionModule::motion_kind(boma) == hash40("attack_12") {//jab2
            if fighter.motion_frame() > 13.0 {
                if b_press {
                    StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK, false);//jab3
                }
            }
            if fighter.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100) && a_press {
                StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK_100, false);//rapid
            }
        }
        if MotionModule::motion_kind(boma) == hash40("attack_11") { //jab1
            if fighter.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && a_press {
                StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK, false);
            }
        }
    }
}

unsafe fn bat_within_air_motion(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if ((fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN) && fighter.is_situation(*SITUATION_KIND_AIR)) || fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_LW_BATWITHIN)) {
        let stick_y =  ControlModule::get_stick_y(fighter.module_accessor);
        let stick_x =  ControlModule::get_stick_x(fighter.module_accessor);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.4 * stick_y);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4 * stick_x);
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

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    aerial_cancels(fighter, boma);
    jump_cancels(fighter, boma);
    nspecial_mechanics(fighter, boma);
    reset_flags_resources(fighter, boma);
    fair_momentum_handling(fighter, boma);
    abk(fighter, frame);
    heel_slide_off(fighter, boma);
    branching_ftilt_jab(fighter);
    bat_within_air_motion(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_BAYONETTA)]
pub unsafe fn bayonetta_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    bayonetta_frame(fighter);
}
pub unsafe fn bayonetta_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.frame);
    }
}
