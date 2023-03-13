use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn aerial_cancels(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if !fighter.is_situation(*SITUATION_KIND_AIR){
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_S_CANCEL_THIS_AIRTIME, 0);
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME, 0);
    }

    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F])
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && !fighter.is_motion_one_of(&[Hash40::new("attack_air_f"), Hash40::new("attack_air_f_hold"), Hash40::new("attack_air_f2_hold"), Hash40::new("attack_air_f3_hold"), Hash40::new("attack_air_lw_hold")])
    && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::IS_BULLET_ARTS)
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

        if fighter.is_cat_flag(Cat1::SpecialS) {
            if  WorkModule::get_int(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) <= 2 && VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_S_CANCEL_THIS_AIRTIME) <= 1 {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_S;
            }
        }

        if fighter.is_cat_flag(Cat1::SpecialHi) {
            if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME) <= 0 {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
            }
        }

        if is_input_cancel {
            if fighter.is_motion(Hash40::new("attack_air_hi")) {
                if fighter.motion_frame() < 22.0 {
                    return;
                }
            }
            if fighter.is_motion(Hash40::new("attack_air_f3")) {
                if fighter.motion_frame() < 18.0 {
                    return;
                }
            }
            if !fighter.is_in_hitlag(){
                //disable fair1 special/attack cancel
                // if fighter.is_motion(Hash40::new("attack_air_f")) {
                //     if new_status != *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                //         return;
                //     }
                // }
                if fighter.is_motion(Hash40::new("attack_air_f2")) && fighter.motion_frame() < 13.5 {
                    return;
                }
                // disable dair jump cancel
                if fighter.is_motion(Hash40::new("attack_air_lw")) {
                    if new_status == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                        return;
                    }
                }
                if new_status == *FIGHTER_STATUS_KIND_SPECIAL_S  {
                    VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_S_CANCEL_THIS_AIRTIME);
                }
                if new_status == *FIGHTER_STATUS_KIND_SPECIAL_HI  {
                    VarModule::inc_int(fighter.battle_object, vars::bayonetta::instance::NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME);
                }
                VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::IS_NONSPECIAL_CANCEL);
                fighter.change_status_req(new_status, false);
                return;
            }
        }
    }
}

unsafe fn special_cancels(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    // Special Cancels
    if fighter.is_status(*FIGHTER_STATUS_KIND_FALL) {
        VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP);
    }
    if fighter.is_motion_one_of(&[Hash40::new("jump_aerial_f"), Hash40::new("jump_aerial_b")]) {
        if fighter.motion_frame() < 6.0 {
            VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP);
            }
        if fighter.motion_frame() > 5.0 && fighter.motion_frame() < 9.0 {
            VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP);
            if fighter.is_cat_flag(Cat1::SpecialS) || fighter.is_cat_flag(Cat1::SpecialHi) {
                EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 5, 5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                PLAY_SE(fighter, Hash40::new("vc_bayonetta_ottotto"));
            }
        }
    }
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && fighter.motion_frame() > 32.0
    && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::status::IS_BULLET_ARTS)
    && WorkModule::get_int(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) < 3
    {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe fn nspecial_cancels(fighter: &mut L2CFighterCommon) {
    //PM-like neutral-b canceling
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE) {
        if fighter.is_situation(*SITUATION_KIND_AIR) && fighter.is_cat_flag(Cat1::AirEscape) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
        }
        let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
        let pass_flick_y = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("pass_flick_y"));
        if GroundModule::is_passable_ground(fighter.module_accessor)
        && fighter.global_table[FLICK_Y].get_i32() < pass_flick_y
        && fighter.global_table[STICK_Y].get_f32() < pass_stick_y
        {
            GroundModule::pass_floor(fighter.module_accessor);
        }
    }
}

