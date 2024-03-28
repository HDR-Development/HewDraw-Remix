use super::*;

// FIGHTER_STATUS_KIND_CATCH_PULL
// fixed unwanted buffered throws and walking

unsafe extern "C" fn catch_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_pull"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(catch_pull_main_loop as *const () as _))
    // 0.into()
}

// FIGHTER_STATUS_KIND_CATCH_DASH_PULL

unsafe extern "C" fn catch_dash_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_pull"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(catch_pull_main_loop as *const () as _))
    // 0.into()
}

pub unsafe fn catch_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return true.into()
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.1
        || PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.1{
            VarModule::on_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
        }
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
        return true.into()
    }
    else if MotionModule::frame(fighter.module_accessor) >= 1.0 {
        if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW != 0 {
            fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }
        else if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI != 0 {
            fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        //check stick directly for easier instant f-throw
        }
        else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.7 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }
        }
        else if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B != 0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }
        }
        else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_ATTACK.into(), false.into());
            return true.into()
        }
    }
    return false.into()
}

unsafe extern "C" fn catch_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_attack"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(catch_attack_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn catch_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return true.into()
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
        return true.into()
    }
    else if {let pummel_max_cancel_frame = ParamModule::get_int(fighter.object(), ParamType::Common, "pummel_max_cancel_frame") as f32;
            fighter.global_table[CURRENT_FRAME].get_i32() as f32 + 1.0 >= MotionModule::end_frame(fighter.module_accessor).min(pummel_max_cancel_frame)}
    {
        let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if ControlModule::get_stick_y(fighter.module_accessor) < -0.7 {
            fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }
        else if ControlModule::get_stick_y(fighter.module_accessor) > 0.7 {
            fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()

        }
        else if PostureModule::lr(fighter.module_accessor) * ControlModule::get_stick_x(fighter.module_accessor) > 0.7 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }
            else {
                VarModule::on_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
                return true.into()
            }
        }
        else if PostureModule::lr(fighter.module_accessor) * ControlModule::get_stick_x(fighter.module_accessor) < -0.7 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }
            else{
                VarModule::on_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
                return true.into()
            }
        }
        else if fighter.global_table[PAD_FLAG].get_i32() & (*FIGHTER_PAD_FLAG_ATTACK_TRIGGER | *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) != 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_ATTACK.into(), true.into());
            return true.into()
        }
    }
    return false.into()
}

////added grab walk

unsafe extern "C" fn catch_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    ControlModule::reset_trigger(fighter.module_accessor);
    if VarModule::is_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK) {
        if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.1 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_f"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.1 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_b"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            VarModule::off_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(catch_wait_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn catch_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return true.into()
    }
    else if fighter.is_cat_flag(Cat2::ThrowLw) {
        fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return true.into()
    }
    else if fighter.is_cat_flag(Cat2::ThrowHi) {
        fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return true.into()
    }
    else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.7 {
            fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }
        else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.7 {
            fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_ATTACK.into(), false.into());
            return true.into()
        }
    }
    else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.1 {
        if !VarModule::is_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK) {
            VarModule::on_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_b"), 0.0, 1.0, false, 0.0, false, false);
        }
        let walk_speed:f32 = 1.6*(PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor)*-1.0);
        MotionModule::set_rate(fighter.module_accessor, walk_speed);
    }
    else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.1 {
        if !VarModule::is_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK) {
            VarModule::on_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_f"), 0.0, 1.0, false, 0.0, false, false);
        }
        let walk_speed:f32 = 1.4*(PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor));
        MotionModule::set_rate(fighter.module_accessor, walk_speed);
    }
    else if VarModule::is_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK) {
        VarModule::off_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    return false.into()
}

unsafe extern "C" fn catch_wait_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    VarModule::off_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_CATCH_WAIT)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_PULL, catch_pull_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, catch_dash_pull_main);

    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_main);

    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, catch_wait_main);
    agent.status(End, *FIGHTER_STATUS_KIND_CATCH_WAIT, catch_wait_end);
}