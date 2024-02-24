use super::*;
use globals::*;
 

pub fn install() {
    install_status_scripts!(
        snake_side_special_status_main
    );
}

////side-special tranq gun
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_side_special_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.set_int64(hash40("special_s_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    fighter.set_int64(hash40("special_air_s_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_side_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn special_side_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }
    else {
        change_motion_by_situation(fighter, false);
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return true.into()
            }
        }
    }
    false.into()
}

pub unsafe fn change_motion_by_situation(fighter: &mut L2CFighterCommon, skip_check: bool) {
    if skip_check == false {
        if StatusModule::situation_kind(fighter.module_accessor) == StatusModule::prev_situation_kind(fighter.module_accessor) {
            return
        }
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f::zero());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        let motion = fighter.get_int64(*FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let motion = fighter.get_int64(*FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
}