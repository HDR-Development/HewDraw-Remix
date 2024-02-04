use super::*;


unsafe extern "C" fn trail_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_start").into(), Hash40::new("special_air_s_start").into(), false.into());
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
    WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT);
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_TARGET_ID);

    fighter.sub_shift_status_main(L2CValue::Ptr(trail_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn trail_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    trail_special_s_set_angle_guide(fighter);
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

unsafe extern "C" fn trail_special_s_set_angle_guide(fighter: &mut L2CFighterCommon) {
    let effect = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE) as u32;
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
        angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_FLOAT_TARGET_ANGLE);
    }
    else {
        let mut degrees = vector["y"].get_f32().atan2(vector["x"].get_f32()).to_degrees();
        if degrees < 0.0 { degrees += 360.0 }
        let lr = PostureModule::lr(fighter.module_accessor);
        let max_angle = 30.0;
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
    let guide_pos = trail_special_s_get_guide_pos(fighter, angle.into());
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
        EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_team_color.x, effect_team_color.y, effect_team_color.z);
        WorkModule::set_int(fighter.module_accessor, effect as i32, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE);
    }
}

unsafe extern "C" fn trail_special_s_get_guide_pos(fighter: &mut L2CFighterCommon, angle: L2CValue) -> Vector2f {
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


unsafe extern "C" fn trail_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let effect = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE) as u32;
    if effect != 0 {
        EffectModule::kill(fighter.module_accessor, effect, true, true);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_SEARCH_GUIDE_EFFECT_HANDLE);
    }
    0.into()
}

// FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END


pub unsafe extern "C" fn trail_special_s_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::trail::status::STOP_SIDE_SPECIAL);
    0.into()
}



pub fn install() {
    smashline::Agent::new("trail")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, trail_special_s_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, trail_special_s_end)
        .status(
            End,
            *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END,
            trail_special_s_end_end,
        )
        .install();
}
