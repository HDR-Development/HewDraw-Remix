use super::*;

// FIGHTER_STATUS_KIND_ATTACK_AIR

pub unsafe extern "C" fn attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if under USpecial penalty and next status would have been landing, use special landing instead
    let next_status = fighter.global_table[STATUS_KIND].get_i32();
    if VarModule::is_flag(fighter.object(), vars::lucario::instance::SPECIAL_HI_ATTACK_CANCEL) {
        VarModule::off_flag(fighter.object(), vars::lucario::instance::SPECIAL_HI_ATTACK_CANCEL);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::resume_energy_all(fighter.module_accessor);
        if [
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        ].contains(&next_status) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
    }
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter)
}

pub unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    if VarModule::is_flag(fighter.object(), vars::lucario::instance::SPECIAL_HI_ATTACK_CANCEL) {
        // special kinetics
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 0.2, y: 0.2, z: 0.2}, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::suspend_energy_all(fighter.module_accessor);
        // meter cost
        let rate = VarModule::get_float(fighter.battle_object, vars::lucario::instance::SPECIAL_HI_MOTION_RATE);
        MeterModule::drain_direct(fighter.object(), MeterModule::meter_per_level(fighter.object()) - (8.1 * rate));
        opff::check_burnout(fighter);
        opff::pause_meter_regen(fighter, 120);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

    if fighter.is_flag(*FIGHTER_LUCARIO_ATTACK_AIR_STATUS_WORK_ID_FLAG_DEC_SPEED) {
        // get the current DAir speed and decrement it
        let mut attack_air_lw_speed = fighter.get_float(*FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_ATTACK_AIR_LW_SPEED);
        let attack_air_lw_speed_dec = fighter.get_param_float("param_private", "attack_air_lw_speed_dec");
        attack_air_lw_speed = attack_air_lw_speed - attack_air_lw_speed_dec;

        // ensure DAir speed never goes below air_speed_y_stable
        let air_speed_y_stable = -1.0 * fighter.get_param_float("air_speed_y_stable", "");
        if attack_air_lw_speed < air_speed_y_stable {
            attack_air_lw_speed = air_speed_y_stable;
        }

        // update DAir speed
        fighter.set_float(attack_air_lw_speed, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_ATTACK_AIR_LW_SPEED);
        fighter.off_flag(*FIGHTER_LUCARIO_ATTACK_AIR_STATUS_WORK_ID_FLAG_DEC_SPEED);
    }

    if !status_AttackAir_Main_lucario(fighter).get_bool() {
        // idk what this stuff is but it's in common impl
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOPPING].get_bool() {
            app::FighterUtil::check_cloud_through_out(fighter.module_accessor);
        }
    }

    false.into()
}

unsafe extern "C" fn status_AttackAir_Main_lucario(fighter: &mut L2CFighterCommon) -> L2CValue {
    // idk what this is but it's in common impl
    if fighter.attack_air_common_strans().get_bool() {
        return true.into();
    }

    let should_special_fall = VarModule::is_flag(fighter.object(), vars::lucario::instance::SPECIAL_HI_ATTACK_CANCEL) 
        && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD);

    // disallow cancel if under USpecial penalty
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::resume_energy_all(fighter.module_accessor);
        if !should_special_fall {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return true.into();
            }
        }
    }

    // transition to special fall if under USpecial penalty
    if MotionModule::is_end(fighter.module_accessor) {
        if should_special_fall {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into();
    }
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_end);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
}