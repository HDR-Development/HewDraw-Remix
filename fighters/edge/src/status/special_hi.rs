use super::*;

unsafe extern "C" fn edge_special_hi_param_int_helper(fighter: &mut L2CFighterCommon, hash: L2CValue, charged_rush: L2CValue) -> L2CValue {
    let param = edge_special_hi_param_helper_inner(hash, charged_rush).get_u64();
    WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), param).into()
}

unsafe extern "C" fn edge_special_hi_param_float_helper(fighter: &mut L2CFighterCommon, hash: L2CValue, charged_rush: L2CValue) -> L2CValue {
    let param = edge_special_hi_param_helper_inner(hash, charged_rush).get_u64();
    WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), param).into()
}

unsafe extern "C" fn edge_special_hi_param_helper_inner(hash: L2CValue, charged_rush: L2CValue) -> L2CValue {
    let hash = hash.get_u64();
    if !charged_rush.get_bool() {
        return hash.into();
    }
    let new_hash = if hash == hash40("rot_decide_frame") {
        hash40("charged_rot_decide_frame")
    }
    else if hash == hash40("rot_end_frame") {
        hash40("charged_rot_end_frame")
    }
    else if hash == hash40("rush_frame") {
        hash40("charged_rush_frame")
    }
    else if hash == hash40("rush_speed") {
        hash40("charged_rush_speed")
    }
    else if hash == hash40("rush_brake_frame") {
        hash40("charged_rush_brake_frame")
    }
    else if hash == hash40("rush_brake") {
        hash40("charged_rush_brake")
    }
    else if hash == hash40("ground_speed_x_mul") {
        hash40("charged_ground_speed_x_mul")
    }
    else if hash == hash40("landing_speed_x_mul") {
        hash40("charged_landing_speed_x_mul")
    }
    else if hash == hash40("landing_brake_x") {
        hash40("charged_landing_brake_x")
    }
    else if hash == hash40("landing_fix_frame") {
        hash40("charged_landing_fix_frame")
    }
    else if hash == hash40("rotate_back_begin_frame") {
        hash40("charged_rotate_back_begin_frame")
    }
    else if hash == hash40("rotate_back_end_frame") {
        hash40("charged_rotate_back_end_frame")
    }
    else if hash == hash40("rush_end_speed_mul") {
        hash40("charged_rush_end_speed_mul")
    }
    else if hash == hash40("rush_end_brake_x") {
        hash40("charged_rush_end_brake_x")
    }
    else if hash == hash40("rush_end_gravity_accel") {
        hash40("charged_rush_end_gravity_accel")
    }
    else if hash == hash40("control_accel_x_mul") {
        hash40("charged_control_accel_x_mul")
    }
    else if hash == hash40("control_speed_x_max_mul") {
        hash40("charged_control_speed_x_max_mul")
    }
    else{
        hash
    };
    new_hash.into()
}

// FIGHTER_STATUS_KIND_SPECIAL_HI

unsafe extern "C" fn edge_special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE,
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
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn edge_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_DIRECTION_EFFECT_VISIBLE);
    WorkModule::set_int(fighter.module_accessor, *EFFECT_HANDLE_NULL, *FIGHTER_EDGE_STATUS_SPECIAL_HI_INT_DIRECTION_EFFECT_HANDLE);
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_hi").into());
    edge_special_hi_set_kinetics(fighter, true.into());
    let mot = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        Hash40::new("special_air_hi_start")
    }
    else {
        Hash40::new("special_hi_start")
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_hi_set_kinetics(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if param_1.get_bool() && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            return;
        }
        sv_kinetic_energy!(
            set_needs_set_param,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            false
        );
        if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            sv_kinetic_energy!(
                set_needs_set_param,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                false
            );
        }
    }
}

