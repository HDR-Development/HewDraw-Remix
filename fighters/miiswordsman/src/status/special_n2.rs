use super::*;

pub unsafe extern "C" fn special_n2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02) - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02) - 1);
    VarModule::on_flag(fighter.battle_object, vars::common::status::CHECK_HOLD_INPUT);

    fighter.main_shift(special_n2_main_loop)
}

pub unsafe extern "C" fn special_n2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            if fighter.is_flag(*FIGHTER_MIISWORDSMAN_STATUS_LIGHT_SYURIKEN_FLAG_FIRST) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n2"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n2"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_MIISWORDSMAN_STATUS_LIGHT_SYURIKEN_FLAG_FIRST);
            }
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if fighter.is_flag(*FIGHTER_MIISWORDSMAN_STATUS_LIGHT_SYURIKEN_FLAG_FIRST) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n2"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n2"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_MIISWORDSMAN_STATUS_LIGHT_SYURIKEN_FLAG_FIRST);
            }
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.check_hold_input(0, 8, Buttons::SpecialAll) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }
    if fighter.motion_frame() >= 27.0 {
        fighter.sub_air_check_dive();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
    }

    return 0.into();
}