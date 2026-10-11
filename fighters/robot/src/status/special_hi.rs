use super::*;

use vars::robot::{
    instance::*,
    status::*
};

// FIGHTER_STATUS_KIND_SPECIAL_HI

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        false, // now lets people grab rob during the startup
        false,
        0,
        (*FIGHTER_STATUS_ATTR_INTO_DOOR | *FIGHTER_STATUS_ATTR_START_TURN) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // reset variables once
    VarModule::off_flag(fighter.battle_object, SPECIAL_HI_GROUND_START);
    VarModule::set_int(fighter.battle_object, SPECIAL_HI_CHARGE_FRAME, 0);
    VarModule::set_float(fighter.battle_object, SPECIAL_HI_ROT_X, 0.0);

    if fighter.is_situation(*SITUATION_KIND_AIR) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
    }

    let start_mul_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_mul_y");
    let start_stable_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_stable_y");
    let start_mul_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_mul_x");
    let start_fly_stable_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_fly_stable_x");
    let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let y_speed = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);

    let prev_damage = fighter.is_prev_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL
    ]);

    // limit horizontal speed
    if !prev_damage {
        sv_kinetic_energy!(set_accel_x_add, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
    } else {
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, start_fly_stable_x, 0.0);
    }
    // reset speed
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, x_speed * start_mul_x, 0.0);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, y_speed * start_mul_y);
    // limit gravity
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);

    fighter.main_shift(special_hi_main_loop)
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // charge double speed on ground
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        VarModule::add_int(fighter.battle_object, SPECIAL_HI_CHARGE_FRAME, 2);
    } else {
        VarModule::inc_int(fighter.battle_object, SPECIAL_HI_CHARGE_FRAME);
    }

    let charge_frame = VarModule::get_int(fighter.battle_object, SPECIAL_HI_CHARGE_FRAME) as f32;
    let charge_frame_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.charge_frame_max");

    // defines fuel consumption throughout the move
    let start_fuel = fighter.get_float(*FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
    let max_fuel = fighter.get_param_float("param_special_hi", "energy_max_frame");
    let launch_fuel_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.launch_fuel_mul"); // fuel per frame of charge
    let launch_fuel_min = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.launch_fuel_min"); // min fuel to launch
    let required_fuel = (launch_fuel_mul * charge_frame).clamp(launch_fuel_min, max_fuel);
    let remaining_fuel = (start_fuel - required_fuel).clamp(0.0, max_fuel);

    // no charge launchless variant if fuel below min thresh
    if launch_fuel_min > start_fuel {
        VarModule::set_int(fighter.battle_object, SPECIAL_HI_CHARGE_FRAME, 0);
    }

    // end stall at frame x of charge
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        let start_stop_y_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.start_stop_y_frame");
        if fighter.global_table[CURRENT_FRAME].get_i32() == start_stop_y_frame {
            let start_accel_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_accel_y");
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -start_accel_y); // 1/4 base accel
            let start_stable_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_stable_y");
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, start_stable_y);
        }
    }

    // calculates angle of move
    let mut rot = VarModule::get_float(fighter.battle_object, SPECIAL_HI_ROT_X);
    let charge_angle_air = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.charge_angle_air");
    let charge_angle_ground = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.charge_angle_ground");
    let max_launch_angle = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.max_launch_angle");
    let rot_amount = if fighter.is_situation(*SITUATION_KIND_AIR) {charge_angle_air} else {charge_angle_ground}; // how much rob rotates each frame
    let mut stick_x = fighter.left_stick_x();
    let mut stick_y = fighter.left_stick_y();
    let mut length = sv_math::vec2_length(stick_x, stick_y);
    let wrap_stick = 0.5;
    // don't change angle if stick is in teleport angling deadzone
    let mut angle = rot * -1.0 + 90.0;
    if length >= wrap_stick {
        // if holding down, convert to max horizontal angle
        if stick_y < 0.0 {
            stick_x = (stick_x * 100.0).clamp(-1.0, 1.0);
            stick_y = 0.0;
        }
        angle = stick_y.atan2(stick_x).to_degrees().clamp(90.0 - max_launch_angle, 90.0 + max_launch_angle);
    }
    // calc new rot based on old rot and new angle
    let new_rot = (angle - 90.0) * -1.0; // convert to offset from 90 deg
    rot = new_rot.clamp(rot - rot_amount, rot + rot_amount);

    // flips if rotation crosses center threshold
    let old_lr = fighter.lr();
    let lr_rot = if rot.abs() > 0.0 {
        (rot * 100.0).clamp(-1.0, 1.0)
    } else {
        old_lr
    };
    if (lr_rot + old_lr) < 1.0 {
        PostureModule::set_lr(fighter.module_accessor, lr_rot);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    // angle body ody
    PostureModule::set_rot(fighter.module_accessor, &Vector3f::new(rot * 0.3 * lr_rot, 0.0, 0.0), 0);
    VarModule::set_float(fighter.battle_object, SPECIAL_HI_ROT_X, rot);

    // summon guide effect
    special_hi_guide_handler(fighter);

    // launch if full charge, or past minimum change and ineligible to continue charging
    let fuel_depleted = required_fuel >= start_fuel;
    if charge_frame >= charge_frame_max
    || ((fighter.status_frame() + 1) >= 8 // -2
    && (fuel_depleted || fighter.is_button_off(Buttons::Special)))
    {
        fighter.set_float(remaining_fuel, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            VarModule::on_flag(fighter.battle_object, SPECIAL_HI_GROUND_START);
        }
        fighter.change_status(FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP.into(), true.into());
        return 1.into();
    }
    return 0.into();
}

unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_ROBOT_STATUS_BURNER_FLAG_TRANSFORM_COMP);
    PostureModule::set_rot(fighter.module_accessor, &Vector3f::zero(), 0);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("robot_lamp_l"), true, true);
    let eff_handle = VarModule::get_int(fighter.battle_object, SPECIAL_HI_MARKER_EFFECT_HANDLE) as u32;
    if EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
        EffectModule::kill(fighter.module_accessor, eff_handle, true, true);
        VarModule::set_int(fighter.battle_object, SPECIAL_HI_MARKER_EFFECT_HANDLE, 0);
    }

    return 0.into();
}

unsafe extern "C" fn arrow_guide_pos(fighter: &mut L2CFighterCommon, angle: L2CValue) -> Vector2f {
    let pos = PostureModule::pos(fighter.module_accessor);
    let rad = angle.get_f32().to_radians();
    let scale = PostureModule::scale(fighter.module_accessor);
    let dist = 20.0;
    let dist_scaled = dist * scale;
    let x_pos = rad.cos() * dist_scaled + (*pos).x;
    let y_pos = rad.sin() * dist_scaled + (*pos).y;
    let y_offset = 6.0;
    let y_pos = y_offset * scale + y_pos;
    Vector2f {
        x: x_pos,
        y: y_pos,
    }
}

pub unsafe fn special_hi_guide_handler(fighter: &mut L2CFighterCommon) {
    // thanks wuboy <3
    let mut angle = (VarModule::get_float(fighter.battle_object, SPECIAL_HI_ROT_X) - 90.0) * -1.0;
    //println!("angle: {}", angle);

    let mut eff_handle = VarModule::get_int(fighter.battle_object, SPECIAL_HI_MARKER_EFFECT_HANDLE) as u32;
    let guide_pos = arrow_guide_pos(fighter, angle.into());
    if !EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
        eff_handle = EffectModule::req(
            fighter.module_accessor,
            Hash40::new("sys_direction2"),
            &Vector3f {
                x: guide_pos.x,
                y: guide_pos.y,
                z: 0.0,
            },
            &Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            1.0,
            0,
            -1,
            false,
            0,
        ) as u32;
        VarModule::set_int(fighter.battle_object, SPECIAL_HI_MARKER_EFFECT_HANDLE, eff_handle as i32);
    } else {
        EffectModule::set_pos(
            fighter.module_accessor,
            eff_handle,
            &Vector3f {
                x: guide_pos.x,
                y: guide_pos.y,
                z: 0.0,
            },
        );
    }
    EffectModule::set_rot(
        fighter.module_accessor,
        eff_handle,
        &Vector3f {
            x: 0.0,
            y: 0.0,
            z: angle - 90.0,
        },
    );

    let team_color = FighterUtil::get_team_color(fighter.module_accessor);
    let effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
    EffectModule::set_rgb(fighter.module_accessor, eff_handle, effect_team_color.x(), effect_team_color.y(), effect_team_color.z());
}

// FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP

unsafe extern "C" fn special_hi_keep_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_keep_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_rise"), 0.0, 1.0, false, 0.0, false, false);

    // set rot f0
    let rot_x = VarModule::get_float(fighter.battle_object, SPECIAL_HI_ROT_X) * fighter.lr() * 0.8;
    PostureModule::set_rot(fighter.module_accessor, &Vector3f::new(rot_x, 0.0, 0.0), 0);

    // fall during startup without landing
    fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
    fighter.set_situation(SITUATION_KIND_AIR.into());
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 1.into());

    // sfx
    let charge_frame = VarModule::get_int(fighter.battle_object, SPECIAL_HI_CHARGE_FRAME) as f32;
    let charge_frame_stage_1 = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.charge_frame_stage_1");
    let charge_frame_stage_2 = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.charge_frame_stage_2");
    let charge_frame_stage_3 = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.charge_frame_stage_3");
    let sfx = if charge_frame >= charge_frame_stage_1 {
        "se_common_bomb_ll"
    } else if charge_frame >= charge_frame_stage_2 {
        "se_common_bomb_l"
    } else if charge_frame >= charge_frame_stage_1 {
        "se_common_bomb_m"
    } else {
        "se_common_bomb_s"
    };
    PLAY_STATUS(fighter, Hash40::new(sfx));

    fighter.main_shift(special_hi_keep_main_loop)
}

