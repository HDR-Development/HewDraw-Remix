use super::*;
use globals::*;

unsafe extern "C" fn special_n_charge_main(agent: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_FIRE_NUM);
    WorkModule::set_int64(agent.module_accessor, hash40("special_n_hold") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(agent.module_accessor, hash40("special_air_n_hold") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_n_hold"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_n_hold"), 0.0, 1.0, false, 0.0, false, false);
    }
    let max_charge_frame = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_n"), hash40("max_charge_frame")) as f32;
    let end_frame = MotionModule::end_frame(agent.module_accessor);
    MotionModule::set_rate(agent.module_accessor, end_frame/max_charge_frame);
    HIT_NODE(agent, Hash40::new("virtualweakpoint"), *HIT_STATUS_NORMAL);
    WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    let charge = WorkModule::get_int(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    if !StopModule::is_stop(agent.module_accessor) {
        WorkModule::inc_int(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    }
    agent.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_n_charge_substatus as *const () as _));

    agent.main_shift(special_n_charge_main_loop)
}

unsafe extern "C" fn special_n_charge_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(agent.module_accessor) {
        if StatusModule::is_situation_changed(agent.module_accessor) {
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                agent.set_situation(SITUATION_KIND_GROUND.into());
                MotionModule::change_motion_inherit_frame(agent.module_accessor, Hash40::new("special_n_hold"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                agent.set_situation(SITUATION_KIND_AIR.into());
                MotionModule::change_motion_inherit_frame(agent.module_accessor, Hash40::new("special_air_n_hold"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    let mut min_weak_size = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_n"), hash40("min_weak_size"));
    let mut max_weak_size = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_n"), hash40("max_weak_size"));
    if agent.kind() == *FIGHTER_KIND_KIRBY {
        min_weak_size = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_n"), hash40("min_weak_size_kirby"));
        max_weak_size = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_n"), hash40("max_weak_size_kirby"));
    }
    let end_frame = MotionModule::end_frame(agent.module_accessor);
    let size_ratio = (max_weak_size - min_weak_size);
    let ratio = MotionModule::frame(agent.module_accessor) / end_frame;
    let size = min_weak_size + (size_ratio * ratio);
    WorkModule::set_float(agent.module_accessor, size, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_FLOAT_WEAK_SIZE);
    ModelModule::set_joint_scale(agent.module_accessor, Hash40::new("virtualweakpoint"), &Vector3f::new(size, size, size));
    if !MotionModule::is_end(agent.module_accessor) {
        if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            VarModule::on_flag(agent.object(), vars::ridley::instance::SPECIAL_N_EXPLODE);
        }
        else if !ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            return 0.into()
        }
    }
    let max_charge_frame = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
    let charge_count = WorkModule::get_int(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    let max_fire_num = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_n"), hash40("max_fire_num"));
    let fire_num = ((max_fire_num - 1) * (charge_count / max_charge_frame)) + 1;
    WorkModule::set_int(agent.module_accessor, fire_num, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_FIRE_NUM);
    agent.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());

    return 0.into()
}

unsafe extern "C" fn special_n_charge_substatus(agent: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    }

    return 0.into()
}

unsafe extern "C" fn special_n_shoot_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(agent.object(), vars::ridley::instance::SPECIAL_N_EXPLODE) {
        VarModule::off_flag(agent.object(), vars::ridley::instance::SPECIAL_N_EXPLODE);
        WorkModule::set_int64(agent.module_accessor, hash40("special_n_explode") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        WorkModule::set_int64(agent.module_accessor, hash40("special_air_n_explode") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("special_n_explode"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_n_explode"), 0.0, 1.0, false, 0.0, false, false);
        }
        HIT_NODE(agent, Hash40::new("virtualweakpoint"), *HIT_STATUS_NORMAL);

        agent.sub_shift_status_main(L2CValue::Ptr(special_n_shoot_main_loop as *const () as _))
    }
    else {
        smashline::original_status(Main, agent, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT)(agent)
    }
}

unsafe extern "C" fn special_n_shoot_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(agent.module_accessor) != StatusModule::prev_situation_kind(agent.module_accessor) {
        special_n_situation_helper(agent);
    }
    if MotionModule::is_end(agent.module_accessor) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            agent.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }
    else if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_air_check_fall_common().get_bool()
        || agent.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }

    false.into()
}

unsafe extern "C" fn special_n_situation_helper(agent: &mut L2CFighterCommon) {
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        agent.set_situation(SITUATION_KIND_GROUND.into());
        let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        MotionModule::change_motion_inherit_frame(agent.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        agent.set_situation(SITUATION_KIND_AIR.into());
        let motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion_inherit_frame(agent.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_CHARGE, special_n_charge_main);
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_main);
}