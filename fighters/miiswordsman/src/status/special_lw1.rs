use super::*;

pub unsafe extern "C" fn special_lw1_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn special_lw1_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw1"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw1"), 0.0, 1.0, false, 0.0, false, false);
    }
    special_lw1_set_physics(fighter, 0);

    fighter.main_shift(special_lw1_main_loop)
}

unsafe extern "C" fn special_lw1_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        special_lw1_change_motion(fighter);
    }
    if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
        fighter.sub_air_check_dive();
    }
    if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW1_CHECK_INPUT) {
        if fighter.is_cat_flag(Cat1::AttackLw3 | Cat1::AttackLw4) {
            VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW1_CHECK_INPUT);
            fighter.change_motion_by_situation("special_lw1_mordschlag", "special_air_lw1_mordschlag", 0.0, 1.0, false, 0.0, false, false);
            special_lw1_set_physics(fighter, 2);
        }
        else if fighter.is_cat_flag(Cat1::SpecialLw) {
            VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW1_CHECK_INPUT);
            fighter.change_motion_by_situation("special_lw1_flourish", "special_air_lw1_flourish", 0.0, 1.0, false, 0.0, false, false);
            special_lw1_set_physics(fighter, 1);
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW1_CHANGE_KINETIC) {
        VarModule::off_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW1_CHANGE_KINETIC);
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
    
    return 0.into();
}

unsafe fn special_lw1_set_physics(fighter: &mut L2CFighterCommon, attack_type: i32) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) { return; }
    if attack_type == 0 {
        // pommel
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        let mut speed_x_mul = 0.8;  // parameterize
        if !VarModule::is_flag(fighter.battle_object, vars::common::instance::SPECIAL_STALL_USED) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::SPECIAL_STALL_USED);
            speed_y = 0.5;  // parameterize
            speed_x_mul = 0.4; // parameterize
        }
        let air_accel_y = fighter.get_param_float("air_accel_y", "");
        let air_speed_y_stable = fighter.get_param_float("air_speed_y_stable", "");
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y * 0.8);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, sum_speed_x * speed_x_mul, 0.0, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else if attack_type == 1 {
        // flourish
        //sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    }
    else {
        // mordschlag
        let air_accel_y = fighter.get_param_float("air_accel_y", "");
        let air_speed_y_stable = fighter.get_param_float("air_speed_y_stable", "");
        //sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.1);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y * 0.8);  // parameterize
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable * 0.8);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
    }
}

unsafe fn special_lw1_change_motion(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        let motion = if fighter.is_motion(Hash40::new("special_air_lw1_flourish")) { Hash40::new("special_lw1_flourish") }
        else if fighter.is_motion(Hash40::new("special_air_lw1_mordschlag")) { Hash40::new("special_lw1_mordschlag") }
        else { Hash40::new("special_lw1") };
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
    }
    else {
        let motion = if fighter.is_motion(Hash40::new("special_lw1_flourish")) { Hash40::new("special_air_lw1_flourish") }
        else if fighter.is_motion(Hash40::new("special_lw1_mordschlag")) { Hash40::new("special_air_lw1_mordschlag") }
        else { Hash40::new("special_air_lw1") };
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
    }
}

pub unsafe extern "C" fn special_lw1_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn special_lw1_check_attack(fighter: &mut L2CFighterCommon, param_1: &L2CValue, param_2: &L2CValue) -> L2CValue {
    if fighter.is_motion(Hash40::new("special_air_lw1")) {
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let add_speed = 1.3;    // parameterize
        KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(0.0, add_speed, 0.0));
    }

    return 0.into();
}