unsafe extern "C" fn special_hi_keep_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // handle movement / rot
    if !StopModule::is_stop(fighter.module_accessor) {
    //&& !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { // is_stop didnt work?
        special_hi_keep_movement_handling(fighter);
    }

    // lc if landing (bypass soft landing)
    if fighter.check_land_cancel(None) {
        return 1.into();
    }

    // run ledge check
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    // act out
    fighter.sub_transition_group_check_air_attack();

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        // enable fastfall on cancel frame
        fighter.sub_air_check_dive();
        // enable acting on cancel frame
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_keep_movement_handling(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::inc_int(fighter.battle_object, SPECIAL_HI_KEEP_FRAME);
    let frame = VarModule::get_int(fighter.battle_object, SPECIAL_HI_KEEP_FRAME);
    let charge_frame = VarModule::get_int(fighter.battle_object, SPECIAL_HI_CHARGE_FRAME) as f32;
    let launch_rise_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.launch_rise_frame");
    let launch_brake_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.launch_brake_frame");
    let launch_brake_end_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.launch_brake_end_frame");
    let launch_fall_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.launch_fall_frame");
    let rot = VarModule::get_float(fighter.battle_object, SPECIAL_HI_ROT_X);

    // init movement
    if frame == launch_rise_frame {
        // Enable ledgegrab
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());

        if charge_frame > 0.0 {
            // launch speed
            let launch_speed = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.launch_speed");
            let launch_speed_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.launch_speed_mul");
            let speed = launch_speed + (charge_frame * launch_speed_mul);
            let stick_x = (rot * -1.0 + 90.0).to_radians().cos();
            let stick_y = (rot * -1.0 + 90.0).to_radians().sin();
            let speed_x = speed * stick_x * fighter.lr();
            let speed_y = (speed * stick_y);
            let lr = fighter.lr();
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
            KineticModule::unable_energy_all(fighter.module_accessor);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_FREE, speed_x * lr, speed_y, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, -1.0);
            sv_kinetic_energy!(enable, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        } else {
            // give 0 stall empty use better frames
            VarModule::on_flag(fighter.battle_object, SPECIAL_HI_GROUND_START);
            sv_kinetic_energy!(add_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.75);
        }
        fighter.set_situation_keep(L2CValue::I32(*SITUATION_KIND_AIR), 0.into());
    }
    // slowly decel before enabling drift/gravity
    if frame >= launch_brake_frame && frame <= launch_brake_end_frame {
        if charge_frame > 0.0 {
            let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
            let speed = Vector2f {
                x: lua_bind::KineticEnergy::get_speed_x(stop_energy),
                y: lua_bind::KineticEnergy::get_speed_y(stop_energy),
            };
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x * 0.95, speed.y * 0.95);
        }
    }
    // end movement
    if frame == launch_fall_frame {
        if charge_frame > 0.0 {
            let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let y_speed = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, x_speed * 0.60, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, y_speed * 0.60);
        }
        // air attack cancel
        fighter.enable_transition_term(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    }
    // interpolate back to upright position
    let current_rot = PostureModule::rot_x(fighter.module_accessor, 0);
    if (fighter.motion_frame() >= 39.0) && current_rot != 0.0 {
        let rot_mul = 1.0;
        let rot_amount = 0.07 * rot_mul; // percent of remaining distance rotated each frame. will decrease exponentially
        let mut new_rot = current_rot - (current_rot * rot_amount);
        if (-1.0..1.0).contains(&new_rot) {
            new_rot = 0.0
        }; // snap to 0 when close enough
        PostureModule::set_rot(fighter.module_accessor, &Vector3f::new(new_rot, 0.0, 0.0), 0);
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_keep_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_rot(fighter.module_accessor, &Vector3f::zero(), 0);
    0.into()
}

unsafe extern "C" fn stub_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end);

    agent.status(Pre, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, special_hi_keep_pre);
    agent.status(Main, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, special_hi_keep_main);
    agent.status(End, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, special_hi_keep_end);

    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, stub_status);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, stub_status);
    agent.status(Init, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, stub_status);
    agent.status(Exec, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, stub_status);
}
