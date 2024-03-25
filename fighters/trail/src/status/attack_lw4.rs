use super::*;

// lets down smash travel past ledges during a DACUS
pub unsafe extern "C" fn attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    fighter.attack_lw4_mtrans();
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.status_ThrowKirby_Uniq(L2CValue::Bool(false));
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_status_ThrowKirby_Uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(trail_attack_lw4_main_loop as *const () as _))
}

pub unsafe extern "C" fn trail_attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    && !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND)
    && !MotionModule::is_end(fighter.module_accessor) {
        fighter.sub_status_uniq_process_ThrowKirby_execFixPos();
        return 0.into()
    }
    fighter.status_AttackLw4_Main()
}

pub unsafe extern "C" fn attack_lw4_map_correction(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor) as i32;

    // animation startup
    if frame < 6 {
        return 0.into()
    }
    // first frame of being airborne
    if frame == 6 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
        VarModule::set_float(fighter.battle_object, vars::trail::status::DACUS_SPEED_Y, -2.8); // initial speed for sora to start falling
    }
    // window in which sora will accel downwards 
    if frame == (18 | 19)
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        let speed_y = VarModule::get_float(fighter.battle_object, vars::trail::status::DACUS_SPEED_Y);
        let accel_mul = 1.04; // rate in which the decent will accelerate each frame
        let new_speed = speed_y * accel_mul;
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, new_speed);
        sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.05); // level of horizontal control while falling
        sv_kinetic_energy!(set_accel_x_add, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.05);
        VarModule::set_float(fighter.battle_object, vars::trail::status::DACUS_SPEED_Y, new_speed);
        if frame == 19 { // freeze the animation
            MotionModule::set_rate(fighter.module_accessor, 0.0);
        }
    }
    // ray check to see if sora is close enough to the ground to finish the animation
    let should_land = 
        GroundModule::ray_check(fighter.module_accessor, 
                                &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, 
                                &Vector2f{ x: 0.0, y: -6.0}, true) == 1;
    if frame == 19 {
        if should_land {
            //println!("landing!");
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        } else {
            let fall_frame = VarModule::get_int(fighter.battle_object, vars::trail::status::ATTACK_LW4_TIMER);
            if fall_frame < 17 {
                VarModule::inc_int(fighter.battle_object, vars::trail::status::ATTACK_LW4_TIMER);
            } else {
                VarModule::set_int(fighter.battle_object, vars::trail::status::ATTACK_LW4_TIMER, 0);
                KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 1.into();
            }
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, attack_lw4_main);
    agent.status(MapCorrection, *FIGHTER_STATUS_KIND_ATTACK_LW4, attack_lw4_map_correction);
}