unsafe extern "C" fn edge_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_hi").into());
        edge_special_hi_set_kinetics(fighter, false.into());
        fighter.sub_change_motion_by_situation(Hash40::new("special_hi_start").into(), Hash40::new("special_air_hi_start").into(), true.into());
    }
    edge_special_hi_set_accel(fighter);
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let mut stick = fighter.Vector2__create(stick_x.into(), stick_y.into());
    if stick["x"].get_f32().abs() + stick["y"].get_f32().abs() < 0.5 {
        stick["x"].assign(&L2CValue::F32(0.0));
        stick["y"].assign(&L2CValue::F32(1.0));
    }
    let normalize = fighter.Vector2__normalize(stick);
    edge_special_hi_set_dir_handle(fighter, normalize["x"].clone(), normalize["y"].clone());
    let charged_rush = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
    let param = edge_special_hi_param_int_helper(fighter, hash40("rot_decide_frame").into(), charged_rush.into()).get_i32();
    if param as f32 <= fighter.global_table[CURRENT_FRAME].get_f32() {
        WorkModule::set_float(fighter.module_accessor, normalize["x"].get_f32(), *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_DIR_X);
        WorkModule::set_float(fighter.module_accessor, normalize["y"].get_f32(), *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_DIR_Y);
        if 0.125 < fighter.global_table[STICK_X].get_f32().abs() {
            PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
        let direction = edge_special_hi_ground_touch_down(fighter, normalize["x"].clone(), normalize["y"].clone());
        let lr = PostureModule::lr(fighter.module_accessor);
        let angle = sv_math::vec2_angle(lr, 0.0, direction["x"].get_f32(), direction["y"].get_f32());
        let degrees = angle.to_degrees();
        let sign = fighter.sign((angle * -1.0).into()).get_f32();
        WorkModule::set_float(fighter.module_accessor, degrees * sign, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_ROT_DEGREE);
        fighter.shift(L2CValue::Ptr(edge_special_hi_main_loop_shift as *const () as _));
    }
    0.into()
}

unsafe extern "C" fn edge_special_hi_main_loop_shift(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_hi").into());
    edge_special_hi_set_kinetics(fighter, false.into());
    fighter.sub_change_motion_by_situation(Hash40::new("special_hi_start").into(), Hash40::new("special_air_hi_start").into(), true.into());
    edge_special_hi_set_accel(fighter);
    let stick_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_DIR_X);
    let stick_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_DIR_Y);
    edge_special_hi_set_dir_handle(fighter, stick_x.into(), stick_y.into());
    let charged_rush = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
    let rot_decide_frame = edge_special_hi_param_int_helper(fighter, hash40("rot_decide_frame").into(), charged_rush.into()).get_i32();
    let rot_end_frame = edge_special_hi_param_int_helper(fighter, hash40("rot_end_frame").into(), charged_rush.into()).get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let diff = rot_end_frame - rot_decide_frame;
    let ratio: f32 = (frame - rot_decide_frame as f32) / diff as f32;
    let clamp = ratio.clamp(0.0, 1.0);
    let degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_ROT_DEGREE);
    let rot_step = clamp * degree;
    //HDR
    // Handle logic for grounded blade dash
    let attack_extra_startup_frames_ground = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "blade_dash.attack_extra_startup_frames_ground");
    let fakeout_extra_startup_frames_ground = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "blade_dash.fakeout_extra_startup_frames_ground");
    let attack_decision_frame_ground = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "blade_dash.attack_decision_frame_ground");
    let mut blade_dash_frame_delay = 0;
    let mut grounded_blade_dash = false;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        grounded_blade_dash = true;
        if VarModule::is_flag(fighter.battle_object, vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX) {
            blade_dash_frame_delay = fakeout_extra_startup_frames_ground;
        }
        else{
            blade_dash_frame_delay = attack_extra_startup_frames_ground;
        }
    }
    let mut transition_frame = rot_end_frame;
    if grounded_blade_dash {
        transition_frame = attack_decision_frame_ground + blade_dash_frame_delay;
    }
    WorkModule::set_float(fighter.module_accessor, rot_step, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE);
    slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, MA_MSC_CMD_SLOEP_SLOPE_KIND_NONE);
    if !charged_rush {
        if transition_frame as f32 <= frame {
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH.into(), false.into());
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn edge_special_hi_set_accel(fighter: &mut L2CFighterCommon) {
    let start_stop_y_frame_air = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("start_stop_y_frame_air"));
    if start_stop_y_frame_air as f32 <= fighter.global_table[CURRENT_FRAME].get_f32() + 1.0 {
        let start_gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_gravity"));
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -start_gravity
        );
    }
}

