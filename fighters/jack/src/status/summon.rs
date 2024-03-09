use super::*;

pub unsafe extern "C" fn jack_summon_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        0,
        0
    );
    0.into()
}

pub unsafe extern "C" fn jack_summon_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if FighterSpecializer_Jack::is_cut_in_effect(fighter.module_accessor) {
    //     WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SUMMON_FLAG_CUT_IN_EFFECT);
    // }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("summon"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if fighter.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    // if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SUMMON_FLAG_CUT_IN_EFFECT) {
    //     MotionAnimcmdModule::flush(fighter.module_accessor, false);
    //     FighterSpecializer_Jack::set_cut_in_effect(fighter.module_accessor);
    // }
    VisibilityModule::set_int64(fighter.module_accessor, hash40("mask") as i64, hash40("on") as i64);
    VisibilityModule::set_material_anim_priority(fighter.module_accessor, Hash40::new("mask"), true);
    fighter.sub_shift_status_main(L2CValue::Ptr(jack_summon_main_loop as *const () as _))
}

unsafe extern "C" fn jack_summon_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_STATUS_KIND_WAIT
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

pub fn install() {
    smashline::Agent::new("jack")
        .status(Pre, *FIGHTER_JACK_STATUS_KIND_SUMMON, jack_summon_pre)
        .status(Main, *FIGHTER_JACK_STATUS_KIND_SUMMON, jack_summon_main)
        .install();
}
