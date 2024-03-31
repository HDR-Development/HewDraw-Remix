use super::*;
use super::helper::*;

unsafe extern "C" fn rockbuster_shoot_walk_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let keep_flag;
    let keep_int;
    let keep_float;
    if prev_status == *FIGHTER_STATUS_KIND_WALK {
        keep_flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG;
        keep_int = *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT;
        keep_float = *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT;
    }
    else {
        keep_flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG;
        keep_int = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT;
        keep_float = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT;
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_OTTOTTO as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        keep_flag,
        keep_int,
        keep_float,
        *FS_SUCCEEDS_KEEP_VISIBILITY
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N |
            *FIGHTER_LOG_MASK_FLAG_SHOOT
        ) as u64,
        (
            *FIGHTER_STATUS_ATTR_SCALE_KINETIC_ENERGY |
            *FIGHTER_STATUS_ATTR_INTO_DOOR |
            *FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn rockbuster_shoot_walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if prev_status != *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT {
        fighter.sub_GetLightItemImm(L2CValue::Void());
        if StatusModule::status_kind_que_from_script(fighter.module_accessor) as i32 != *FIGHTER_STATUS_KIND_NONE {
            return 0.into();
        }
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    rockbuster_main_helper(fighter, false.into(), true.into(), L2CValue::Void(), L2CValue::Void());
    if prev_status != *FIGHTER_STATUS_KIND_WALK {
        let walk_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_speed_max"), 0);
        fighter.init_walk_speed(
            walk_speed_max.into(),
            FIGHTER_STATUS_WALK_WORK_FLOAT_SPEED.into(),
            1.0f32.into(),
            false.into(),
            hash40("buster_walk_fast").into(),
            hash40("buster_walk_middle").into(),
            hash40("buster_walk_slow").into()
        );
        fighter.init_walk_motion(
            hash40("buster_walk_fast").into(),
            hash40("buster_walk_middle").into(),
            hash40("buster_walk_slow").into(),
            FIGHTER_STATUS_WALK_WORK_FLOAT_SPEED.into(),
            1.0f32.into()
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(rockbuster_shoot_walk_main_loop as *const () as _))
}

unsafe extern "C" fn rockbuster_shoot_walk_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let walk_speed = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_WALK_WORK_FLOAT_SPEED);
    let walk_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_speed_max"), 0);
    let walk_accel_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_accel_mul"), 0);
    let walk_accel_add = WorkModule::get_param_float(fighter.module_accessor, hash40("walk_accel_add"), 0);
    let walk_accel_mul_common = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_accel_mul"));
    fighter.calc_walk_speed(
        walk_speed.into(),
        walk_speed_max.into(),
        walk_accel_mul.into(),
        walk_accel_add.into(),
        0.0f32.into(),
        walk_accel_mul_common.into(),
        FIGHTER_STATUS_WALK_WORK_FLOAT_SPEED.into(),
        1.0f32.into(),
        true.into()
    );
    fighter.set_walk_motion(
        hash40("buster_walk_fast").into(),
        hash40("buster_walk_middle").into(),
        hash40("buster_walk_slow").into(),
        FIGHTER_STATUS_WALK_WORK_FLOAT_SPEED.into(),
        1.0f32.into(),
        L2CValue::Ptr(rockbuster_walk_mot_helper as *const () as _)
    );

    let helper_ret = rockbuster_main_loop_helper(fighter, false.into(), true.into()).get_bool();
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if helper_ret
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK)
        && sit == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WALK.into(), false.into());
            return 0.into();
        }
        if sit == *SITUATION_KIND_AIR {
            let status = if helper_ret {
                FIGHTER_STATUS_KIND_FALL
            }
            else {
                FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_AIR
            };
            fighter.change_status(status.into(), false.into());
            return 0.into();
        }
        if fighter.sub_check_button_jump().get_bool()
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_PREV_ESCAPE) {
            let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT);
            WorkModule::set_int(fighter.module_accessor, count + 1, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT_MINI_JUMP_ATTACK);
            let status = if helper_ret {
                FIGHTER_STATUS_KIND_JUMP_SQUAT
            }
            else {
                FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT
            };
            fighter.change_status_jump_mini_attack_common(status.into(), true.into());
            return 1.into();
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_PREV_ESCAPE) {
            if fighter.sub_check_button_jump().get_bool()
            || fighter.sub_check_button_frick().get_bool() {
                let status = if helper_ret {
                    FIGHTER_STATUS_KIND_JUMP_SQUAT
                }
                else {
                    FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT
                };
                fighter.change_status(status.into(), true.into());
                return 1.into();
            }
        }
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        let walk_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x"));
        if stick_x * lr < walk_stick_x {
            let status = if helper_ret {
                FIGHTER_STATUS_KIND_WAIT
            }
            else {
                FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT
            };
            fighter.change_status(status.into(), true.into());
            return 1.into();
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(
            Pre,
            *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK,
            rockbuster_shoot_walk_pre,
        );
    agent.status(
            Main,
            *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK,
            rockbuster_shoot_walk_main,
        );
}