unsafe extern "C" fn edge_special_hi_set_dir_handle(fighter: &mut L2CFighterCommon, stick_x: L2CValue, stick_y: L2CValue) {
    let vec2 = fighter.Vector2__create(stick_x, stick_y);
    let normalize = fighter.Vector2__normalize(vec2.clone());
    let angle = sv_math::vec2_angle(0.0, 1.0, normalize["x"].get_f32(), normalize["y"].get_f32());
    let degrees = angle.to_degrees();
    let sign = fighter.sign((-normalize["x"].get_f32()).into());
    let degrees = sign.get_f32() * degrees;
    let pos = PostureModule::pos(fighter.module_accessor);
    let direction_effect_offset_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("direction_effect_offset_y"));
    let scale = PostureModule::scale(fighter.module_accessor);
    let offset = scale * direction_effect_offset_y + (*pos).y;
    let pos = Vector3f{
        x: (*pos).x,
        y: offset,
        z: (*pos).z
    };
    let direction_effect_radius = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("direction_effect_radius"));
    let radius = scale * direction_effect_radius;
    let x_pos = radius * vec2["x"].get_f32() + pos.x;
    let y_pos = radius * vec2["y"].get_f32() + pos.y;
    let handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_INT_DIRECTION_EFFECT_HANDLE) as u32;
    if handle != *EFFECT_HANDLE_NULL as u32 {
        EffectModule::set_pos(
            fighter.module_accessor,
            handle,
            &Vector3f{x: x_pos, y: y_pos, z: 0.0}
        );
        EffectModule::set_rot(
            fighter.module_accessor,
            handle,
            &Vector3f{x: 0.0, y: 0.0, z: degrees}
        );
    }
    else {
        let effect = EffectModule::req(
            fighter.module_accessor,
            Hash40::new("edge_octaslash_direction"),
            &Vector3f{x: x_pos, y: y_pos, z: 0.0},
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            1.0,
            0,
            -1,
            false,
            0
        ) as u32;
        EffectModule::set_rot(
            fighter.module_accessor,
            effect,
            &Vector3f{x: 0.0, y: 0.0, z: degrees}
        );
        let color = FighterUtil::get_team_color(fighter.module_accessor);
        let effect_color = FighterUtil::get_effect_team_color(EColorKind(color as i32), Hash40::new("direction_effect_color"));
        EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_color.x, effect_color.y, 0.0);
        WorkModule::set_int(fighter.module_accessor, effect as i32, *FIGHTER_EDGE_STATUS_SPECIAL_HI_INT_DIRECTION_EFFECT_HANDLE);
    }
    let visible = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_DIRECTION_EFFECT_VISIBLE);
    EffectModule::set_visible_kind(fighter.module_accessor, Hash40::new("edge_octaslash_direction"), visible);
}

unsafe extern "C" fn edge_special_hi_ground_touch_down(fighter: &mut L2CFighterCommon, stick_x: L2CValue, stick_y: L2CValue) -> L2CValue {
    let vec2 = fighter.Vector2__create(stick_x.clone(), stick_y.clone());
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let normal_x = GroundModule::get_touch_normal_x(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let angle = sv_math::vec2_angle(normal_x, normal_y, stick_x.get_f32(), stick_y.get_f32());
        let rad = 90.0_f32.to_radians();
        if rad < angle {
            let rad = (-90.0_f32).to_radians();
            let lr = PostureModule::lr(fighter.module_accessor);
            let _rot = sv_math::vec2_rot(stick_x.get_f32(), stick_y.get_f32(), rad * lr);
            let mut normalize = fighter.Vector2__normalize(vec2);
            normalize["z"].assign(&L2CValue::Bool(true));
            return normalize;
        }
    }
    let mut normalize = fighter.Vector2__normalize(vec2);
    normalize["z"].assign(&L2CValue::Bool(false));
    normalize
}

// FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH

pub unsafe extern "C" fn edge_special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH)(fighter);

    // Grounded Blade Rush shorten mechanic
    let dir_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_DIR_X);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    && dir_x != 0.0 {
    //&& (dir_x != 0.0 || VarModule::is_flag(fighter.battle_object, vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX)){
        let rush_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_speed"));
        let stopEnergy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergyNormal;
        let vec2 = Vector2f{x: rush_speed * dir_x, y: 0.0};
        // let mut movement_mul = 1.0;
        // if VarModule::is_flag(fighter.battle_object, vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX) {
        //     movement_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "blade_dash.fakeout_dash_movement_mul");;
        // }
        // let vec2 = Vector2f{x: rush_speed * dir_x * movement_mul, y: 0.0};
        app::lua_bind::KineticEnergyNormal::set_speed(stopEnergy, &vec2);
    }

    ret
}

// // FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END

