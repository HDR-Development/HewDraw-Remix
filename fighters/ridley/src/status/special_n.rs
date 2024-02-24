use super::*;
use globals::*;

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_n_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_FIRE_NUM);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hold"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_hold"), 0.0, 1.0, false, 0.0, false, false);
    }
    let max_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_charge_frame")) as f32;
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    MotionModule::set_rate(fighter.module_accessor, end_frame/max_charge_frame);
    HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_NORMAL);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    if !StopModule::is_stop(fighter.module_accessor) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_n_charge_substatus as *const () as _));

    fighter.main_shift(special_n_charge_main_loop)
}

unsafe extern "C" fn special_n_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n_situation_helper(fighter);
    }
    let mut min_weak_size = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("min_weak_size"));
    let mut max_weak_size = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("max_weak_size"));
    if fighter.kind() == *FIGHTER_KIND_KIRBY {
        min_weak_size = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("min_weak_size_kirby"));
        max_weak_size = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("max_weak_size_kirby"));
    }
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    let size_ratio = (max_weak_size - min_weak_size);
    let ratio = MotionModule::frame(fighter.module_accessor) / end_frame;
    let size = min_weak_size + (size_ratio * ratio);
    WorkModule::set_float(fighter.module_accessor, size, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_FLOAT_WEAK_SIZE);
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("virtualweakpoint"), &Vector3f::new(size, size, size));
    if !MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            VarModule::on_flag(fighter.object(), vars::ridley::instance::SPECIAL_N_EXPLODE);
        }
        else if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            return 0.into()
        }
    }
    let max_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
    let charge_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    let max_fire_num = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_fire_num"));
    let fire_num = ((max_fire_num - 1) * (charge_count / max_charge_frame)) + 1;
    WorkModule::set_int(fighter.module_accessor, fire_num, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_FIRE_NUM);
    fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());

    return 0.into()
}

unsafe extern "C" fn special_n_charge_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    }

    return 0.into()
}

#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_n_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.object(), vars::ridley::instance::SPECIAL_N_EXPLODE) {
        VarModule::off_flag(fighter.object(), vars::ridley::instance::SPECIAL_N_EXPLODE);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_explode") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_explode") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_explode"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_explode"), 0.0, 1.0, false, 0.0, false, false);
        }
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_NORMAL);

        fighter.sub_shift_status_main(L2CValue::Ptr(special_n_shoot_main_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

unsafe extern "C" fn special_n_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) != StatusModule::prev_situation_kind(fighter.module_accessor) {
        special_n_situation_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool()
        || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }

    false.into()
}

unsafe extern "C" fn special_n_situation_helper(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
}

pub fn install() {
    install_status_scripts!(
        special_n_charge_main,
        special_n_shoot_main,
    );
}