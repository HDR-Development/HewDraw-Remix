use super::*;


// FIGHTER_STATUS_KIND_SPECIAL_S

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_start").into(), Hash40::new("special_air_s_start").into(), false.into());
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
    VarModule::on_flag(fighter.battle_object, vars::common::status::ENABLE_SPECIAL_WALLJUMP);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let effect = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE);
    special_s_set_angle_guide(fighter, effect.into(), FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE.into());
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_start").into(), Hash40::new("special_air_s_start").into(), true.into());
        fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
    }
    0.into()
}

unsafe extern "C" fn special_s_set_angle_guide(fighter: &mut L2CFighterCommon, eff_handle: L2CValue, angle_const: L2CValue) {
    let effect = eff_handle.get_u32();
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let vector = fighter.Vector2__create(stick_x.into(), stick_y.into());
    let length = fighter.Vector2__length(vector.clone());
    let threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("search_stick"));
    let angle;
    if length.get_f32() < threshold {
        if effect == 0 {
            return;
        }
        angle = WorkModule::get_float(fighter.module_accessor, angle_const.get_i32());
    }
    else {
        let mut degrees = vector["y"].get_f32().atan2(vector["x"].get_f32()).to_degrees();
        if degrees < 0.0 { degrees += 360.0 }
        let lr = PostureModule::lr(fighter.module_accessor);
        let max_angle = 35.0;
        if lr >= 0.0 {
            if degrees <= 180.0 && degrees > max_angle {
                degrees = max_angle;
            }
            else if degrees > 180.0 && degrees < 360.0 - max_angle {
                degrees = 360.0 - max_angle;
            }
        }
        else {
            degrees = degrees.clamp(180.0 - max_angle, 180.0 + max_angle);
        }
        angle = degrees;
        WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_STICK);
    }
    let guide_pos = special_s_get_guide_pos(fighter, angle.into());
    if effect != 0 {
        EffectModule::set_pos(fighter.module_accessor, effect, &Vector3f{x: guide_pos.x, y: guide_pos.y, z: 0.0});
        EffectModule::set_rot(fighter.module_accessor, effect, &Vector3f{x: 0.0, y: 0.0, z: angle - 90.0});
    }
    else {
        let effect = EffectModule::req(
            fighter.module_accessor,
            Hash40::new("sys_direction2"),
            &Vector3f{x: guide_pos.x, y: guide_pos.y, z: 0.0},
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            1.0,
            0,
            -1,
            false,
            0
        ) as u32;
        EffectModule::set_rot(fighter.module_accessor, effect, &Vector3f{x: 0.0, y: 0.0, z: angle - 90.0});
        let team_color = FighterUtil::get_team_color(fighter.module_accessor);
        let effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
        EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_team_color.x(), effect_team_color.y(), effect_team_color.z());
        WorkModule::set_int(fighter.module_accessor, effect as i32, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE);
    }
}

unsafe extern "C" fn special_s_get_guide_pos(fighter: &mut L2CFighterCommon, angle: L2CValue) -> Vector2f {
    let pos = PostureModule::pos(fighter.module_accessor);
    let rad = angle.get_f32().to_radians();
    let scale = PostureModule::scale(fighter.module_accessor);
    let dist = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("search_cursor_dist"));
    let dist_scaled = dist * scale;
    let x_pos = rad.cos() * dist_scaled + (*pos).x;
    let y_pos = rad.sin() * dist_scaled + (*pos).y;
    let y_offset = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("search_cursor_offset_y"));
    let y_pos = y_offset * scale + y_pos;
    Vector2f{x: x_pos, y: y_pos}
}

pub unsafe extern "C" fn special_s_set_joint_rotate(fighter: &mut L2CFighterCommon, angle: L2CValue, param_3: L2CValue) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut angle = angle.get_f32();

    if lr == -1.0 {
        angle = 180.0 - angle;
    }

    if angle > 180.0 {
        angle = angle - 360.0;
    }

    if param_3.get_f32() < 1.0 {
        angle = fighter.lerp(0.0_f32.into(), angle.into(), param_3).get_f32();
    }

    ModelModule::set_joint_rotate(
        fighter.module_accessor,
        Hash40::new("rot"),
        &Vector3f{x: -angle, y: 0.0, z: 0.0},
        MotionNodeRotateCompose { _address: 0 },
        MotionNodeRotateOrder { _address: 0 }
    );
}

