use super::*;
use globals::*;

pub unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_hi"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_hi"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
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
        fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 0.into();
    }

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    
    // Shield cancel
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && fighter.global_table[CURRENT_FRAME].get_i32() >= 30 {
        fighter.change_status(FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 0.into();
    }

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

// FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING

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
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
        return 1.into();
    }
    // <HDR>
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    // </HDR>
    0.into()
}

pub fn install() {
    smashline::Agent::new("packun")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main)
        .status(Main, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING, special_hi_landing_main)
        .install();
}