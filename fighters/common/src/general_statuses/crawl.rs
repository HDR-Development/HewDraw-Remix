// status imports
use super::*;
use globals::*;
// This file contains code for fixing crawl maintaining momentum if going from left to right but not from right to left

pub fn install() {
    skyline::nro::add_hook(nro_main).unwrap();
}

fn nro_main(nro: &skyline::nro::NroInfo) {
    match nro.name {
        "common" => {
            skyline::install_hooks!(
                status_sub_squat_walk_init_hook,
                status_sub_squat_walk_main_hook
            );
        }
        _ => (),
    }
}

// Code runs when crawl starts
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_squat_walk_uniq_process_init)]
pub unsafe fn status_sub_squat_walk_init_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);

    let squat_walk_max_speed = fighter.FL_get_squat_walk_max_speed().get_f32();
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(boma);
    let speed_forward = speed_x * lr;

    /* Vanilla
    let mut crawl_speed = 0.0;
    if speed_forward >= 0.0 {
        crawl_speed = speed_x / squat_walk_max_speed;
    } */
    
    /* HDR
    Using speed_forward instead of speed_x fixes losing your speed when going from left to right 
    Additionally, losing the positive speed check lets you keep backwards speed
    For it to keep backwards speed properly, uses the absolute value, as negative values
    mess up the acceleration calculation in the main function */
    let mut crawl_speed = (speed_forward / squat_walk_max_speed).abs();

    WorkModule::set_float(boma, crawl_speed, *FIGHTER_STATUS_SQUAT_WALK_WORK_FLOAT_SPEED);

    let motion_kind = fighter.get_motion_kind().get_hash();
    let mut trans_tra_end_frame_vec = Vector3f{x:0.0, y:0.0, z:0.0};
    MotionModule::trans_tra_end_frame(boma, motion_kind, &mut trans_tra_end_frame_vec as *mut Vector3f);

    let end_frame = MotionModule::end_frame_from_hash(boma, motion_kind);
    if end_frame.abs() < 0.5 {
        trans_tra_end_frame_vec.z = 20.0;
    }

    let crawl_speed_ratio = (squat_walk_max_speed * end_frame / trans_tra_end_frame_vec.z).abs();
    WorkModule::set_float(boma, crawl_speed_ratio, *FIGHTER_STATUS_SQUAT_WALK_WORK_FLOAT_SPEED_MAX_RATIO);

    return 0.into();
}

// Code runs every frame of crawl
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_squat_walk_uniq_process_main)]
pub unsafe fn status_sub_squat_walk_main_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);

    /* This function returns a value between 0 and 1 (never negative) based on the stick's x position,
    taking into account that since the player is crawling the real stick x position will be in a smaller range of values */
    let stick_x = fighter.FL_get_stick_x_rate().get_f32();
    let crawl_speed = WorkModule::get_float(boma, *FIGHTER_STATUS_SQUAT_WALK_WORK_FLOAT_SPEED);

    let squat_walk_max_forward_speed = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SQUAT_WALK_SPEED_FORWARD_MAX);
    let squat_walk_speed_max_mul = WorkModule::get_param_float(boma, hash40("squat_walk_speed_max_mul"), 0);
    let squat_walk_ratio = squat_walk_max_forward_speed * squat_walk_speed_max_mul;

    let walk_accel_add = WorkModule::get_param_float(boma, hash40("walk_accel_add"), 0);
    let squat_walk_accel_add = walk_accel_add / squat_walk_ratio;

    let walk_accel_mul = WorkModule::get_param_float(boma, hash40("walk_accel_mul"), 0);
    let squat_walk_accel_mul = walk_accel_mul / squat_walk_ratio;

    let mut squat_walk_accel = squat_walk_accel_mul * stick_x;
    if stick_x > 0.0 {
        squat_walk_accel += squat_walk_accel_add;
    }
    else {
        squat_walk_accel -= squat_walk_accel_add;
    }

    let speed_stick_ratio = crawl_speed / stick_x;
    if 0.0 < speed_stick_ratio && speed_stick_ratio < 1.0 {
        // This param is set to 0
        let mysterious_param = WorkModule::get_param_float(boma, Hash40::new_raw(0x6e5ec7051).hash, Hash40::new_raw(0xef53a098).hash);
        // So despite all the math this just sets squat_walk_accel to 0 lol
        squat_walk_accel *= (1.0 - speed_stick_ratio) * mysterious_param;
    }

    let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
    let squat_walk_brake = ground_brake / squat_walk_ratio;

    if stick_x != 0.0 {
        if squat_walk_accel <= 0.0 {
            if squat_walk_accel + crawl_speed < stick_x {
                squat_walk_accel = squat_walk_brake;
                if squat_walk_accel + crawl_speed > stick_x {
                    squat_walk_accel = stick_x - crawl_speed;
                }
            }
        }
        else {
            if squat_walk_accel + crawl_speed > stick_x {
                squat_walk_accel = -squat_walk_brake;
                if squat_walk_accel + crawl_speed < stick_x {
                    squat_walk_accel = stick_x - crawl_speed;
                }
            }
        }
    }
    else if crawl_speed > 0.0 {
        squat_walk_accel = squat_walk_brake;
    }
    else {
        squat_walk_accel = -squat_walk_brake;
    }

    WorkModule::set_float(boma, crawl_speed + squat_walk_accel, *FIGHTER_STATUS_SQUAT_WALK_WORK_FLOAT_SPEED);
    fighter.set_squat_walk_motion_rate();
    return 0.into();
}