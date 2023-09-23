use super::*;


#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_n_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter){
        return original!(fighter);
    }

    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
    let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as i32;
    let c = charge as f32 / charge_max as f32;
    //let is_max = charge >= charge_max;
    let is_max = false;
    let motion_g = if !is_max {Hash40::new("special_n_i")} else {Hash40::new("special_n_i")};
    let motion_a = if !is_max {Hash40::new("special_air_n_i")} else {Hash40::new("special_air_n_i")};
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {motion_g} else {motion_a};

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART){
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_MOT_RESTART);
    }
    else{
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0,false, false);
    }

    let groundcorrect = if fighter.is_situation(*SITUATION_KIND_GROUND) {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(groundcorrect));

    let kinetic = if fighter.is_situation(*SITUATION_KIND_GROUND) {*FIGHTER_KINETIC_TYPE_GROUND_STOP} else {*FIGHTER_KINETIC_TYPE_AIR_STOP};
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    
    //WorkModule::set_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT,0);
    //Something about slopes?
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_f_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_f_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
        let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")) as i32;
        //let is_max = charge >= charge_max;
        let is_max = false;

        let motion_g = if !is_max {Hash40::new("special_n_i")} else {Hash40::new("special_n_i")};
        let motion_a = if !is_max {Hash40::new("special_air_n_i")} else {Hash40::new("special_air_n_i")};
        fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(),true.into());

        let groundcorrect = if fighter.is_situation(*SITUATION_KIND_GROUND) {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(groundcorrect));
    
        let kinetic = if fighter.is_situation(*SITUATION_KIND_GROUND) {*FIGHTER_KINETIC_TYPE_GROUND_STOP} else {*FIGHTER_KINETIC_TYPE_AIR_STOP};
        KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) {FIGHTER_STATUS_KIND_WAIT}  else {FIGHTER_STATUS_KIND_FALL};
        fighter.change_status(status.into(),false.into());
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    0.into()
}


#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_n_f_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter,Hash40::new("sys_ice"),false,false);
    return original!(fighter);
}

pub fn install() {
    install_status_scripts!(
        special_n_f_main,
        special_n_f_end,
    );
}