use super::*;

pub unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL);
    VarModule::off_flag(fighter.battle_object, vars::miigunner::instance::BOOSTED_AERIAL_LANDING);
    fighter.sub_attack_air();
    fighter.main_shift(attack_air_main_loop)
}

unsafe extern "C" fn attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !sub_attack_air_main(fighter).get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOPPING].get_bool() {
            app::FighterUtil::check_cloud_through_out(fighter.module_accessor);
        }
    }

    // transition to boosted aerial motions
    if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
        // cancel boosted aerial input if not holding the button until the check frame
        if fighter.is_button_release(Buttons::Attack) {
            VarModule::off_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL);
        }
        if VarModule::is_flag(fighter.battle_object, vars::miigunner::status::CHECK_BOOSTED_AERIAL)
        && VarModule::is_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL) {
            VarModule::off_flag(fighter.battle_object, vars::miigunner::status::CHECK_BOOSTED_AERIAL);
            VarModule::off_flag(fighter.battle_object, vars::miigunner::status::BOOSTED_AERIAL);
            VarModule::on_flag(fighter.battle_object, vars::miigunner::instance::BOOSTED_AERIAL_LANDING);
            let motion = if fighter.is_motion(Hash40::new("attack_air_f")) { Hash40::new("attack_air_f_boost") }
            else if fighter.is_motion(Hash40::new("attack_air_b")) { Hash40::new("attack_air_b_boost") }
            else if fighter.is_motion(Hash40::new("attack_air_hi")) { Hash40::new("attack_air_hi_boost") }
            else { Hash40::new("attack_air_lw_boost") };

            let frame = MotionModule::frame(fighter.module_accessor);
            let rate = MotionModule::rate(fighter.module_accessor);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, frame, rate, 0.0, false, false);
        }
    }

    // handle boosted down air charge
    if fighter.is_motion(Hash40::new("attack_air_lw_boost")) {
        let charge = VarModule::get_float(fighter.battle_object, vars::miigunner::status::ATTACK_CHARGE);
        let mut charge_start_frame = 0.0;
        let mut charge_end_frame = 0.0;
        let mut max_charge_frames = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_charge.max_charge_frames");
        charge_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_charge.attack_air_lw_charge_start");
        charge_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_charge.attack_air_lw_charge_end");

        if (charge_start_frame..charge_end_frame).contains(&fighter.motion_frame()) && charge < max_charge_frames
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            let handle = VarModule::get_int64(fighter.battle_object, vars::miigunner::instance::SPECIAL_HI1_LAUNCH_EFFECT_HANDLE);
            EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f::new(0.75 + 0.018 * charge, 0.75 + 0.018 * charge, 0.75 + 0.018 * charge));
            let motion_rate = (charge_end_frame - charge_start_frame) / max_charge_frames;
            MotionModule::set_rate(fighter.module_accessor, motion_rate);
            VarModule::set_float(fighter.battle_object, vars::miigunner::status::ATTACK_CHARGE, charge + 1.0);
        }
        else {
            let handle = VarModule::get_int64(fighter.battle_object, vars::miigunner::instance::SPECIAL_HI1_LAUNCH_EFFECT_HANDLE);
            EffectModule::set_rate(fighter.module_accessor, handle as u32, 1.0);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    }

    return 0.into();
}

unsafe extern "C" fn sub_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.attack_air_common_strans().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("miigunner_atk_shot_after"), false, false);
    if fighter.is_motion(Hash40::new("attack_air_lw_boost"))
    && StatusModule::status_kind_next(fighter.module_accessor) != FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_smash_flash"), false, false);
    }
    
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_end);
}