use core::f32;

use super::*;

unsafe extern "C" fn special_hi2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        false,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn angler(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.left_stick_x();
    let stick_y = fighter.left_stick_y();
    let mut length = sv_math::vec2_length(stick_x, stick_y);
    let wrap_stick = fighter.get_param_float("param_special_hi", "wrap_stick");

    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    let lr = fighter.lr();
    let mut angle = if length >= wrap_stick {
        stick_y.atan2(stick_x * lr)
    } else {
        90.0_f32.to_radians()
    };
    let test_angle = if angle < f32::consts::PI {angle} else {angle - f32::consts::PI};
    let mut detach = false;
    if test_angle < f32::consts::PI && test_angle > 0.0 {
        detach = true;
    }
    let wrap_speed_multi = fighter.get_param_float("param_special_hi", "wrap_speed_multi");
    let wrap_speed_add = fighter.get_param_float("param_special_hi", "wrap_speed_add");
    let mut speed_x = 0.0;
    let mut speed_y = wrap_speed_multi + wrap_speed_add;
    // if angled w/ stick
    if length > wrap_stick {
        let length_mul = wrap_speed_multi * length;
        let speed = length_mul + wrap_speed_add;
        let cos = angle.cos();
        speed_x = speed * cos;
        speed_x *= lr;

        let sin = angle.sin();
        speed_y = speed * sin;
    }
    // If teleport angle is upwards or you are already in air
    // force airborne state
    if detach || fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::set_attach_ground(fighter.module_accessor, false);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    KineticModule::unable_energy_all(fighter.module_accessor);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_FREE, speed_x, speed_y, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, -1.0);
    sv_kinetic_energy!(enable, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    GroundModule::clear_cliff_point(fighter.module_accessor);
    0.into()
}

unsafe extern "C" fn special_hi2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    VisibilityModule::set_whole(fighter.module_accessor, false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
    fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
    fighter.set_int(0, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    angler(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi2_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    let frame = fighter.get_int(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    let move_time = fighter.get_param_int("param_special_hi", "move_time");
    if frame >= move_time {
        fighter.change_status(FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3.into(), true.into())
    }
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    special_hi_2_check_ground(fighter);
    if fighter.is_cat_flag(Cat1::SpecialAny) && !StatusModule::is_changing(fighter.module_accessor) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        fighter.change_status(FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3.into(), true.into())
    }
    //substatus
    if !StatusModule::is_changing(fighter.module_accessor) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        let move_xlu = fighter.get_param_int("param_special_hi", "move_xlu"); // travel frame to begin ignoring platforms
        let cliff_check_frame = fighter.get_param_int("param_special_hi", "move_cliff_check");
        if frame == move_xlu {
            GroundModule::set_passable_check(fighter.module_accessor, false);
        }
        if frame == cliff_check_frame {
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
        }
        if frame < 2 {
            fighter.on_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND);
        }
    }
    0.into()
}

// Copies nasty vanilla math
// with adjusted logic to control wall-ride/floor-ride behavior
unsafe extern "C" fn special_hi_2_check_ground(fighter: &mut L2CFighterCommon) {
    let init_speed_x = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_X);
    let init_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_Y);
    let floor_speed_x = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_FLOOR_SPEED_X);
    if floor_speed_x.abs() > 0.0 && init_speed_y < 0.0 && init_speed_x.abs() > 0.0 
    && (GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) && !GroundModule::is_passable_ground(fighter.module_accessor)) {
        // Travel speed for diagonally-down floor-rides
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, floor_speed_x, 0.0, 0.0);
    } else {
        // Travel speed for all other scenarios
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, init_speed_x, init_speed_y, 0.0);
    }
    // If on a platform,
    // skip floor-ride speed redirection
    if GroundModule::is_passable_check(fighter.module_accessor) && GroundModule::is_passable_ground(fighter.module_accessor) {
        return;
    }
    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
    let speed = Vector2f {x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};
    // If not a diagonally-down teleport,
    // or if already grounded,
    // skip floor-ride speed redirection
    if !GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    || (speed.x.abs() < 0.001 || speed.y > -0.001)
    || fighter.global_table[PREV_SITUATION_KIND] != SITUATION_KIND_AIR {
        return;
    }
    // Compute a new ground-aligned velocity vector
    // for intended floor-ride speed redirection
    // 
    // Only intended to run on the first frame you land during the travel
    let mut length = sv_math::vec3_length(speed.x, speed.y, 0.0);
    if length > 0.0 {
        let touch_x = GroundModule::get_touch_normal_x(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let touch_y = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);

        let touch = fighter.Vector3__create(touch_x.into(), touch_y.into(), 0.0_f32.into());
        let something = fighter.Vector3__create(0.0_f32.into(), 0.0_f32.into(), 1.0_f32.into());
        let mut cross = fighter.Vector3__cross(touch.clone(), something);

        let math = 1.0 / length;
        let speed_mul = Vector3f {
            x: speed.x * math,
            y: speed.y * math,
            z: 0.0,
        };
        let mut final_dot = sv_math::vec3_dot(cross["x"].get_f32(), cross["y"].get_f32(), cross["z"].get_f32(), speed_mul.x, speed_mul.y, speed_mul.z);
        if -0.00001 <= final_dot && final_dot <= 0.00001 {
            final_dot = sv_math::vec3_dot(cross["x"].get_f32(), cross["y"].get_f32(), cross["z"].get_f32(), fighter.lr(), 0.0, 0.0);
        }

        if final_dot < 0.0 {
            let x = cross["x"].get_f32();
            let y = cross["y"].get_f32();
            let z = cross["z"].get_f32();
            cross["x"].assign(&L2CValue::F32(x * -1.0));
            cross["y"].assign(&L2CValue::F32(y * -1.0));
            cross["z"].assign(&L2CValue::F32(z * -1.0));
        }
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, cross["x"].get_f32() * length, cross["y"].get_f32() * length, cross["z"].get_f32() * length);
        // set new speed to be reapplied each frame
        VarModule::set_float(fighter.battle_object, vars::common::status::TELEPORT_FLOOR_SPEED_X, cross["x"].get_f32() * length);
    }
}

