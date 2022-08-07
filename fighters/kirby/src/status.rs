use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        pre_jump,
        throw_kirby_map_correction
    );
}

// FIGHTER_STATUS_KIND_JUMP //

#[status_script(agent = "kirby", status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_PICKEL {
            if fighter.status_pre_Jump_Common_param(L2CValue::Bool(true)).get_bool() {
                return 1.into()
            }
            else {
                if kirby_pickel_jump_status_check(fighter).get_bool() {
                    fighter.status_pre_Jump_sub_param(
                        L2CValue::I32(-1),
                        L2CValue::I32(-1),
                        L2CValue::I32(-1),
                        L2CValue::I32(*KINETIC_TYPE_NONE),
                        L2CValue::I32(*FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_SOUND | *FS_SUCCEEDS_KEEP_TRANSITION | *FS_SUCCEEDS_KEEP_CANCEL)
                    );
                }
                else {
                    fighter.status_pre_Jump_sub_param(
                        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLAG),
                        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_INT),
                        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLOAT),
                        L2CValue::I32(*FIGHTER_KINETIC_TYPE_JUMP),
                        L2CValue::I32(0)
                    );
                }
                return 0.into()
            }
        }
    }
    fighter.status_pre_Jump()
}

unsafe extern "C" fn kirby_pickel_jump_status_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_KIRBY_STATUS_KIND_PICKEL_SPECIAL_N1_JUMP && fighter.global_table[PREV_STATUS_KIND] != FIGHTER_KIRBY_STATUS_KIND_PICKEL_SPECIAL_N3_JUMP {
        return L2CValue::Bool(false);
    }
    else {
        return L2CValue::Bool(true);
    }
}

// FIGHTER_STATUS_KIND_THROW_KIRBY //

#[status_script(agent = "kirby", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_MAP_CORRECTION)]
pub unsafe fn throw_kirby_map_correction(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
    let frame = MotionModule::frame(fighter.module_accessor);
    let prev_frame = MotionModule::prev_frame(fighter.module_accessor);
    let fall_start_frame = if motion_kind == hash40("throw_b") { 28.0 } else { 32.0 };
    let fall_stop_frame = if motion_kind == hash40("throw_b") { 30.0 } else { 35.0 };
    let landing_frame = if motion_kind == hash40("throw_b") { 31.0 } else { 36.0 };
    let return_air_frame = if motion_kind == hash40("throw_b") { 40.0 } else { 44.0 };
    
    if (motion_kind != hash40("throw_b") && motion_kind != hash40("throw_f"))
    || frame <= fall_start_frame
    {
        return 0.into()
    }
    if prev_frame < return_air_frame && frame >= return_air_frame {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        if prev_frame < fall_stop_frame && frame >= fall_stop_frame {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::set_frame(fighter.module_accessor, fall_stop_frame, true);
            MotionModule::set_rate(fighter.module_accessor, 0.0);
            LinkModule::send_event_nodes_throw(fighter.module_accessor, Hash40::new("throw_sync_motion"), Hash40::new("invalid"), true, 0, 0, 0);
        }
    }
    else {
        if frame < landing_frame {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::set_frame(fighter.module_accessor, landing_frame, true);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            LinkModule::send_event_nodes_throw(fighter.module_accessor, Hash40::new("throw_sync_motion"), Hash40::new("invalid"), true, 0, 0, 0);
        }
    }
    0.into()
}