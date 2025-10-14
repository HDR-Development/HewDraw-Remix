use super::*;

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
    fighter.sub_change_motion_by_situation(Hash40::new("special_hi_start").into(), Hash40::new("special_air_hi_start").into(), false.into());
    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_hi").into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    fighter.main_shift(special_hi_main_loop)
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE.into(), false.into());
        return 0.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_HI_FLAG_REVERSE_LR) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
        let stick_x = fighter.global_table[STICK_X].get_f32().abs();
        let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
        if lr_stick_x <= stick_x {
            PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
    }

    let changed = fighter.sub_set_ground_correct_by_situation(true.into()).get_bool();
    fighter.sub_set_special_start_inherit_common_kinetic_setting(Hash40::new("param_special_hi").into());
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        let start_stop_y_frame_air = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("start_stop_y_frame_air"));
        if changed {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
            if fighter.global_table[CURRENT_FRAME].get_i32() + 1 < start_stop_y_frame_air {
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    0.0
                );
            }
        }
        if start_stop_y_frame_air <= fighter.global_table[CURRENT_FRAME].get_i32() + 1 {
            let fall_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_speed_y"));
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -fall_speed_y
            );
        }
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
}