use super::*;
use globals::*;

/// Hold neutral special to explode
#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_RIDLEY_SPECIAL_N_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_n_shoot_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_FIRE_NUM) >= WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_fire_num"))
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        WorkModule::set_int64(fighter.module_accessor, hash40("ridley_special_n_explode") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("ridley_special_air_n_explode") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("ridley_special_n_explode"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("ridley_special_air_n_explode"), 0.0, 1.0, false, 0.0, false, false);
        }
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_NORMAL);
        fighter.sub_shift_status_main(L2CValue::Ptr(special_n_shoot_main_loop as *const () as _))
    }else {
        original!(fighter)
    }
}
unsafe extern "C" fn special_n_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) != StatusModule::prev_situation_kind(fighter.module_accessor) {
        special_n_air_to_ground_transition(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool()
        || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    false.into()
}
unsafe extern "C" fn special_n_air_to_ground_transition(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
}

pub fn install() {
    smashline::install_status_scripts!(
        special_n_shoot_status_main
    );
}
