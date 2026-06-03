use super::*;


pub unsafe extern "C" fn special_hi1_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn special_hi1_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
    fighter.sub_change_motion_by_situation(Hash40::new("special_hi1").into(), Hash40::new("special_air_hi1").into(), false.into());
    let landing_frame = fighter.get_param_float("param_special_hi", "hi1_landing_frame");
    fighter.set_float(landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_07) + -1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_07) + -1);

    fighter.main_shift(special_hi1_main_loop)
}

unsafe extern "C" fn special_hi1_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if StatusModule::is_changing(fighter.module_accessor) {
        return 0.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
    }
    special_hi1_charge(fighter);
    // handle actionability
    if fighter.motion_frame() > 46.0 && VarModule::get_float(fighter.battle_object, vars::miigunner::status::ATTACK_CHARGE) <= 10.0 {
        // if already used once this airtime
        if VarModule::is_flag(fighter.battle_object, vars::miigunner::instance::SPECIAL_HI1_AIR_USED) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        }
        else {
            VarModule::on_flag(fighter.battle_object, vars::miigunner::instance::SPECIAL_HI1_AIR_USED);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_hi1_charge(fighter: &mut L2CFighterCommon) {
    let charge = VarModule::get_float(fighter.battle_object, vars::miigunner::status::ATTACK_CHARGE);
    let mut charge_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_charge.special_hi1_charge_start");
    let mut charge_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_charge.special_hi1_charge_end");
    let mut max_charge_frames = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_charge.max_charge_frames");

    if (charge_start_frame..charge_end_frame).contains(&fighter.motion_frame()) && charge < max_charge_frames
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        let motion_vec = if charge <= 10.0 { Vector3f{ x: 1.0, y: 0.55, z: 1.0 } } else { Vector3f{ x: 1.0, y: 0.35, z: 1.0 } };
        KineticModule::mul_speed(fighter.module_accessor, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let handle = VarModule::get_int64(fighter.battle_object, vars::miigunner::instance::SPECIAL_HI1_LAUNCH_EFFECT_HANDLE);
        EffectModule::set_rate(fighter.module_accessor, handle as u32, 1.0/fighter.motion_frame());
        let motion_rate = (charge_end_frame - charge_start_frame)/max_charge_frames;
        MotionModule::set_rate(fighter.module_accessor, motion_rate);
        VarModule::set_float(fighter.battle_object, vars::miigunner::status::ATTACK_CHARGE, charge + 1.0);
    }
    else {
        let handle = VarModule::get_int64(fighter.battle_object, vars::miigunner::instance::SPECIAL_HI1_LAUNCH_EFFECT_HANDLE);
        EffectModule::set_rate(fighter.module_accessor, handle as u32, 1.0);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
}

pub unsafe extern "C" fn special_hi1_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("miigunner_bottom_shot"), false, false);
    return 0.into();
}