// #[status_script(agent = "edge", status = FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn edge_special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let rush_degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE);
//     WorkModule::set_float(fighter.module_accessor, rush_degree, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_ROT_DEGREE);
//     let charged_rush = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
//     fighter.clear_lua_stack();
//     lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
//     let speed = sv_kinetic_energy::get_speed(fighter.lua_state_agent);
//     let vec2 = fighter.Vector2__create(speed.x.into(), speed.y.into());
//     let rush_end_speed_mul = edge_special_hi_param_float_helper(fighter, hash40("rush_end_speed_mul").into(), charged_rush.into()).get_f32();
//     let rush_end_brake_x = edge_special_hi_param_float_helper(fighter, hash40("rush_end_brake_x").into(), charged_rush.into()).get_f32();
//     let speed_x = vec2["x"].get_f32() * rush_end_speed_mul;
//     sv_kinetic_energy!(
//         set_speed,
//         fighter,
//         FIGHTER_KINETIC_ENERGY_ID_STOP,
//         speed_x,
//         0.0
//     );
//     sv_kinetic_energy!(
//         set_brake,
//         fighter,
//         FIGHTER_KINETIC_ENERGY_ID_STOP,
//         rush_end_brake_x.abs(),
//         0.0
//     );
//     fighter.clear_lua_stack();
//     lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
//     let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
//     let rush_end_gravity_accel = edge_special_hi_param_float_helper(fighter, hash40("rush_end_gravity_accel").into(), charged_rush.into()).get_f32();
//     let speed_y = speed_y * rush_end_speed_mul;
//     sv_kinetic_energy!(
//         set_speed,
//         fighter,
//         FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
//         speed_y
//     );
//     sv_kinetic_energy!(
//         set_accel,
//         fighter,
//         FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
//         -rush_end_gravity_accel
//     );
//     let mot = if !charged_rush {
//         Hash40::new("special_air_hi1_end")
//     }
//     else {
//         Hash40::new("special_air_hi2_end")
//     };
//     MotionModule::change_motion(
//         fighter.module_accessor,
//         mot,
//         0.0,
//         1.0,
//         false,
//         0.0,
//         false,
//         false
//     );
//     fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_hi_end_main_loop as *const () as _))
// }

// unsafe extern "C" fn edge_special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if fighter.sub_transition_group_check_air_cliff().get_bool() {
//         return 1.into();
//     }
//     if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
//         if !MotionModule::is_end(fighter.module_accessor) {
//             let charged_rush = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
//             let rotate_back_begin_frame = edge_special_hi_param_int_helper(fighter, hash40("rotate_back_begin_frame").into(), charged_rush.into()).get_i32();
//             let rotate_back_end_frame = edge_special_hi_param_int_helper(fighter, hash40("rotate_back_end_frame").into(), charged_rush.into()).get_i32();
//             let frame = fighter.global_table[CURRENT_FRAME].get_f32();
//             if rotate_back_begin_frame as f32 <= frame {
//                 let mut diff = rotate_back_end_frame - rotate_back_begin_frame;
//                 if diff <= 0 {
//                     diff = 1;
//                 }
//                 let ratio: f32 = 1.0 - ((frame - rotate_back_begin_frame as f32) / diff as f32);
//                 let clamp = ratio.clamp(0.0, 1.0);
//                 let degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_ROT_DEGREE);
//                 let rot_step = clamp * degree;
//                 WorkModule::set_float(fighter.module_accessor, rot_step, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE);
//             }
//             if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL) {
//                 WorkModule::off_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
//                 let control_accel_x_mul = edge_special_hi_param_float_helper(fighter, hash40("control_accel_x_mul").into(), charged_rush.into()).get_f32();
//                 let control_speed_x_max_mul = edge_special_hi_param_float_helper(fighter, hash40("control_speed_x_max_mul").into(), charged_rush.into()).get_f32();
//                 sv_kinetic_energy!(
//                     reset_energy,
//                     fighter,
//                     FIGHTER_KINETIC_ENERGY_ID_CONTROL,
//                     ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
//                     0.0, 0.0, 0.0, 0.0, 0.0
//                 );
//                 sv_kinetic_energy!(
//                     mul_x_speed_max,
//                     fighter,
//                     FIGHTER_KINETIC_ENERGY_ID_CONTROL,
//                     control_speed_x_max_mul
//                 );
//                 sv_kinetic_energy!(
//                     mul_x_accel_mul,
//                     fighter,
//                     FIGHTER_KINETIC_ENERGY_ID_CONTROL,
//                     control_accel_x_mul
//                 );
//                 sv_kinetic_energy!(
//                     mul_x_accel_add,
//                     fighter,
//                     FIGHTER_KINETIC_ENERGY_ID_CONTROL,
//                     control_accel_x_mul
//                 );
//                 sv_kinetic_energy!(
//                     enable,
//                     fighter,
//                     *FIGHTER_KINETIC_ENERGY_ID_CONTROL
//                 );
//             }
//         }
//         else {
//             fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
//         }
//     }
//     else {
//         fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
//     }
//     0.into()
// }

pub fn install(agent: &mut Agent) {
        //.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, edge_special_hi_pre)
        //.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, edge_special_hi_main)
    agent.status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, edge_special_hi_rush_main);
        //.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI_RUSH, edge_special_hi_end)
}