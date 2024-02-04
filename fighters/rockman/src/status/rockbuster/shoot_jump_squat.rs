use super::*;
use super::helper::*;


unsafe extern "C" fn rockman_rockbuster_shoot_jump_squat_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
            *FIGHTER_STATUS_ATTR_INTO_DOOR
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}


unsafe extern "C" fn rockman_rockbuster_shoot_jump_squat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_jump_life = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    if stick_jump_life == 0 || fighter.global_table[FLICK_Y_DIR].get_i32() <= 0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_BUTTON);
        if ControlModule::is_jump_mini_button(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_sub_x(fighter.module_accessor);
    fighter.global_table[FLICK_X].assign(&L2CValue::I32(0xFE));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    }
    rockman_rockbuster_main_helper(fighter, false.into(), true.into(), L2CValue::Void(), L2CValue::Void());
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("jump_squat"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        rockman_rockbuster_shoot_jump_squat_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(rockman_rockbuster_shoot_jump_squat_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(rockman_rockbuster_shoot_jump_squat_main_loop as *const () as _))
}

unsafe extern "C" fn rockman_rockbuster_shoot_jump_squat_substatus(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if !param_2.get_bool() {
        fighter.sub_jump_squat_uniq_check_sub(FIGHTER_ROCKMAN_STATUS_ROCKBUSTER_SHOOT_JUMP_WORK_ID_FLAG_BUTTON.into());
        fighter.sub_jump_squat_uniq_check_sub_mini_attack();
    }
    0.into()
}

unsafe extern "C" fn rockman_rockbuster_shoot_jump_squat_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let helper_ret = rockman_rockbuster_main_loop_helper(fighter, false.into(), true.into()).get_bool();
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    if sit == *SITUATION_KIND_AIR {
        let status = if helper_ret {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_AIR
        };
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if helper_ret {
            FIGHTER_STATUS_KIND_JUMP
        }
        else {
            FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP
        };
        fighter.change_status(status.into(), true.into());
        return 1.into();
    }
    0.into()
}


pub fn install() {
    smashline::Agent::new("rockman")
        .status(
            Pre,
            *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT,
            rockman_rockbuster_shoot_jump_squat_pre,
        )
        .status(
            Main,
            *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP_SQUAT,
            rockman_rockbuster_shoot_jump_squat_main,
        )
        .install();
}
