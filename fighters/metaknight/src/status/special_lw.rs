use super::*;

// FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK

unsafe extern "C" fn special_lw_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // reset vars
    fighter.off_flag(*FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_STOP_X);
    fighter.set_float(0.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_FLOAT_MOVE_DISTANCE);
    fighter.set_float(0.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_FLOAT_RIGHT_EDGE_DISTANCE_X);
    fighter.set_float(0.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_FLOAT_LEFT_EDGE_DISTANCE_X);
    // set default mot
    fighter.set_int64(hash40("special_lw_f") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    fighter.set_int64(hash40("special_air_lw_f") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    fighter.set_int64(hash40("special_lw_f") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
    fighter.set_int64(hash40("special_air_lw_f") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
    FighterSpecializer_Metaknight::check_edge_special_lw(fighter.module_accessor);
    let attack_air_neutral_x = fighter.get_param_float("common", "attack_air_neutral_x");
    let left_stick = fighter.left_stick_x();
    let right_stick = fighter.right_stick_x();
    // calc move dir
    let mut move_stick = if right_stick.abs() > attack_air_neutral_x {right_stick} else {0.0};
    if move_stick.abs() < 1.0 && left_stick.abs() > attack_air_neutral_x {move_stick = left_stick};
    move_stick = (move_stick*100.0).clamp(-1.0, 1.0); // round 
    // calc facing dir (prio left stick then move dir)
    let lr = fighter.lr();
    let mut new_lr = if move_stick.abs() > 0.0 {move_stick} else {-lr};
    if left_stick.abs() > attack_air_neutral_x {new_lr = (left_stick*100.0).clamp(-1.0, 1.0)};
    PostureModule::set_lr(fighter.module_accessor, new_lr);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    // use turnaround anims if input opposite direction of moving direction
    if move_stick*new_lr < 0.0 {
        fighter.set_int64(hash40("special_lw_b") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
        fighter.set_int64(hash40("special_air_lw_b") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        fighter.set_int64(hash40("special_lw_b") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
        fighter.set_int64(hash40("special_air_lw_b") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
    }
    // don't slash past ledge if starting grounded
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if situation == *SITUATION_KIND_GROUND 
    && GroundModule::is_ottotto_lr(fighter.module_accessor, move_stick, 1.5) {
        fighter.on_flag(*FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_STOP_X);
    }
    // add speed on slash, bonus if pivoted
    let mut x_add = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.attack_base_x");
    let mut start_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if new_lr * start_speed < 0.0 
    && move_stick * new_lr < 0.0 {
        x_add += ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_lw.attack_add_x_turn");
    }
    if !fighter.is_flag(*FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_STOP_X) {
        PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f {x: x_add*move_stick, y: 0.0});
    }
    // removed forced airborne state on unaimed?
    fighter.set_int(situation, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_SITUATION_KIND);
    fighter.off_flag(*FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    motion_handling(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_attack_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && (fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        motion_handling(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
    }
    0.into()
}

pub unsafe extern "C" fn motion_handling(fighter: &mut L2CFighterCommon) -> L2CValue { // see if need to add article change situation?
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    fighter.set_int(situation, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_SITUATION_KIND);
    let prev_situation = fighter.get_int(*FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_SITUATION_KIND_PREV);
    let motion = fighter.get_int64(*FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    let mantle = fighter.get_int64(*FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
    let motion_air = fighter.get_int64(*FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    let mantle_air = fighter.get_int64(*FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
    if !fighter.is_flag(*FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
        if situation == *SITUATION_KIND_AIR {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_air), 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle_air), true, -1.0);
        } else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle), true, -1.0);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 2.0, 0.0);
        }
        fighter.on_flag(*FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        return 1.into();
    }
    let frame = fighter.motion_frame(); // cant find article frame cmd?
    if situation == *SITUATION_KIND_AIR {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion_air), -1.0, 1.0, 0.0, false, false);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle_air), true, frame);
    } else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle), true, frame);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 2.0, 0.0);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, special_lw_attack_main);
}