unsafe fn recovery_resource_management(fighter: &mut L2CFighterCommon) {
    if !fighter.is_situation(*SITUATION_KIND_AIR)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD,
                                  *FIGHTER_STATUS_KIND_REBIRTH,
                                  *FIGHTER_STATUS_KIND_WIN,
                                  *FIGHTER_STATUS_KIND_LOSE,
                                  *FIGHTER_STATUS_KIND_ENTRY]){
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 0);
        VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP);
    }
    else{
        if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) >= 2 {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        }
        else{
            VarModule::off_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
            VarModule::off_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        }
        if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL]){
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 0);
        }
    }
}

unsafe fn clear_proration(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor) {
    if (fighter.is_prev_situation(*SITUATION_KIND_AIR) && fighter.is_situation(*SITUATION_KIND_GROUND)) || 
        (fighter.is_prev_situation(*SITUATION_KIND_AIR) && fighter.is_situation(*SITUATION_KIND_CLIFF)) ||
        fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL]) && VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_NONSPECIAL_CANCEL) {
            VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_NONSPECIAL_CANCEL);
            VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_JUMP_KEEP);
        }
}

unsafe fn abk_flight_drift(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) && fighter.motion_frame() < 26.0 && !StopModule::is_stop(fighter.module_accessor){
        let stick_y =  ControlModule::get_stick_y(fighter.module_accessor);
        if stick_y != 0.0 && !fighter.is_in_hitlag(){
            KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f::new(0.0, 1.0 * stick_y, 0.0));
        }
    }
}

unsafe fn hold_dabk(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor) {
    if fighter.is_situation(*SITUATION_KIND_AIR) && fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) {
        if fighter.motion_frame() < 12.0 {
            if (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW)) {
                StatusModule::change_status_request(boma, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, false);
                WorkModule::on_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S_U_TO_D);
            }
        }
    }
}

unsafe fn heel_slide_off(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) {
        if fighter.motion_frame() < 22.0 {
            GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
        else {
            GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
    }
}

unsafe fn neutral_b_drift(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor) {
    if (fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) || fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE)) && fighter.is_situation(*SITUATION_KIND_AIR) {
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let stick_x =  ControlModule::get_stick_x(fighter.module_accessor);
        if !fighter.is_motion_one_of(&[Hash40::new("game_specialairnchargeh"), Hash40::new("game_specialairnchargef")]) {
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.25 * stick_x);
        }
        else {
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.25 * stick_x);
        }     
    }
}

unsafe fn branching_ftilt_jab(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    let b_press = (ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW));
    let a_press = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK);
    if MotionModule::motion_kind(boma) == hash40("attack_s3_s") && fighter.motion_frame() > 26.5 {
        if b_press {
            MotionModule::change_motion(boma, Hash40::new("attack_s3_s3"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    }
    if MotionModule::motion_kind(boma) == hash40("attack_12") {
        if fighter.motion_frame() > 13.0 {
            if b_press == true {
                StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK, false);
            }
        }
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100) && a_press {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK_100, false);
        }
    }
    if MotionModule::motion_kind(boma) == hash40("attack_11") {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) && a_press {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_ATTACK, false);
        }
    }
}

unsafe fn bat_within_speed(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if ((fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN) && fighter.is_situation(*SITUATION_KIND_AIR)) || fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_LW_BATWITHIN)) {
        let stick_y =  ControlModule::get_stick_y(fighter.module_accessor);
        let stick_x =  ControlModule::get_stick_x(fighter.module_accessor);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.38 * stick_y);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.38 * stick_x);
    }
}

#[utils::macros::opff(FIGHTER_KIND_BAYONETTA )]
pub unsafe fn bayonetta_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);

    if let Some(info) = FrameInfo::update_and_get(fighter) {
        aerial_cancels(fighter);
        special_cancels(fighter);
        nspecial_cancels(fighter);
        recovery_resource_management(fighter);
        abk_flight_drift(fighter);
        hold_dabk(fighter, info.boma);
        neutral_b_drift(fighter, info.boma);
        heel_slide_off(fighter, info.boma);
        clear_proration(fighter, info.boma);
        branching_ftilt_jab(fighter);
        bat_within_speed(fighter);
    }

    
}