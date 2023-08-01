use super::*;

// #[status_script(agent = "krool", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
// unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
//     StatusModule::init_settings(
//         fighter.module_accessor,
//         app::SituationKind(*SITUATION_KIND_NONE),
//         *FIGHTER_KINETIC_TYPE_UNIQ,
//         *GROUND_CORRECT_KIND_KEEP as u32,
//         app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
//         true,
//         *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
//         *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
//         *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
//         0
//     );

//     FighterStatusModuleImpl::set_fighter_status_data(
//         fighter.module_accessor,
//         false,
//         *FIGHTER_TREADED_KIND_NO_REAC,
//         false,
//         false,
//         false,
//         (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
//         (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION) as u32,
//         *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
//         0
//     );

//     0.into()
// }

#[status_script(agent = "krool", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0);
    }
    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if !fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }
    0.into()
}
pub fn install() {
    smashline::install_status_scripts!(
        special_lw_main,
    );
}
