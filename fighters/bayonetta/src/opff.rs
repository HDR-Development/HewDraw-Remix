use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn aerial_cancels(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F])
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && !fighter.is_motion_one_of(&[Hash40::new("attack_air_f"), Hash40::new("attack_air_f_hold"), Hash40::new("attack_air_f2_hold"), Hash40::new("attack_air_f3_hold"), Hash40::new("attack_air_lw_hold")])
    && !fighter.is_in_hitlag()
    {
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

        if fighter.is_cat_flag(Cat1::SpecialS) { //2 side b cancels
            if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_S_CANCEL_THIS_AIRTIME) <= 1 {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_S;
            }
        }

        if fighter.is_cat_flag(Cat1::SpecialHi) { //1 up b cancel
            if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME) <= 0 {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
            }
        }

        if is_input_cancel { //cancel conditions / resource checks, generally enables cancels after hitboxes clear/sourspot out so that opponent isnt locked in place
            if fighter.is_motion(Hash40::new("attack_air_hi")) && fighter.motion_frame() < 18.0 { //frame 12 sourspot
                return;
            } else if fighter.is_motion(Hash40::new("attack_air_f3")) && fighter.motion_frame() < 16.0 { //frame 11 fair3
                return;
            } else if fighter.is_motion(Hash40::new("attack_air_f2")) && fighter.motion_frame() < 12.0 { //frame 9, 2 frames after hitbox clear
                return;
            } else if fighter.is_motion(Hash40::new("attack_air_lw")) && (new_status == *FIGHTER_STATUS_KIND_JUMP_AERIAL || fighter.motion_frame() < 33.0 ) {// disable dair jump cancel, cancels after sweetspot ends frame 26
                return;
            } else if fighter.is_motion(Hash40::new("attack_air_n")) && fighter.motion_frame() < 29.0 { //leg behind bayo frame 12
                return;
            } else if fighter.is_motion(Hash40::new("attack_air_b")) && fighter.motion_frame() < 25.0 { //frame 16
                return;
            }
            if new_status == *FIGHTER_STATUS_KIND_SPECIAL_S  {
                VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_S_CANCEL_THIS_AIRTIME);
            }
            if new_status == *FIGHTER_STATUS_KIND_SPECIAL_HI  {
                VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME);
            }
            fighter.change_status_req(new_status, false);
            return;
        }
    }
}

unsafe fn special_cancels(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor) { //both jump cancel fx/logic and infliction status checks that broke in acmd
    let boma = fighter.boma();
    if fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) { //enable jump cancel flag
        if boma.status_frame() <= 5 {
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP);
            }
        if boma.status_frame() > 5 && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::IS_ACTIVATE) { //jump cancel failure visual/audio cue
            VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP); //disable flag
            if fighter.is_cat_flag(Cat1::SpecialS) || fighter.is_cat_flag(Cat1::SpecialHi) {
                VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::IS_ACTIVATE);
                EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 5, 5, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
                EFFECT(fighter, Hash40::new("bayonetta_butterfly_landing"), Hash40::new("top"), 5, 5, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
                PLAY_SE(fighter, Hash40::new("vc_bayonetta_ottotto"));
                PLAY_SE(fighter, Hash40::new("se_bayonetta_landing03"));
            }
        }
    }
    if fighter.is_status_one_of(&[*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP]) {//each special has 2 statuses
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) {
            VarModule::dec_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED); //recover special once per hit
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT);
        }
        if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) { //cancel frames
            if boma.status_frame() == 1 && !fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP) {
                VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT); //clears hit flag at startup
            }
            if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP]) {
                if WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_GROUND_START) {
                    if boma.status_frame() == 29 {
                        KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f::new( 0.0, 7.0, 0.0));
                    }
                    if boma.status_frame() == 32 { //normal hit cancel, 7 less frames of lag than miss
                        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_NO_SHOOTING_ENABLE_CANCEL);
                    }
                    if boma.status_frame() == 36 { //ba cancels
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
                else { //air (1 frame faster startup)
                    if boma.status_frame() == 28 { 
                        KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f::new( 0.0, 7.5, 0.0));
                        WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_NO_SHOOTING_ENABLE_CANCEL);
                    } else if boma.status_frame() == 35 { //enables BA cancels
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }
            else if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) && WorkModule::get_int(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) < 3 {
                if boma.status_frame() >= 26 && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::IS_ACTIVATE) {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
                if boma.status_frame() >= 33 { //ba abk cancel frame so that it isnt a suicide button on-hit but doesnt cancel instantly
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }
        }
        if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP) { //jc effect and height
            let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma) - 6.5, z: 0.0 };
            PostureModule::set_pos(boma, &pos);
            VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP);
            PLAY_SE(fighter, Hash40::new("se_bayonetta_swing_punch_03"));
            EFFECT(fighter, Hash40::new("bayonetta_change_circle"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
            if !fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
                WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
        }
    } else { //disable hit flag
        VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT);
    }
}

