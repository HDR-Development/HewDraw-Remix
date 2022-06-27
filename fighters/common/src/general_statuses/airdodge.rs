// status imports
use interpolation::Lerp;
use super::*;
use globals::*;
// This file contains code for aerial glide tosses, wavelanding

pub fn install() {
    install_status_scripts!(
        status_pre_EscapeAir,
        status_EscapeAir,
        status_end_EscapeAir
    );
    install_hooks!(
        sub_escape_air_common,
        sub_escape_air_uniq,
        sub_escape_air_common_main,
        sub_escape_air_common_strans_main
    );
}

// pre status
#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon20status_pre_EscapeAirEv")]
unsafe fn status_pre_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    // handles instant wavedashes/wavelands
    // we make sure to include this before change_motion so we check for proximity to the ground using our jumpsquat animation's ECB, rather than airdodge anim's ECB
    // a character's ECB x position can shift on the first frame of their airdodge anim, which sometimes makes them unable to wavedash on an edge even if standing on solid ground the previous frame
    if VarModule::is_flag(fighter.battle_object, vars::common::PERFECT_WAVEDASH) || fighter.handle_waveland(false, false) {
        VarModule::on_flag(fighter.battle_object, vars::common::SHOULD_WAVELAND);
        GroundModule::attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon16status_EscapeAirEv")]
unsafe fn status_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    if !VarModule::is_flag(fighter.battle_object, vars::common::SHOULD_WAVELAND) {
        fighter.sub_escape_air_common();
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);
            VarModule::on_flag(fighter.battle_object, vars::common::ENABLE_AIR_ESCAPE_MAGNET);
        } else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 0.0, 1.0, false, 0.0, false, false);
            VarModule::off_flag(fighter.battle_object, vars::common::ENABLE_AIR_ESCAPE_MAGNET);
        }
        let mut motion_rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
        let start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
        if 0 < start_frame {
            let intan_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_XLU_FRAME);
            let add_xlu_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
            motion_rate = 1.0 / ((intan_frame as f32) / ((intan_frame - add_xlu_frame) as f32));
        }
        MotionModule::set_rate(fighter.module_accessor, motion_rate);
    }

    // prevents knockback speed from applying into wavelands (boosted wavelands out of hitstun)
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE, 0.0, 0.0);
    app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    fighter.sub_shift_status_main(L2CValue::Ptr(status_EscapeAir_Main as *const () as _))
}

unsafe extern "C" fn status_EscapeAir_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::common::SHOULD_WAVELAND) {
        fighter.set_situation(L2CValue::I32(*SITUATION_KIND_GROUND));
        return 1.into();
    }
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    0.into()
}

// end status
#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon20status_end_EscapeAirEv")]
unsafe fn status_end_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].clone();
    if status_kind == FIGHTER_STATUS_KIND_FALL || status_kind == FIGHTER_STATUS_KIND_LANDING {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {

            let current_frame = fighter.global_table[CURRENT_FRAME].get_i32() as f32;
            let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("escape_air_slide"));
            let progress = current_frame / end_frame;
            let escape_air_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide"));
            let escape_air_end_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide_max"));
            let landing_frame = Lerp::lerp(&escape_air_frame, &escape_air_end_frame, &progress);
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            let speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_landing_speed_max"));
            let speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_speed_mul_escape_air_slide"));
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent) * speed_mul;
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            let speed_x = if speed_max < speed_x.abs() { speed_max * speed_x.signum() } else { speed_x };
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        } else {
            let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("landing_frame_escape_air")) as f32;
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
        if status_kind == FIGHTER_STATUS_KIND_LANDING {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
        }
        VarModule::off_flag(fighter.battle_object, vars::common::SHOULD_WAVELAND);
        VarModule::off_flag(fighter.battle_object, vars::common::PERFECT_WAVEDASH);
    }
    0.into()
}

