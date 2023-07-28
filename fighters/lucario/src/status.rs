use super::*;
use globals::*;
// status script import
mod special_n;

extern "C" {
    #[link_name = "\u{1}_ZN3app14LinkEventThrow13new_l2c_tableEv"]
    fn new_event_table() -> L2CValue;
    #[link_name = "\u{1}_ZN3app8lua_bind31LinkEvent__store_l2c_table_implEPKNS_9LinkEventE"]
    fn store_event_table(event: *const app::LinkEvent) -> L2CValue;
}

pub fn install() {
    special_n::install();
    install_status_scripts!(
        special_s_throw_pre,
        special_s_throw_main,
        pre_walk,
        pre_dash,
        pre_run,
        special_hi_bound_end,
        lucario_special_lw_pre,
        lucario_special_lw_init,
        lucario_special_lw_main,
    );
    smashline::install_agent_init_callbacks!(lucario_init);
}

#[smashline::fighter_init]
fn lucario_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if smash::app::utility::get_kind(&mut *fighter.module_accessor) != *FIGHTER_KIND_LUCARIO {
            return;
        }
        fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
        fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
        if fighter.kind() == *FIGHTER_KIND_LUCARIO {
            MeterModule::reset(fighter.battle_object);
            let meter_max = (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object()));
            MeterModule::add(fighter.battle_object, meter_max / 3.0);
            VarModule::off_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT);
            VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 10 * 60);
        }
    }
}

unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::lucario::instance::DISABLE_SPECIAL_LW) {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {

    // re-enable DSpecial when landing or hit
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL])
    {
        VarModule::off_flag(fighter.battle_object, vars::lucario::instance::DISABLE_SPECIAL_LW);
    }

    0.into()
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn lucario_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_FinalCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    return 0.into();
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucario_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_lw_main_loop as *const () as _))
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn lucario_special_lw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucario_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // check for cancels
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) 
        && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
        if fighter.is_situation(*SITUATION_KIND_AIR) 
        && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    // end
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into())
        } 
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into())
        }
    } else {
        // check if motion should be changed
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            if fighter.is_prev_situation(*SITUATION_KIND_AIR) {
                let frame = MotionModule::frame(fighter.module_accessor);
                if CancelModule::is_enable_cancel(fighter.module_accessor)
                || frame > 25.0 {
                    // WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                } else {
                    fighter.set_float(10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR.into(), false.into());
                }
                // MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
            }
        } else {
            VarModule::on_flag(fighter.battle_object, vars::lucario::instance::DISABLE_SPECIAL_LW);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            if fighter.is_prev_situation(*SITUATION_KIND_GROUND) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    0.into()
}

// FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW //

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_s_throw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s_throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

// FIGHTER_STATUS_KIND_WALK //

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_WALK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_walk(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Walk()
}

// FIGHTER_STATUS_KIND_DASH //

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Dash()
}

// FIGHTER_STATUS_KIND_RUN //

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Run()
}

// FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_hi_bound_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    
    ret
}
