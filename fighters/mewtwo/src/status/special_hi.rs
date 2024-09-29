use super::*;

unsafe extern "C" fn special_hi_3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, "special_hi".to_hash(), 0.0, 1.0, false, 0.0, false, false);
    } else {
        GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_int(*SITUATION_KIND_AIR, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, "special_air_hi".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        let landing_lag = fighter.get_param_float("param_special_hi", "landing_frame");
        //freefall ver's special lag
        if VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::UP_SPECIAL_FREEFALL) {
            WorkModule::set_float(fighter.module_accessor, landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME); 
        } //lag if not cancellable (also done in end status ig)
        //special fall speed
        let x_max = fighter.get_param_float("param_special_hi", "fall_x_mull_value");
        WorkModule::set_float(fighter.module_accessor, x_max, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    }
    
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_3_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let control = ControlModule::get_attack_air_kind(fighter.module_accessor);
    if control == *FIGHTER_COMMAND_ATTACK_AIR_KIND_NONE {
        FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
    } //?
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
          return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into())
        } else {
            if !VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::TELEPORT_CANCEL) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into())
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into())
            }
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if !VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::TELEPORT_CANCEL) {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into())
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), false.into())
            } //re-adds nil from p+?
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into())
        }
    }
    //momentum and fastfall
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_1) { //drift enable frame
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            fighter.off_flag(*FIGHTER_MEWTWO_STATUS_SPECIAL_HI_FLAG_1); 
        } else {
            if fighter.status_frame() < 7 { //should run 8 times frames like pm?
                let stop_energy: *mut KineticEnergy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                let speed = Vector2f{x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x, speed.y *0.9);
            } else if fighter.status_frame() == 7 {
                let stop_energy: *mut KineticEnergy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                let speed = Vector2f{x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x, 0.0);
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed.y);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn special_hi_2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, special_hi_3_main);
}