// common air dodge init code
#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon21sub_escape_air_commonEv")]
unsafe fn sub_escape_air_common(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
    let enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL
    ];
    for x in enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_NO_WATER_INOUT_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_escape_air_uniq(L2CValue::Bool(false));
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_escape_air_uniq as *const () as _));
}

unsafe fn force_ground_attach(fighter: &mut L2CFighterCommon) {
    let id = VarModule::get_int(fighter.battle_object, vars::common::COSTUME_SLOT_NUMBER) as usize;
    let mut fighter_pos = Vector3f {
        x: PostureModule::pos_x(fighter.module_accessor),
        y: PostureModule::pos_y(fighter.module_accessor),
        z: PostureModule::pos_z(fighter.module_accessor)
    };

    let mut threshold = ParamModule::get_float(fighter.object(), ParamType::Common, "waveland_distance_threshold");
    let correction = VarModule::get_float(fighter.object(), vars::common::ECB_Y_OFFSETS);
    fighter_pos.y += correction;
    loop {
        let prev_y_pos = VarModule::get_float(fighter.battle_object, vars::common::Y_POS);
        let dist = VarModule::get_float(fighter.battle_object, vars::common::GET_DIST_TO_FLOOR);
        let new_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, &fighter_pos, fighter_pos.y, true);
        if GroundModule::attach_ground(fighter.module_accessor, false) != 0 { break; }
        if new_dist == 0.0 {
            break;
        }
        else if new_dist > 0.0 && new_dist <= threshold {
            fighter_pos.y -= new_dist;
            PostureModule::set_pos(fighter.module_accessor, &fighter_pos);
            VarModule::set_float(fighter.battle_object, vars::common::GET_DIST_TO_FLOOR, new_dist);
        }
        else {
            //println!("break");
            if prev_y_pos < fighter_pos.y {
                fighter_pos.y -= ((fighter_pos.y - prev_y_pos) + dist);
            }
            else {
                fighter_pos.y -= (dist - (prev_y_pos - fighter_pos.y));
            }
            PostureModule::set_pos(fighter.module_accessor, &fighter_pos);
            GroundModule::attach_ground(fighter.module_accessor, false);
            break;
        }
    }
}

#[deprecated(since="0.6.30", note="please use `handle_waveland` instead")]
unsafe fn sub_escape_air_waveland_check(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, vars::common::ENABLE_AIR_ESCAPE_MAGNET) {
        let mut fighter_pos = Vector3f {
            x: PostureModule::pos_x(fighter.module_accessor),
            y: PostureModule::pos_y(fighter.module_accessor),
            z: PostureModule::pos_z(fighter.module_accessor),
        };
        let mut threshold = ParamModule::get_float(fighter.object(), ParamType::Common, "waveland_distance_threshold");
        fighter_pos.y += VarModule::get_float(fighter.object(), vars::common::ECB_Y_OFFSETS);
        VarModule::set_float(fighter.battle_object, vars::common::Y_POS, fighter_pos.y);
        VarModule::set_float(fighter.battle_object, vars::common::GET_DIST_TO_FLOOR, GroundModule::get_distance_to_floor(fighter.module_accessor, &fighter_pos, fighter_pos.y, true));
        let dist = VarModule::get_float(fighter.battle_object, vars::common::GET_DIST_TO_FLOOR);
        //println!("dist: {}", dist);
        if 0.0 <= dist && dist <= threshold {
            //println!("should_waveland");
            fighter_pos.y -= dist;
            PostureModule::set_pos(fighter.module_accessor, &fighter_pos);
        }
    }
}

