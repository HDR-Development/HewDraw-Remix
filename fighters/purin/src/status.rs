use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        special_lw
    );
}

// FIGHTER_STATUS_KIND_SPECIAL_LW

#[status_script(agent = "purin", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_lw(fighter: &mut L2CFighterCommon) -> L2CValue {
    if PostureModule::lr(fighter.module_accessor) != 1.0 {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_l") as i64, *FIGHTER_PURIN_STATUS_SPECIAL_LW_WORK_INT_MOTION_KIND_GROUND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_l") as i64, *FIGHTER_PURIN_STATUS_SPECIAL_LW_WORK_INT_MOTION_KIND_AIR);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_r") as i64, *FIGHTER_PURIN_STATUS_SPECIAL_LW_WORK_INT_MOTION_KIND_GROUND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_r") as i64, *FIGHTER_PURIN_STATUS_SPECIAL_LW_WORK_INT_MOTION_KIND_AIR);
    }
    special_lw_situation_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main as *const () as _))
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_LW_FLAG_HIT)
    && (
        !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    )
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_LW_FLAG_HIT_CANCEL_OK) {
        let frame = fighter.global_table[CURRENT_FRAME].get_i32();
        let cancel_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_LW_WORK_INT_ENABLE_HIT_CANCEL_FRAME);

        if frame == cancel_frame - 30 {
            // Skip to wake-up animation, 30 frames before on-hit FAF
            // Wake-up anim lasts 30 frames
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 179.0, true, true, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_LW_FLAG_HIT_CANCEL_OK);
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_lw_situation_helper(fighter);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                L2CValue::Bool(false)
            );
            return 1.into()
        }
        else {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_WAIT),
                L2CValue::Bool(false)
            );
            return 1.into()
        }
    }
    return 0.into()
}

unsafe extern "C" fn special_lw_situation_helper(fighter: &mut L2CFighterCommon) {
    let special_lw_mot_rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_LW_WORK_FLOAT_MOTION_RATE);

    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        let special_air_lw_mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_LW_WORK_INT_MOTION_KIND_AIR);

        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(special_air_lw_mot_kind), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(special_air_lw_mot_kind), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::set_rate(fighter.module_accessor, special_lw_mot_rate);
    }
    else {
        let special_lw_mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_LW_WORK_INT_MOTION_KIND_GROUND);

        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(special_lw_mot_kind), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PURIN_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(special_lw_mot_kind), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::set_rate(fighter.module_accessor, special_lw_mot_rate);
    }
}