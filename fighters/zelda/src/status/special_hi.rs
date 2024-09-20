use super::*;

//pub unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
//    create_arrow_eff(fighter);
//    0.into()
//}
//
//unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
//    let effect = VarModule::get_int(fighter.battle_object, vars::common::status::WARP_EFF_HANDLER) as u32;
//    if effect != 0 {
//        EffectModule::kill(fighter.module_accessor, effect, true, true);
//        VarModule::set_int(fighter.battle_object, vars::common::status::WARP_EFF_HANDLER, 0);
//    }
//    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
//}

unsafe extern "C" fn special_hi_2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3 {
        //re-uses waveland window logic
        let init_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_Y); //teleport direction
        let pos = *PostureModule::pos(fighter.module_accessor); //top bone (bottom of ecb w/o shifting)
        let bot_snap = &Vector2f::new(pos.x, pos.y - 1.0);
        let top_snap = &Vector2f::new(pos.x, pos.y + 11.0);//around chest level
        let ground_pos_any = &mut Vector2f::zero();
        let ground_pos_stage = &mut Vector2f::zero();
        let is_touch_any = GroundModule::line_segment_check(fighter.module_accessor, top_snap, bot_snap, &Vector2f::zero(), ground_pos_any, true);
        let is_touch_stage = GroundModule::line_segment_check(fighter.module_accessor, top_snap, bot_snap, &Vector2f::zero(), ground_pos_stage, false);
        let can_snap = !(is_touch_any == 0 as *const *const u64 || (is_touch_stage != 0 as *const *const u64 && init_speed_y > 0.0)); //avoid snapping to stage from below
        if can_snap {
            PostureModule::set_pos(fighter.module_accessor, &Vector3f::new(pos.x, ground_pos_any.y + 0.1, pos.z));
            GroundModule::attach_ground(fighter.module_accessor, false);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
            PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
                let lr = PostureModule::lr(fighter.module_accessor);
                let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                let speed = Vector2f{ x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x.abs() * lr * 1.05, speed.y); //b-reverse telecancel reverses momentum on ground
            }
        }
    }
    0.into()
}

unsafe extern "C" fn special_hi_3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            MotionModule::change_motion(fighter.module_accessor, "landing_fall_special".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        } else {
            MotionModule::change_motion(fighter.module_accessor, "special_hi".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        }
    } else {
        GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_int(*SITUATION_KIND_AIR, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, "special_air_hi".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        let landing_lag = fighter.get_param_float("param_special_hi", "landing_frame");
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            WorkModule::set_float(fighter.module_accessor, 17.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME); //half lag if cancel
        } else {
            WorkModule::set_float(fighter.module_accessor, landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
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
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into())
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into())
        } else {
            //clear buffer
            ControlModule::reset_trigger(fighter.module_accessor);
            ControlModule::clear_command(fighter.module_accessor, true);
            ControlModule::reset_special_command(fighter.module_accessor, true);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into())
        }
    }
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        if fighter.is_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_DIVE) {
            fighter.sub_air_check_dive();
        }  //bypass manual fastfall
        if fighter.is_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CONTROL) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            fighter.off_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CONTROL);
        } //bypass manual drift code
        if !fighter.global_table[IS_STOPPING].get_bool() && KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_MOTION_FALL { //only runs t he capping stuff before she can drift and fastfall
            if !fighter.is_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_1) {
                let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                let speed = Vector2f{x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};//pretty sure code subtracts 1/10th of y speed every frame before normal fall is enabled
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x, speed.y *0.9);
            } else {
                if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
                    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                    let speed_y = lua_bind::KineticEnergy::get_speed_y(stop_energy);
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                } //only enables gravity once instead of every frame
                let air_speed_x_stable: f32 = fighter.get_param_float("air_speed_x_stable", "");
                let fall_x_mul: f32 = fighter.get_param_float("param_special_hi", "fall_x_mull_value");
                let mut x_cap = air_speed_x_stable * fall_x_mul;
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, x_cap, 0.0);
            } //bypass once per frame set_speed to x_cap
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, special_hi_2_end);
    agent.status(Main, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, special_hi_3_main);
}