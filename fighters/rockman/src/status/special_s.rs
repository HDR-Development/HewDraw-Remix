use super::*;
use super::helper::*;


unsafe extern "C" fn rockman_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON |
            *FIGHTER_LOG_MASK_FLAG_SHOOT
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}


unsafe extern "C" fn rockman_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_METALBLADE) {
        LinkModule::send_event_nodes(
            fighter.module_accessor,
            *LINK_NO_ARTICLE,
            Hash40::new_raw(0x273692b070),
            0
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(rockman_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn rockman_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    let mut change_motion = false;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_STICK_CHECK_FRAME_END) {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        if 0.1225 < stick_x * stick_x + stick_y * stick_y {
            WorkModule::set_float(fighter.module_accessor, stick_x, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLOAT_STICK_X);
            WorkModule::set_float(fighter.module_accessor, stick_y, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLOAT_STICK_Y);
        }
    }
    else if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_STICK_CHECK_ACCEPT) {
        let stick_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLOAT_STICK_X);
        let stick_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLOAT_STICK_Y);
        if 0.1225 < stick_x * stick_x + stick_y * stick_y {
            let lr = PostureModule::lr(fighter.module_accessor);
            let atan = stick_y.atan2(stick_x * lr);
            let rad = 112.5f32.to_radians();
            let rad_negative = -112.5f32.to_radians();
            if rad <= atan || atan <= rad_negative {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_TURN_MOTION);
                change_motion = true;
            }
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_STICK_CHECK_ACCEPT);
    }

    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor)
    || change_motion {
        let mot_g;
        let mot_a;
        let metal_blade_exist = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_TURN_MOTION) {
            if metal_blade_exist {
                mot_g = hash40("special_n_turn_empty");
                mot_a = hash40("special_air_n_turn_empty");
            }
            else {
                mot_g = hash40("special_n_turn");
                mot_a = hash40("special_air_n_turn");
            }
        }
        else {
            if metal_blade_exist {
                mot_g = hash40("special_n_empty");
                mot_a = hash40("special_air_n_empty");
            }
            else {
                mot_g = hash40("special_n");
                mot_a = hash40("special_air_n");
            }
        }
        rockman_special_motion_helper(
            fighter,
            mot_g.into(),
            mot_a.into(),
            FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),
            FIGHTER_KINETIC_TYPE_FALL.into(),
            FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_FIRST.into(),
            GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK.into()
        );
        if sit == *SITUATION_KIND_GROUND {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ATTACK_HI3_LANDING);
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if sit == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}


unsafe extern "C" fn rockman_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_INT_METALBLADE_ID) != 0 {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x29b79a80a1));
    }
    0.into()
}


pub fn install() {
    smashline::Agent::new("rockman")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, rockman_special_s_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, rockman_special_s_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, rockman_special_s_end)
        .install();
}
