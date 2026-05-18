use core::f32;

use super::*;

unsafe extern "C" fn special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        let mut stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
        let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::KineticEnergy;
        let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let y_speed = fighter.get_param_float("param_special_hi", "speed_y");
        // set momentum
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SHEIK_SPECIAL_HI_AIR);
        lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f{x: x_speed, y: 0.0}, &Vector3f::zero(), fighter.module_accessor);
        lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f{x: 0.0, y: y_speed}, &Vector3f::zero(), fighter.module_accessor);
        lua_bind::KineticEnergy::enable(stop_energy);
        lua_bind::KineticEnergy::enable(gravity_energy);
        // should make startup naturally decel?
        let air_brake_x = fighter.get_param_float("air_brake_x", "");
        let air_speed_x_stable = fighter.get_param_float("air_speed_x_stable", "");
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, air_brake_x, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, air_speed_x_stable, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    }
    0.into()
}

unsafe extern "C" fn special_hi_move_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        (*FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_ATTACK) as i32
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        (*FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

// Wuboy translated this at WuBoytH/vanilla_status
unsafe extern "C" fn angler(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.left_stick_x();
    let stick_y = fighter.left_stick_y();
    let mut length = sv_math::vec2_length(stick_x, stick_y);
    let wrap_stick = fighter.get_param_float("param_special_hi", "warp_stick");

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
    let wrap_speed_multi = fighter.get_param_float("param_special_hi", "warp_speed_mul");
    let wrap_speed_add = fighter.get_param_float("param_special_hi", "warp_speed_add");
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

unsafe extern "C" fn special_hi_move_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_FUSIN, false, -1);
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    VisibilityModule::set_whole(fighter.module_accessor, false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
    fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
    fighter.set_int(0, *FT_SHEIK_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    angler(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_move_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_move_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    let frame = fighter.get_int(*FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    let move_time = fighter.get_param_int("param_special_hi", "move_time");
    if frame >= move_time {
        fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END.into(), true.into())
    }
    // substatus
    if !StatusModule::is_changing(fighter.module_accessor)
    & !StopModule::is_stop(fighter.module_accessor) {
        fighter.inc_int(*FT_SHEIK_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        let move_xlu = fighter.get_param_int("param_special_hi", "move_xlu"); // time ignoring platforms
        let cliff_check_frame = 1; // doesn't have a param like other tps? matched to m2
        if frame == move_xlu {
            GroundModule::set_passable_check(fighter.module_accessor, false);
        }
        if frame == cliff_check_frame {
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
        }
    }
    // floor ride
    special_hi_move_check_ground(fighter);
    0.into()
}

// Copies nasty vanilla math
// with adjusted logic to control wall-ride/floor-ride behavior
unsafe extern "C" fn special_hi_move_check_ground(fighter: &mut L2CFighterCommon) {
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

unsafe extern "C" fn special_hi_null(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_init);
    agent.status(Pre, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE, special_hi_move_pre);
    agent.status(Init, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE, special_hi_null);
    agent.status(Main, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE, special_hi_move_main);
    agent.status(Exec, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE, special_hi_null);
}