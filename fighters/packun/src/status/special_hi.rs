use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_HI

pub unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_PACKUN_CLIFF_HANG_DATA_SPECIAL_HI as u32);
	fighter.main_shift(special_hi_main_loop)
}

pub unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_i32();
    WorkModule::set_int(fighter.module_accessor, current_frame, *FIGHTER_PACKUN_STATUS_SPECIAL_HI_WORK_INT_STATUS_FRAME);
    if fighter.is_motion(Hash40::new("special_hi")) {
        fighter.set_situation(L2CValue::I32(*SITUATION_KIND_GROUND));
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 0.into();
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    
    // Shield cancel
    // if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    // && fighter.global_table[CURRENT_FRAME].get_i32() >= 30 {
    //     fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
    //     return 0.into();
    // }

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_HI_FLAG_START_RISE) {
        let start_rise_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("start_rise_frame"));
        if fighter.motion_frame() >= start_rise_frame as f32 {
            fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
            GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("accel_y"));
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                accel_y
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_HI_FLAG_START_RISE);
        }
    }
  
    let start_no_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("start_no_landing_frame"));
    if current_frame >= start_no_landing_frame {
        if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_AIR
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        {
            fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
            return 0.into();
        }
    }

    let stop_add_speed_y_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("stop_add_speed_y_frame"));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_SPECIAL_HI_FLAG_START_RISE)
    || current_frame < stop_add_speed_y_frame
    {
        0.into()
    }
    else {
        fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        0.into()
    }
}

pub unsafe extern "C" fn special_hi_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Init, fighter, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END)(fighter);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.75, 0.0);
    }

    ret
}

// FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING
unsafe extern "C" fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_PACKUN_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);

    fighter.main_shift(special_hi_end_main_loop)
}

unsafe extern "C" fn special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    fighter.sub_air_check_dive();
    let status_frame = fighter.get_int(*FIGHTER_PACKUN_STATUS_SPECIAL_HI_WORK_INT_STATUS_FRAME);
    let mut start_no_landing_frame = fighter.get_param_int("param_special_hi", "start_no_landing_frame");
    if fighter.is_motion(Hash40::new("special_hi")) && !fighter.is_prev_situation(*SITUATION_KIND_AIR) { start_no_landing_frame = 999; }
    if fighter.is_situation(*SITUATION_KIND_GROUND) && start_no_landing_frame <= status_frame {
        fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
    }
    let stop_add_speed_y_frame = fighter.get_param_int("param_special_hi", "stop_add_speed_y_frame");
    let end_frame_from_stop_add_speed = fighter.get_param_int("param_special_hi", "end_frame_from_stop_add_speed");
    if MotionModule::is_end(fighter.module_accessor)
    || end_frame_from_stop_add_speed <= status_frame - stop_add_speed_y_frame {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    if fighter.is_flag(*FIGHTER_PACKUN_STATUS_SPECIAL_HI_DIVE) {
        if !fighter.is_flag(*FIGHTER_PACKUN_STATUS_SPECIAL_HI_DIVE_DONE) {
            let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            let dive_cont_value = fighter.get_param_float("common", "dive_cont_value");
            if fighter.stick_y() <= dive_cont_value {
                let dive_flick_frame_value = fighter.get_param_int("common", "dive_flick_frame_value");
                if fighter.global_table[FLICK_Y].get_i32() < dive_flick_frame_value {
                    let dive_speed_y = fighter.get_param_float("dive_speed_y", "");
                    if sum_speed_y <= -dive_speed_y {
                        // There's some stuff here about BattleObjectWorld gravity coefficients but we no longer have those kind of stages so I don't care
                        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, dive_speed_y);
                        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, dive_speed_y);
                        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
                        fighter.on_flag(*FIGHTER_PACKUN_STATUS_SPECIAL_HI_DIVE_DONE);
                        fighter.check_mach_stamp();
                        EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_smash_flash_s"), Hash40::new("hip"), &Vector3f::new(0.0, 4.0, 8.0), &Vector3f::zero(), 1.1, &Vector3f::new(18.0, 12.0, 0.0), &Vector3f::zero(), false, 0, 0, 0);
                        let inertia_status = fighter.get_int(*FIGHTER_PACKUN_STATUS_SPECIAL_HI_WORK_INT_INERTIA_STATUS);
                        if inertia_status != *FIGHTER_PACKUN_SPECIAL_HI_TILT_INERTIA_STATUS_NONE {
                            fighter.set_int(*FIGHTER_PACKUN_SPECIAL_HI_TILT_INERTIA_STATUS_TO_CENTER_DIVE, *FIGHTER_PACKUN_STATUS_SPECIAL_HI_WORK_INT_INERTIA_STATUS);
                            fighter.set_int(0, *FIGHTER_PACKUN_STATUS_SPECIAL_HI_WORK_INT_PENDULUM_FRAME);
                        }
                    }
                }
            }
        }
    }
    if fighter.is_motion(Hash40::new("special_hi")) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
        }
        let stop_add_speed_y_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("stop_add_speed_y_frame"));
        if fighter.is_situation(*SITUATION_KIND_GROUND)
        && fighter.status_frame() >= stop_add_speed_y_frame {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING, false);
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_lag = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    let anim_length = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("special_hi_landing"));
    let rate: f32 = if landing_lag > 0 {
        anim_length / landing_lag as f32
    } else {
        1.0
    };
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_landing"), 0.0, rate, false, 0.0, false, false);
    fighter.main_shift(special_hi_landing_main_loop)
}

unsafe extern "C" fn special_hi_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
        }
        else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
        return 1.into();
    }
    // <HDR>
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        // edge cancel if started in the air
        let status = if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK)
            { *FIGHTER_STATUS_KIND_FALL_SPECIAL } else { *FIGHTER_STATUS_KIND_FALL };
        fighter.change_status_req(status, false);
        return 1.into();
    }
    // </HDR>
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);

    agent.status(Init, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, special_hi_end_init);
    
    agent.status(Main, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, special_hi_end_main);
    agent.status(Main, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING, special_hi_landing_main);
}