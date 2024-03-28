use super::*;

pub unsafe fn rockbuster_pre_helper(
    prev_status: L2CValue
) -> L2CValue {
    [
        *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT,
        *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK,
        *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK_BRAKE,
        *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN,
        *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT,
        *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP,
        *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_AIR,
        *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_LANDING
    ].contains(&prev_status.get_i32()).into()
}

pub unsafe fn rockbuster_main_helper(
    fighter: &mut L2CFighterCommon,
    is_air: L2CValue,
    is_jump_squat: L2CValue,
    was_shoot_walk: L2CValue,
    is_turn: L2CValue
) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    if status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        let mini_jump_attack = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT_MINI_JUMP_ATTACK);
        if 3 <= mini_jump_attack {
            WorkModule::set_int(fighter.module_accessor, 3, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
        }
    }
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
    let step_is_zero = step == 0;
    if step_is_zero {
        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP_PREVIOUS);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT);
        if status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            let mini_jump_attack = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT_MINI_JUMP_ATTACK);
            WorkModule::set_int(fighter.module_accessor, mini_jump_attack, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT);
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_LOOP_ACCEPT);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_LOOP);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT_END);
        ControlModule::reset_trigger(fighter.module_accessor);
        ControlModule::clear_command(fighter.module_accessor, false);
        let status_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
        if status_interrupt != *FIGHTER_STATUS_KIND_ATTACK_AIR {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, FIGHTER_LOG_ATTACK_KIND_ATTACK11, true);
            if !is_jump_squat.get_bool() {
                if is_air.get_bool() {
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01 - 1);
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01 - 1);
                }
            }
            else {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02 - 1);
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02 - 1);
            }
        }
    }
    if !is_turn.get_bool() {
        let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
        rockbuster_change_motion_helper(fighter, is_air.get_bool().into(), step.into(), (!step_is_zero).into(), is_jump_squat.get_bool().into(), was_shoot_walk.get_bool().into());
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT_MINI_JUMP_ATTACK);
}

pub unsafe fn rockbuster_main_loop_helper(
    fighter: &mut L2CFighterCommon,
    is_air: L2CValue,
    is_walk: L2CValue
) -> L2CValue {
    let mut step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
    if step == 0 {
        return true.into();
    }
    let mut ret = false;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_LOOP_ACCEPT) {
        // if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 { // Original
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_LOOP);
        }
    }
    let ude_motion = MotionModule::motion_kind_partial(fighter.module_accessor, *FIGHTER_ROCKMAN_MOTION_PART_SET_UDE);
    let is_end = if ude_motion == hash40("invalid") {
        MotionModule::is_end(fighter.module_accessor)
    }
    else {
        MotionModule::is_end_partial(fighter.module_accessor, *FIGHTER_ROCKMAN_MOTION_PART_SET_UDE)
    };
    if step == 1 {
        let mut allow_end = true;
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN != 0
        && rockbuster_can_turn_helper(fighter).get_bool() {
            allow_end = false;
        }
        if allow_end && is_end {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT_END) {
                rockbuster_change_motion_helper(fighter, is_air.get_bool().into(), 2.into(), false.into(), is_walk.get_bool().into(), L2CValue::Void());
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
                step = 2;
            }
            else {
                step = rockbuster_shoot_end_helper(fighter, step.into(), is_air.get_bool().into(), is_walk.get_bool().into()).get_i32();
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT_END);
            }
        }
    }
    else if step == 2 {
        if is_end {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT);
            step = rockbuster_shoot_end_helper(fighter, step.into(), is_air.get_bool().into(), is_walk.get_bool().into()).get_i32();
        }
    }
    else if step == 3 && is_end {
        step = 0;
        ret = true;
    }
    let step_orig = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
    if step != step_orig {
        WorkModule::set_int(fighter.module_accessor, step_orig, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP_PREVIOUS);
    }
    WorkModule::set_int(fighter.module_accessor, step, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
    ret.into()
}

pub unsafe fn rockbuster_can_turn_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
    let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT);
    if step == 1
    && count == 0
    && fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN {
        return true.into();
    }
    else {
        if step == 1 || step == 2 {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT)
            && count < 2 {
                return true.into();
            }
        }
    }
    false.into()
}