// custom substatus for airdodges
#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon19sub_escape_air_uniqEN3lib8L2CValueE")]
unsafe extern "C" fn sub_escape_air_uniq(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if arg.get_bool() {
        if fighter.handle_waveland(true, true) {
            fighter.set_situation(L2CValue::I32(*SITUATION_KIND_GROUND));
            return 1.into();
        }
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        // if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO) {
        //     let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
        //     if WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("attack_air_lasso_enable_frame")) <= frame {
        //         WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO);
        //     }
        // }
    } else {
        let frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
        if frame <= WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("item_air_catch_frame_escape")) {
            fighter.sub_GetLightItemImm(L2CValue::Void());
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE)
            && ItemModule::is_have_item(fighter.module_accessor, 0)
            && frame <= WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("escape_throw_item_frame")) {
            fighter.sub_AIRChkDropItemImm();
        }
        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
                fighter.exec_escape_air_slide();
            }
            if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_FLAG_HIT_XLU) {
                    let stale_motion_rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
                    MotionModule::set_rate(fighter.module_accessor, stale_motion_rate);
                    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
                }
            }
            if StatusModule::is_changing(fighter.module_accessor) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_STIFF) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_END_STIFF)
                    && CancelModule::is_enable_cancel(fighter.module_accessor) {
                    MotionModule::set_rate(fighter.module_accessor, 1.0);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_END_STIFF);
                }
            } else if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_STIFF) {
                let start_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
                let frame = MotionModule::frame(fighter.module_accessor);
                let end_frame = MotionModule::end_frame(fighter.module_accessor);
                if 0.0 <= start_frame && start_frame <= frame {
                    let mut cancel_frame = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
                        WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_cancel_frame"))
                    } else {
                        WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_cancel_frame"))
                    };
                    if cancel_frame < 0.0 {
                        cancel_frame = end_frame;
                    }
                    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME) <= frame {
                        WorkModule::set_float(fighter.module_accessor, end_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
                    }
                    let stiff_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
                    let ratio = (cancel_frame - frame) / (stiff_frame - frame);
                    let new_rate = ratio * MotionModule::rate(fighter.module_accessor);
                    MotionModule::set_rate(fighter.module_accessor, new_rate);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_STIFF);
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_FALL) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL);
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
                    fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
                }
            }
            fighter.sub_fall_common_uniq(arg);
        }
    }
    0.into()
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon26sub_escape_air_common_mainEv")]
unsafe extern "C" fn sub_escape_air_common_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let id = VarModule::get_int(fighter.battle_object, vars::common::COSTUME_SLOT_NUMBER) as usize;
    let curr_frame = fighter.global_table[CURRENT_FRAME].get_i32();
    

    // RoA airdodge stalling
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        if StatusModule::is_changing(fighter.module_accessor) {
            let escape_air_slide_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("escape_air_slide_speed"), 0);
            
            VarModule::set_float(fighter.battle_object, vars::common::ESCAPE_AIR_SLIDE_SPEED_X, escape_air_slide_speed * WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X));
            VarModule::set_float(fighter.battle_object, vars::common::ESCAPE_AIR_SLIDE_SPEED_Y, escape_air_slide_speed * WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y));
        }
        else {
            if curr_frame < 29 {
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
                
                let x_speed = VarModule::get_float(fighter.battle_object, vars::common::ESCAPE_AIR_SLIDE_SPEED_X);
                let y_speed = VarModule::get_float(fighter.battle_object, vars::common::ESCAPE_AIR_SLIDE_SPEED_Y);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, (x_speed * 0.9), (y_speed * 0.9));
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

                if curr_frame == 28 {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                }

                VarModule::set_float(fighter.battle_object, vars::common::ESCAPE_AIR_SLIDE_SPEED_X, x_speed * 0.9);
                VarModule::set_float(fighter.battle_object, vars::common::ESCAPE_AIR_SLIDE_SPEED_Y, y_speed * 0.9);
            }
        }
    }
    else {
        if curr_frame >= 1 && curr_frame < 30 {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
        }
        if curr_frame == 30 {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
    
    /***println!("");
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	let control_speed_x = smash::app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    println!("control_speed_x: {}", control_speed_x);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
	let control_speed_y = smash::app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    println!("control_speed_y: {}", control_speed_y);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
	let stop_speed_x = smash::app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    println!("stop_speed_x: {}", stop_speed_x);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
	let stop_speed_y = smash::app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    println!("stop_speed_y: {}", stop_speed_y);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
	let gravity_speed_x = smash::app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    println!("gravity_speed_x: {}", gravity_speed_x);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
	let gravity_speed_y = smash::app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    println!("gravity_speed_y: {}", gravity_speed_y);

    println!("");

    println!("x speed: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
    println!("y speed: {}", KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL));
    ***/

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return L2CValue::Bool(true);
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
        || (!fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()){
        if fighter.sub_escape_air_common_strans_main().get_bool() {
            return L2CValue::Bool(true);
        }
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING),
                L2CValue::Bool(false)
            );
            return L2CValue::Bool(true);
        }
        if !MotionModule::is_end(fighter.module_accessor) {
            return L2CValue::Bool(false);
        } else {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                L2CValue::Bool(false)
            );
        }
    }
    L2CValue::Bool(true)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon33sub_escape_air_common_strans_mainEv")]