unsafe extern "C" fn special_hi2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3 {
        // Reappearance turnaround
        if fighter.left_stick_x() * fighter.lr()
        <= fighter.get_param_float("common", "turn_stick_x") {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
        // Use a waveland-esque snap threshold to determine whether to snap to ground
        let init_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_Y);
        let pos = *PostureModule::pos(fighter.module_accessor);
        let bot_snap = &Vector2f::new(pos.x, pos.y - 1.0);
        let top_snap = &Vector2f::new(pos.x, pos.y + 11.0); // around chest level
        let ground_pos_any = &mut Vector2f::zero();
        let ground_pos_stage = &mut Vector2f::zero();
        let is_touch_any = GroundModule::line_segment_check(fighter.module_accessor, top_snap, bot_snap, &Vector2f::zero(), ground_pos_any, true);
        let is_touch_stage = GroundModule::line_segment_check(fighter.module_accessor, top_snap, bot_snap, &Vector2f::zero(), ground_pos_stage, false);
        let can_snap = !(is_touch_any == 0 as *const *const u64 || (is_touch_stage != 0 as *const *const u64 && init_speed_y > 0.0)); // avoid snapping to stage during rising teleport
        if can_snap {
            PostureModule::set_pos(fighter.module_accessor, &Vector3f::new(pos.x, ground_pos_any.y + 0.1, pos.z));
            GroundModule::attach_ground(fighter.module_accessor, false);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
                // telecancel b reverse momentum
                let lr = PostureModule::lr(fighter.module_accessor);
                let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                let speed = Vector2f{ x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x.abs() * lr * 1.05, speed.y); //b-reverse telecancel reverses momentum on ground
                // telecancel gfx
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("zelda_atk"), Hash40::new("top"), &Vector3f::new(5.5 * lr, 8.0, -2.1), &Vector3f::zero(), 1.65, true, 0, 0, 0, 0, 0, false, false);
                LAST_EFFECT_SET_COLOR(fighter, 0.95, 3.0, 0.6);
                LAST_EFFECT_SET_ALPHA(fighter, 0.75);
                LAST_EFFECT_SET_RATE(fighter, 1.10);
                // telecancel sound
                PLAY_SE(fighter, Hash40::new("se_zelda_appear02"));
            }
        }
    }
    0.into()
}

unsafe extern "C" fn special_hi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            let mut end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, "landing_heavy".to_hash());
            let mut motion_rate = (end_frame-3.0)/13.0;
            if !VarModule::is_flag(fighter.battle_object, vars::zelda::instance::SPECIAL_HI_GROUNDED_TELEPORT) {
                motion_rate = (end_frame-3.0)/17.0;
            } //extra 4f A2G
            MotionModule::change_motion(fighter.module_accessor, "landing_heavy".to_hash(), 3.0, motion_rate, false, 0.0, false, false);
        } else {
            MotionModule::change_motion(fighter.module_accessor, "special_hi".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        }
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_int(*SITUATION_KIND_AIR, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, "special_air_hi".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        let x_max = fighter.get_param_float("param_special_hi", "fall_x_mull_value");
        WorkModule::set_float(fighter.module_accessor, x_max, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    }

    fighter.main_shift(special_hi3_main_loop)
}

unsafe extern "C" fn special_hi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let control = ControlModule::get_attack_air_kind(fighter.module_accessor);
    if control == *FIGHTER_COMMAND_ATTACK_AIR_KIND_NONE {
        FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
    } //?
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
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            WorkModule::set_float(fighter.module_accessor, 18.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME); // halved lag if canceled
        } else {
            let landing_lag = fighter.get_param_float("param_special_hi", "landing_frame");
            WorkModule::set_float(fighter.module_accessor, landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
                WorkModule::set_float(fighter.module_accessor, 18.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME); // halved lag if canceled
            } else {
                let landing_lag = fighter.get_param_float("param_special_hi", "landing_frame");
                WorkModule::set_float(fighter.module_accessor, landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            }
            return 1.into();
        } else {
            // clear buffer
            ControlModule::reset_trigger(fighter.module_accessor);
            ControlModule::clear_command(fighter.module_accessor, true);
            ControlModule::reset_special_command(fighter.module_accessor, true);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        if fighter.is_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_DIVE) {
            fighter.sub_air_check_dive();
        }
        if fighter.is_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CONTROL) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            fighter.off_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CONTROL);
        }
        if !fighter.global_table[IS_STOPPING].get_bool() && KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_MOTION_FALL {
            if !fighter.is_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_1) {
                let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                let speed = Vector2f{x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};
                // Gradually reduce y speed by 10% every frame
                // until you change to MOTION_FALL energy
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x, speed.y * 0.9);
            } else {
                if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
                    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                    let speed_y = lua_bind::KineticEnergy::get_speed_y(stop_energy);
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    let air_speed_x_stable = fighter.get_param_float("air_speed_x_stable", "");
                    let fall_x_mul = fighter.get_param_float("param_special_hi", "fall_x_mull_value");
                    let mut x_cap = air_speed_x_stable * fall_x_mul;
                    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, x_cap, 0.0);
                } // only set limits and enable gravity once
            }
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, special_hi2_pre);
    agent.status(Main, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, special_hi2_main);
    agent.status(End, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, special_hi2_end);

    agent.status(Main, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, special_hi3_main);
}