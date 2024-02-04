use super::*;
use super::helper::*;


unsafe extern "C" fn rockman_rockbuster_shoot_turn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_TURN,
        *GROUND_CORRECT_KIND_GROUND_OTTOTTO as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        true,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N
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


unsafe extern "C" fn rockman_rockbuster_shoot_turn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    rockman_rockbuster_main_helper(fighter, false.into(), false.into(), false.into(), true.into());
    PostureModule::reverse_lr(fighter.module_accessor);
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
    if step == 2 {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT);
    rockman_rockbuster_main_helper(fighter, false.into(), false.into(), false.into(), true.into());
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_LOOP);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_L_SHOULDER_FIX);
    let ude_motion = MotionModule::motion_kind_partial(fighter.module_accessor, *FIGHTER_ROCKMAN_MOTION_PART_SET_UDE);
    if ude_motion != hash40("invalid") {
        MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_ROCKMAN_MOTION_PART_SET_UDE, false);
    }
    WorkModule::set_int(fighter.module_accessor, step, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP_PREVIOUS);
    WorkModule::set_int(fighter.module_accessor, 4, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("buster_wait_turn"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(rockman_rockbuster_shoot_turn_main_loop as *const () as _))
}

unsafe extern "C" fn rockman_rockbuster_shoot_turn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let helper_ret = false;
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if sit == *SITUATION_KIND_AIR {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP_PREVIOUS);
        let status = if helper_ret {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_AIR
        };
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }
    if sit == *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("attack1"));
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("buster_wait"),
                end_frame,
                1.0,
                0.0,
                false,
                false
            );
            let prev_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP_PREVIOUS);
            if prev_step == 2 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT_END);
            }
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_STEP);
            let status = if helper_ret {
                FIGHTER_STATUS_KIND_WAIT
            }
            else {
                FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT
            };
            fighter.change_status(status.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}


pub fn install() {
    smashline::Agent::new("rockman")
        .status(
            Pre,
            *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN,
            rockman_rockbuster_shoot_turn_pre,
        )
        .status(
            Main,
            *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_TURN,
            rockman_rockbuster_shoot_turn_main,
        )
        .install();
}