pub unsafe extern "C" fn special_s_set_cursor_on_posture(fighter: &mut L2CFighterCommon) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR_ON_POSTURE);
    let cursor_offset_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("cursor_offset_y"));
    let scale = PostureModule::scale(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, cursor_offset_y * scale, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CURSOR_OFFSET_Y);
}

pub unsafe extern "C" fn special_s_reset_angle(fighter: &mut L2CFighterCommon) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let angle = if lr == -1.0 {
        180.0
    }
    else {
        0.0
    };
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
}

unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let effect = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE) as u32;
    if effect != 0 {
        EffectModule::kill(fighter.module_accessor, effect, true, true);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE);
    }
    0.into()
}

// FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK

unsafe extern "C" fn special_s_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let touch_down = GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let attack_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT);
    let to_search = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH);
    WorkModule::set_flag(fighter.module_accessor, to_search, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH_PREV);
    WorkModule::set_flag(fighter.module_accessor, touch_down, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TOUCH_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH);

    let attack_button = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH_PREV)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_STICK);
    WorkModule::set_flag(fighter.module_accessor, attack_button, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_ATTACK_BUTTON);

    special_s_attack_set_motion_multiple(fighter, attack_count.into(), false.into(), touch_down.into());
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    GroundModule::set_passable_check(fighter.module_accessor, true);
    let target_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_TARGET_ID) as u32;
    let mut speed = special_s_attack_get_speed(fighter).get_f32();
    let mut speed_vec = fighter.Vector2__create(speed.into(), 0.0_f32.into());
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut new_lr = lr;

    let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
    let attack_up_angle_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_up_angle_min"));
    let attack_up_angle_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_up_angle_max"));
    if angle >= attack_up_angle_min && angle <= attack_up_angle_max {
        let attack_up_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_up_speed_mul"));
        speed *= attack_up_speed_mul;
    }

    let rad = angle.to_radians();
    let cos = rad.cos();
    speed_vec["x"].assign(&L2CValue::F32(cos * speed));
    let sin = rad.sin();
    speed_vec["y"].assign(&L2CValue::F32(sin * speed));

    if !special_s_attack_check_angle_multiple(fighter, angle.into()).get_bool() {
        let speed_x = speed_vec["x"].get_f32();
        if speed_x < 0.0 {
            new_lr = -1.0;
        }
        else if speed_x > 0.0 {
            new_lr = 1.0;
        }
    }

    if lr != new_lr {
        PostureModule::set_lr(fighter.module_accessor, new_lr);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_vec["x"].get_f32(),
        speed_vec["y"].get_f32()
    );
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        -1.0,
        -1.0
    );

    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.0
    );

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.0
    );

    let attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("attack_frame"));
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    let rate = (end_frame / attack_frame as f32) + 0.01;
    MotionModule::set_rate(fighter.module_accessor, rate);

    special_s_set_cursor_on_posture(fighter);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_attack_main_loop as *const () as _))
}


unsafe extern "C" fn special_s_attack_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        let correct = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_CLIFF_STOP) {
            *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP
        }
        else {
            *GROUND_CORRECT_KIND_GROUND
        };
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
}

unsafe extern "C" fn special_s_attack_set_speed(fighter: &mut L2CFighterCommon) {
    let speed = special_s_attack_get_speed(fighter).get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed * lr,
        0.0
    );
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        -1.0,
        0.0
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.0
    );
}

unsafe extern "C" fn special_s_attack_get_speed(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut attack_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_speed_x"));
    let attack_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT);
    if 0 < attack_count {
        let attack_reduction_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x15530d2d10);
        for _ in 0..attack_count {
            attack_speed_x *= attack_reduction_mul;
        }
    }
    let hit_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_HIT_NUM);
    if 0 < hit_num {
        let hit_reduction_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x17dd304b6f);
        for _ in 0..hit_num {
            attack_speed_x *= hit_reduction_mul;
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_ATTACK_BUTTON) {
        let button_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x1a41a10288);
        attack_speed_x *= button_speed_mul;
    }

    attack_speed_x.into()
}