unsafe extern "C" fn sub_escape_air_common_strans_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("air_escape_passive_trigger_frame")) as f32;
    let curr_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_FRAME);
    let pad = fighter.global_table[PAD_FLAG].get_i32();
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
        && pad & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
        && ItemModule::is_have_item(fighter.module_accessor, 0)
        && curr_frame <= 5 {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
            smash::app::sv_module_access::item(fighter.lua_state_agent);
            let throwable = !fighter.pop_lua_stack(1).get_bool();
            if throwable {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_ITEM_THROW),
                    L2CValue::Bool(false)
                );
                let staling_mul = (1.0 - 0.1 * (VarModule::get_int(fighter.object(), vars::common::AGT_USED_COUNTER) as f32)).max(0.0);
                KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: staling_mul, y: staling_mul, z: staling_mul}, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
                VarModule::inc_int(fighter.object(), vars::common::AGT_USED_COUNTER);
                return 1.into();
        }
    }
    let lasso_type = WorkModule::get_param_int(fighter.module_accessor, hash40("air_lasso_type"), 0);
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO)
        && lasso_type != *FIGHTER_AIR_LASSO_TYPE_NONE
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && pad & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
        && !LinkModule::is_link(fighter.module_accessor, *FIGHTER_LINK_NO_CONSTRAINT) {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_AIR_LASSO),
            L2CValue::Bool(true)
        );
        return 1.into();
    }
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let passive_fb_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value"));
    // lazy eval gaurantees that we don't call handle_waveland if we are on the ground
    if situation_kind == *SITUATION_KIND_GROUND || fighter.handle_waveland(true, false) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
                && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
                && passive_fb_value <= stick_x.abs()
                && (curr_frame as f32) < trigger_frame {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE_FB),
                    L2CValue::Bool(true)
                );
                return 1.into();
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE)
                && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
                && (curr_frame as f32) < trigger_frame {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE),
                    L2CValue::Bool(false)
                );
                return 1.into();
            }
        }
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.set_situation(L2CValue::I32(*SITUATION_KIND_GROUND));
        }
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_AIR) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON)
            && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_LEFT) as u32)
            && (ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP as u8) as f32) < trigger_frame
            && (curr_frame as f32) < trigger_frame {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP),
                L2CValue::Bool(false)
            );
            return 1.into();
        }
        let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP)
            && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_LEFT) as u32)
            && jump_stick_y <= stick_y
            && (curr_frame as f32) < trigger_frame {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP),
                L2CValue::Bool(true)
            );
            return 1.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL)
            && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_LEFT) as u32)
            && (curr_frame as f32) < trigger_frame {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE_WALL),
                L2CValue::Bool(false)
            );
            return 1.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL)
            && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32)
            && (curr_frame as f32) < trigger_frame {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE_CEIL),
                L2CValue::Bool(false)
            );
            return 1.into();
        }
    }
    0.into()
}
