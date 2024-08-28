use super::*;
use super::helper::*;

unsafe extern "C" fn rockbuster_shoot_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut status_data_arg = 0;
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
    if step != 0 {
        status_data_arg = 0;
    }
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let was_rockbuster_status = rockbuster_pre_helper(prev_status.into()).get_bool();
    let fs_succeeds_add = if was_rockbuster_status || prev_status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        *FS_SUCCEEDS_KEEP_SLOPE
    }
    else {
        0
    };
    let status_attr_add = if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_N {
        fighter.sub_status_pre_SpecialNCommon();
        *FIGHTER_STATUS_ATTR_START_TURN
    }
    else {
        0
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_OTTOTTO as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        *FS_SUCCEEDS_KEEP_VISIBILITY | fs_succeeds_add
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
            *FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT |
            status_attr_add
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        status_data_arg
    );
    0.into()
}

unsafe extern "C" fn rockbuster_shoot_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    let was_walk = fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK;
    rockbuster_main_helper(fighter, false.into(), false.into(), was_walk.into(), L2CValue::Void());
    fighter.sub_shift_status_main(L2CValue::Ptr(rockbuster_shoot_wait_main_loop as *const () as _))
}

unsafe extern "C" fn rockbuster_shoot_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let helper_ret = rockbuster_main_loop_helper(fighter, false.into(), false.into()).get_bool();
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if helper_ret
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT)
        && sit == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
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
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN != 0
        && !helper_ret
        && rockbuster_can_turn_helper(fighter).get_bool() {
            fighter.change_status(FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN.into(), true.into());
            return 1.into();
        }
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        let walk_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x"));
        if walk_stick_x < stick_x * lr {
            let status = if helper_ret {
                FIGHTER_STATUS_KIND_WALK
            }
            else {
                FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WALK
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
            *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT,
            rockbuster_shoot_wait_pre,
        );
    agent.status(
            Main,
            *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT,
            rockbuster_shoot_wait_main,
        );
}