unsafe fn nspecial_mechanics(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor) {
    //PM-like neutral-b canceling
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE) {
        if fighter.is_situation(*SITUATION_KIND_AIR) && fighter.is_cat_flag(Cat1::AirEscape)  {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
        }//drift
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            let stick_x =  ControlModule::get_stick_x(fighter.module_accessor);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.45 * stick_x);
        }//platdrop
        if ControlModule::get_stick_y(fighter.module_accessor) < -0.66 && GroundModule::is_passable_ground(fighter.module_accessor) {
            GroundModule::pass_floor(fighter.module_accessor);
        }
    }
}

unsafe fn reset_flags_resources(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor) { //rewrote because undesired things happened
    let boma = fighter.boma();
    if !fighter.is_situation(*SITUATION_KIND_AIR) //checks for (re)spawn or grounded state
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD,
                                  *FIGHTER_STATUS_KIND_REBIRTH,
                                  *FIGHTER_STATUS_KIND_WIN,
                                  *FIGHTER_STATUS_KIND_LOSE,
                                  *FIGHTER_STATUS_KIND_ENTRY]){
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 0);
        VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP);
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_S_CANCEL_THIS_AIRTIME, 0);
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME, 0);
        if !fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) { //clears flag only after slide ends
            VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_HEEL_SLIDE);
        }
    }
    if StopModule::is_damage(boma) || fighter.is_status(*FIGHTER_STATUS_KIND_DAMAGE_AIR) { //resets flags if hit
        VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP);
        VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_HEEL_SLIDE);
        if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) > 0 {
            if !fighter.is_status(*FIGHTER_STATUS_KIND_DAMAGE_AIR) { //tumble check
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 0); //2 specials
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_S_CANCEL_THIS_AIRTIME, 0); //reset special cancels
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME, 0);
            } else { //no tumble
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 1); //1 special
            }
        }
    }
    else{ //disables specials on resource burnout
        if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) >= 2 {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        }
        else{ //enables specials on replenish
            VarModule::off_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
            VarModule::off_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        }
    }
}

unsafe fn abk(fighter: &mut smash::lua2cpp::L2CFighterCommon, frame: f32) {
    let boma = fighter.boma();
    let pos = PostureModule::lr(boma);
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) {
        let angle = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::ABK_ANGLE_ROUNDED) as f32;
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && boma.status_frame() <= 6 { //6 frame dabk window
                StatusModule::change_status_request(boma, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, false);
                WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S_U_TO_D);
            }
        if boma.status_frame() <= 7 { //gather rotation
            let stick_y =  ControlModule::get_stick_y(fighter.module_accessor);
            if stick_y > 0.1 {
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::ABK_ANGLE_ROUNDED, 1);
            } else if stick_y < -0.1 {
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::ABK_ANGLE_ROUNDED, -1);   
            } else if stick_y >= -0.1 && stick_y <= 0.1 {
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::ABK_ANGLE_ROUNDED, 0);
            }
        }
        if boma.status_frame() >= 6 && boma.status_frame() <= 22 { //rotate
            ModelModule::set_joint_rotate(boma, Hash40::new("top"), &Vector3f{ x: -17.0*angle, y: 90.0*pos, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            if !fighter.is_in_hitlag() {
            KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f::new( -0.4 * angle * pos, angle*0.667, 0.0));
            }
        }
    }
}

unsafe fn heel_slide_off(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor) {
    let boma = fighter.boma();
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) {
        if boma.status_frame() < 26 && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::IS_HEEL_SLIDE);
        }
        if boma.status_frame() > 26 || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
    }
    if fighter.is_status(*FIGHTER_STATUS_KIND_DAMAGE_FALL) && VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HEEL_SLIDE) { //go into freefall instead of tumble
        StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_FALL, false);
        VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_HEEL_SLIDE)
    }
}

unsafe fn branching_ftilt_jab(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    let b_press = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL);
    let a_press = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK);
    if StatusModule::is_changing(fighter.module_accessor) { //needed bc have to check for motion 
        return;
    }
    if MotionModule::motion_kind(boma) == hash40("attack_s3_s") && fighter.motion_frame() > 25.0 {
        if b_press {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);        
        }
    }
    if MotionModule::motion_kind(boma) == hash40("attack_s3_s2") && (fighter.motion_frame() < 4.0 || WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)) {
        if b_press {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
        }
    }
    if MotionModule::motion_kind(boma) == hash40("attack_12") {//jab2
        if fighter.motion_frame() > 13.0 {
            if b_press {
                StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK, false);//jab3
            }
        }
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100) && a_press {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK_100, false);//rapid
        }
    }
    if MotionModule::motion_kind(boma) == hash40("attack_11") { //jab1
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && a_press {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK, false);
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

#[utils::macros::opff(FIGHTER_KIND_BAYONETTA)]
pub unsafe fn bayonetta_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);

    if let Some(info) = FrameInfo::update_and_get(fighter) {
        aerial_cancels(fighter);
        special_cancels(fighter, info.boma);
        nspecial_mechanics(fighter, info.boma);
        reset_flags_resources(fighter, info.boma);
        abk(fighter, info.frame);
        heel_slide_off(fighter, info.boma);
        branching_ftilt_jab(fighter);
        bat_within_air_motion(fighter);
    }
}