pub unsafe fn rockbuster_shoot_end_helper(
    fighter: &mut L2CFighterCommon,
    _step: L2CValue,
    is_air: L2CValue,
    is_walk: L2CValue
) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_PREV_ESCAPE);
    let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT);
    if count < 3 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_LOOP) {
            rockbuster_change_motion_helper(fighter, is_air.get_bool().into(), 2.into(), false.into(), is_walk.get_bool().into(), L2CValue::Void());
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
            ControlModule::reset_trigger(fighter.module_accessor);
            ControlModule::clear_command(fighter.module_accessor, false);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_LOOP);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT);
            return 2.into();
        }
    }
    rockbuster_change_motion_helper(fighter, is_air.get_bool().into(), 3.into(), false.into(), is_walk.get_bool().into(), L2CValue::Void());
    3.into()
}

pub unsafe fn rockbuster_change_motion_helper(
    fighter: &mut L2CFighterCommon,
    is_air: L2CValue,
    step: L2CValue,
    step_is_not_zero: L2CValue,
    is_walk: L2CValue,
    set_motion_partial: L2CValue
) {
    let mut mot = 0;
    let mut fix_shoulder = false;
    let step = step.get_i32();
    if is_walk.get_bool() {
        if step == 1 {
            mot = hash40("buster_walk_start");
        }
        else if step == 2 {
            mot = hash40("buster_walk");
            fix_shoulder = true;
        }
        else if step == 3 {
            mot = hash40("buster_walk_end");
        }
    }
    else {
        if !is_air.get_bool() {
            if step == 1 {
                mot = hash40("buster_wait_start");
            }
            else if step == 2 {
                mot = hash40("buster_wait");
            }
            else if step == 3 {
                mot = hash40("buster_wait_end");
            }
        }
        else {
            if step == 1 {
                mot = hash40("buster_air_start");
            }
            else if step == 2 {
                mot = hash40("buster_air");
            }
            else if step == 3 {
                mot = hash40("buster_air_end");
            }
        }
    }
    if !fix_shoulder {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_L_SHOULDER_FIX);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_GENERATE_TOP_N_OFFSET);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_L_SHOULDER_FIX);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_GENERATE_TOP_N_OFFSET);
    }
    let ude_motion = MotionModule::motion_kind_partial(fighter.module_accessor, *FIGHTER_ROCKMAN_MOTION_PART_SET_UDE);
    let is_invalid = ude_motion == hash40("invalid");
    let mut frame = 0.0;
    if step_is_not_zero.get_bool() {
        if is_invalid {
            frame = MotionModule::frame(fighter.module_accessor);
        }
        else {
            frame = MotionModule::frame_partial(fighter.module_accessor, *FIGHTER_ROCKMAN_MOTION_PART_SET_UDE);
        }
    }
    if !is_invalid {
        MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_ROCKMAN_MOTION_PART_SET_UDE, false);
    }
    if is_walk.get_bool() {
        MotionModule::add_motion_partial(
            fighter.module_accessor,
            *FIGHTER_ROCKMAN_MOTION_PART_SET_UDE,
            Hash40::new_raw(mot),
            frame,
            1.0,
            false,
            step_is_not_zero.get_bool(),
            0.0,
            true,
            true,
            false
        );
    }
    else if set_motion_partial.get_bool() {
        if !step_is_not_zero.get_bool() {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                frame,
                1.0,
                false,
                6.0,
                false,
                false
            );
        }
        else {
            if MotionModule::motion_kind(fighter.module_accessor) != mot {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new_raw(mot),
                    frame,
                    1.0,
                    6.0,
                    false,
                    false
                );
            }
        }
    }
    else if step_is_not_zero.get_bool() {
        if MotionModule::motion_kind(fighter.module_accessor) != mot {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                frame,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            frame,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
}

pub unsafe extern "C" fn rockbuster_walk_mot_helper(_fighter: &mut L2CFighterCommon, walk_mot: L2CValue, curr_mot: L2CValue) -> L2CValue {
    let curr = curr_mot.get_u64();
    let walk = walk_mot.get_u64();
    if curr == hash40("walk_fast")
    && walk == hash40("buster_walk_fast") {
        return true.into();
    }
    if curr == hash40("walk_middle")
    && walk == hash40("buster_walk_middle") {
        return true.into();
    }
    if curr == hash40("walk_slow")
    && walk == hash40("buster_walk_slow") {
        return true.into();
    }
    false.into()
}
