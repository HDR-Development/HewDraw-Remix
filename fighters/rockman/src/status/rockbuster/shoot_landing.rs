use super::*;
use super::helper::*;

unsafe extern "C" fn rockman_rockbuster_shoot_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let was_rockbuster_status = rockman_rockbuster_pre_helper(prev_status.into()).get_bool();
    let fs_succeeds_add = if was_rockbuster_status || prev_status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        *FS_SUCCEEDS_KEEP_SLOPE
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
            *FIGHTER_STATUS_ATTR_INTO_DOOR
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn rockman_rockbuster_shoot_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    rockman_rockbuster_main_helper(fighter, true.into(), false.into(), L2CValue::Void(), L2CValue::Void());
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    fighter.sub_shift_status_main(L2CValue::Ptr(rockman_rockbuster_shoot_landing_main_loop as *const () as _))
}

unsafe extern "C" fn rockman_rockbuster_shoot_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let helper_ret = rockman_rockbuster_main_loop_helper(fighter, true.into(), false.into()).get_bool();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
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
    || fighter.sub_check_button_frick().get_bool() {
        let frame = MotionModule::frame(fighter.module_accessor);
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_frame"), 0);
        if landing_frame < frame {
            let status = if helper_ret {
                FIGHTER_STATUS_KIND_JUMP_SQUAT
            }
            else {
                FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT
            };
            fighter.change_status(status.into(), true.into());
            return 1.into()
        }
    }
    let status = if helper_ret {
        FIGHTER_STATUS_KIND_WAIT
    }
    else {
        FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT
    };
    fighter.change_status(status.into(), false.into());
    1.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(
            Pre,
            *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_LANDING,
            rockman_rockbuster_shoot_landing_pre,
        );
    agent.status(
            Main,
            *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_LANDING,
            rockman_rockbuster_shoot_landing_main,
        );
}