unsafe extern "C" fn special_s_attack_set_motion_multiple(
    fighter: &mut L2CFighterCommon,
    attack_count: L2CValue,
    is_inherit: L2CValue,
    touch_ground: L2CValue
) {
    let motion = if attack_count.get_i32() == 0 {
        if touch_ground.get_bool() {
            "special_s_1"
        }
        else {
            "special_air_s_1"
        }
    }
    else if attack_count.get_i32() == 1 {
        if touch_ground.get_bool() {
            "special_s_2"
        }
        else {
            "special_air_s_2"
        }
    }
    else {
        if touch_ground.get_bool() {
            "special_s_3"
        }
        else {
            "special_air_s_3"
        }
    };

    if is_inherit.get_bool() {
        let frame = MotionModule::frame(fighter.module_accessor);
        let rate = MotionModule::rate(fighter.module_accessor);
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new(motion),
            frame,
            rate,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new(motion),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
}

unsafe extern "C" fn special_s_attack_check_angle_multiple(fighter: &mut L2CFighterCommon, angle: L2CValue) -> L2CValue {
    let some_angle = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0xdeb5675e2);
    let angle_adjust_1 = angle.get_f32() - 90.0;
    let angle_adjust_2 = angle.get_f32() - 270.0;
    if angle_adjust_1.abs() > some_angle
    && angle_adjust_2.abs() > some_angle {
        return false.into();
    }

    true.into()
}

unsafe extern "C" fn special_s_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }

    // Reduce speed on shield
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_PARRY)
    && !fighter.is_in_hitlag() {
        let shield_hit_end_speed_x = if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.shield_hit_ground_end_speed_x")
        } else {
            ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.shield_hit_air_end_speed_x")
        };
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            shield_hit_end_speed_x * lr,
            0.0
        );
        fighter.change_status(FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }

    let attack_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT);
    // let to_search = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH);
    // if !to_search {
    //     if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_SEARCH_BUTTON) {
    //         if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
    //             WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH);
    //         }
    //     }
    // }

    if MotionModule::is_end(fighter.module_accessor) {
        let attack_num = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("attack_num"));
        if attack_count < attack_num - 1 {
            let mut status = FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END;
            let mut clear_buffer = false;
            let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
            let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
            let stick_vec = fighter.Vector2__create(stick_x.into(), stick_y.into());
            let length = fighter.Vector2__length(stick_vec);
            let search_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("search_stick"));
            if length.get_f32() < search_stick {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TO_SEARCH) {
                    status = FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH;
                    clear_buffer = true;
                    if attack_count == 0 {
                        special_s_reset_angle(fighter);
                    }
                }
            }
            else {
                if attack_count == 0 {
                    special_s_reset_angle(fighter);
                }
                status = FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH;
                clear_buffer = true;
            }
            fighter.change_status(status.into(), clear_buffer.into());
        }
        else {
            fighter.change_status(FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        }
        return 0.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor) {
        let flags = GroundModule::get_touch_flag(fighter.module_accessor) as u32;
        if flags & *GROUND_TOUCH_FLAG_DOWN as u32 != 0 {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TOUCH_GROUND) {
                if flags & (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_UP | *GROUND_TOUCH_FLAG_RIGHT) as u32 != 0 {
                    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_TOUCH_GROUND_FRAME);
                }
                else {
                    if !fighter.global_table[IS_STOPPING].get_bool() {
                        let ground_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_TOUCH_GROUND_FRAME);
                        let ground_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), 0x1b949b05bc);
                        if ground_frame < ground_frame_max {
                            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_TOUCH_GROUND_FRAME);
                        }
                        else {
                            fighter.change_status(FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END.into(), false.into());
                            return 0.into();
                        }
                    }
                }
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TOUCH_GROUND);
        }
        else {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_TOUCH_GROUND_FRAME);
        }
    }

    let attack_num = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("attack_num"));
    if attack_count < attack_num - 1 {
        let handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE);
        if fighter.global_table[IS_STOPPING].get_bool() {
            if handle != 0 {
                let guide_angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_GUIDE_EFFECT_ANGLE_ATTACK);
                let guide_pos = special_s_get_guide_pos(fighter, guide_angle.into());
                EffectModule::set_pos(fighter.module_accessor, handle as u32, &Vector3f{x: guide_pos.x, y: guide_pos.y, z: 0.0});
            }
        }
        else {
            special_s_set_angle_guide(fighter, handle.into(), FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_GUIDE_EFFECT_ANGLE_ATTACK.into());
        }
    }

    let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
    special_s_set_joint_rotate(fighter, angle.into(), 1.0_f32.into());

    0.into()
}

// FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END

pub unsafe extern "C" fn special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Sora is counted as being airborne when the move is performed horizontally along the ground
    // If `FIGHTER_TRAIL_STATUS_SPECIAL_S_FLAG_TOUCH_GROUND` is checked instead, it will work properly there but then
    //  starting the move on the ground and ending in the air screws up his air physics. Ergo, we're using a ray check
    //  to force him to use his proper grounded landing animation if he is ending right above the ground (and not rising)
    if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) <= 0.0
    && GroundModule::ray_check(
        fighter.module_accessor, 
        &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, 
        &Vector2f{ x: 0.0, y: -2.0}, true
    ) == 1 {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
    }
    special_s_search_end_set_kinetic(fighter);
    special_s_search_end_set_speed(fighter);
    fighter.sub_change_motion_by_situation(
        Hash40::new("special_s_end").into(),
        Hash40::new("special_air_s_end").into(),
        false.into()
    );

    let attack_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT);
    let param = match attack_count {
        1 => "end_frame_1",
        2 => "end_frame_2",
        _ => "end_frame_3"
    };
    let end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40(param)) as f32;
    let motion_end_frame = MotionModule::end_frame(fighter.module_accessor);
    MotionModule::set_rate(fighter.module_accessor, motion_end_frame / end_frame + 0.01);

    let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_BACK_ANGLE);

    special_s_set_cursor_on_posture(fighter);

    fighter.main_shift(special_s_end_main_loop)
}

unsafe extern "C" fn special_s_search_end_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        // Despite what this would have you believe, somehow the cliff stop doesn't stop at the damn cliff so enjoy your edge cancels
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
}

unsafe extern "C" fn special_s_search_end_set_speed(fighter: &mut L2CFighterCommon) {
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        -1.0,
        0.0
    );
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("end_brake_x"));
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            end_brake_x,
            0.0
        );
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    }
    else {
        let end_brake_x_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("end_brake_x_air"));
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            end_brake_x_air,
            0.0
        );
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        let end_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("end_speed_y"));
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y.clamp(-end_speed_y, end_speed_y)
        );
        let end_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("end_accel_y"));
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -end_accel_y
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -1.0
        );

        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let end_speed_x_mul_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("end_speed_x_mul_air"));
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * end_speed_x_mul_air,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

pub unsafe extern "C" fn special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    fighter.check_wall_jump_cancel();
    if fighter.status_frame() > 10 {
        fighter.sub_air_check_dive();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            let end_landing_fall_special_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("end_landing_fall_special_frame"));
            let frame = MotionModule::frame(fighter.module_accessor);
            if frame >= end_landing_fall_special_frame as f32 {
                // let attack_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("attack_landing_frame"));
                // WorkModule::set_float(fighter.module_accessor, attack_landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                VarModule::set_float(fighter.battle_object, vars::common::instance::LAND_CANCEL_LAG, 6.0);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 0.into();
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            // let attack_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("attack_landing_frame"));
            // WorkModule::set_float(fighter.module_accessor, attack_landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            // fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 0.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_motion_inherit_frame_by_situation("special_s_end", "special_air_s_end", -1.0, 1.0, 0.0, false, false);
        special_s_search_end_set_kinetic(fighter);
        special_s_search_end_set_speed(fighter);
    }

    0.into()
}

unsafe extern "C" fn special_s_end_joint_rotate(fighter: &mut L2CFighterCommon) {
    let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_BACK_ANGLE);
    if 0.0 < angle {
        let frame = fighter.global_table[CURRENT_FRAME].get_f32();
        let right_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("right_frame")) as f32;
        if frame < right_frame {
            let lerp = 1.0 - (frame / right_frame);
            special_s_set_joint_rotate(fighter, angle.into(), lerp.into());
        }
    }
}

pub unsafe extern "C" fn special_s_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);

    agent.status(Main, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK, special_s_attack_main);

    agent.status(Main, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END, special_s_end_main);
    agent.status(End, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END, special_s_end_end);
}