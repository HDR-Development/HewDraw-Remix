use super::*;
use globals::*;
// status script import

// FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW //

unsafe extern "C" fn special_s_throw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE,
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        true,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0,
    );
    L2CValue::I32(0)
}

unsafe extern "C" fn special_s_throw_main_side(fighter: &mut L2CFighterCommon, should_run: L2CValue) -> L2CValue {
    if should_run.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_INT_FRAME);
    }
    // PostureModule::set_pos(fighter.module_accessor, &Vector3f{ x: 0.0, y: 20.0, z: 0.0 });
    L2CValue::I32(0)
}

unsafe extern "C" fn special_s_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let rot = VarModule::get_int(fighter.battle_object, vars::lucario::status::FORCE_PALM_ROT_ANGLE) as f32;
    let rot_start_interpolate_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "force_palm_air.rot_start_interpolate_start_frame");
    let rot_start_interpolate_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "force_palm_air.rot_start_interpolate_end_frame");
    let rot_end_interpolate_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "force_palm_air.rot_end_interpolate_start_frame");
    let rot_end_interpolate_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "force_palm_air.rot_end_interpolate_end_frame");

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        && fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() {
            return L2CValue::I32(0);
        }
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
        && fighter.sub_air_check_fall_common().get_bool() {
            return L2CValue::I32(0);
        }
    }

    // linear interpolation of model rotation to match throw angle
    if frame >= rot_start_interpolate_start_frame && frame < rot_start_interpolate_end_frame {
        let calc_interpolated_rot = rot * (1.0 - (rot_start_interpolate_end_frame - frame) / (rot_start_interpolate_end_frame - rot_start_interpolate_start_frame));
        fighter.set_joint_rotate("rot", Vector3f{x: calc_interpolated_rot, y: 0.0, z: 0.0});
    }
    // keep model rotated at throw angle
    if frame >= rot_start_interpolate_end_frame && frame < rot_end_interpolate_start_frame {
        fighter.set_joint_rotate("rot", Vector3f{x: rot, y: 0.0, z: 0.0});
    }
    // linear interpolation of model rotation back to normal
    if frame >= rot_end_interpolate_start_frame && frame < rot_end_interpolate_end_frame {
        let calc_interpolated_rot = rot * (1.0 - (frame - rot_end_interpolate_start_frame) / (rot_end_interpolate_end_frame - rot_end_interpolate_start_frame));
        fighter.set_joint_rotate("rot", Vector3f{x: calc_interpolated_rot, y: 0.0, z: 0.0});
    }

    if !MotionModule::is_end(fighter.module_accessor) {
        let frame = MotionModule::frame(fighter.module_accessor);
        // Uncommenting this block will cause lucario to go into special fall immediately upon grab
        // if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
        // && WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_INT_FRAME) <= 1 {
        //     fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        //     return L2CValue::I32(0);
        // }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_CRITICAL_HIT) {
            if WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("critical_aura_power")) <= WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER) {
                let mut pos = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
                ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("throw"), &mut pos, true);
                let node_obj_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
                app::FighterUtil::request_critical_hit_cut_in_force(fighter.module_accessor, node_obj_id as u32, &Vector2f{ x: pos.x, y: pos.y }, *FIGHTER_KIND_LUCARIO, Hash40::new("param_special_s"), *LINK_NO_NONE, true, 0, true);
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_CRITICAL_HIT);
        }

        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_THROW_DONE)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW) {
            let mut event = new_event_table();
            event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("throw")));
            let callable: extern "C" fn() -> *mut app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
            let link_event = callable();
            lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
            LinkModule::send_event_nodes_struct(fighter.module_accessor, *LINK_NO_CAPTURE, link_event, 0);
            let mem = std::slice::from_raw_parts(link_event as *const u64, 0x50 / 8);
            event = store_event_table(link_event);
            let deleter: extern "C" fn(*mut app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
            deleter(link_event);

            let node_obj_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
            WorkModule::set_int(fighter.module_accessor, node_obj_id as i32, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            WorkModule::set_int(fighter.module_accessor, event["hit_group_"].get_i32(), *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            WorkModule::set_int(fighter.module_accessor, event["hit_no_"].get_i32(), *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            WorkModule::set_float(fighter.module_accessor, event["motion_rate_"].get_f32(), *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_THROW_DONE); 
        }
        
        if !StatusModule::is_changing(fighter.module_accessor)
        && ((fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR)
        || (fighter.global_table[PREV_SITUATION_KIND] != SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND)) {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
                } else {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0);
                }
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            } else {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
                } else {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0);
                }
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            }
        }
    } else {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

unsafe extern "C" fn special_s_throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_s_throw") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    } else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_throw") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    }
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_INT_FRAME);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
        } else {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0);
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    } else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
        } else {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0);
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::suspend_energy_all(fighter.module_accessor);
        KineticModule::clear_speed_all(fighter.module_accessor);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s_throw_main_side as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_throw_main_loop as *const () as _))
}

pub fn install() {
    smashline::Agent::new("lucario")
        .status(Pre, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW, special_s_throw_pre)
        .status(Main, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW, special_s_throw_main)
        .install();
}