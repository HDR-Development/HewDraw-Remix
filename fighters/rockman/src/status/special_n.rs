use super::*;
use super::helper::*;
use super::super::vl;

#[status_script(agent = "rockman", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn rockman_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, vars::rockman::instance::CHARGE_SHOT_PLAYED_FX) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            let prev_escape = fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ESCAPE;
            WorkModule::set_flag(fighter.module_accessor, prev_escape, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_PREV_ESCAPE);
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_WAIT);
            return 1.into();
        }
        else {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_AIR);
            return 1.into();
        }
    }
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N |
            *FIGHTER_LOG_MASK_FLAG_SHOOT
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

#[status_script(agent = "rockman", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn rockman_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::rockman::status::CHARGE_SHOT_KEEP_CHARGE);
    let charge_frame = VarModule::get_int(fighter.battle_object, vars::rockman::instance::CHARGE_SHOT_FRAME);
    let top = charge_frame as f32 - vl::private::CHARGE_SHOT_DELAY_CHARGE_FRAME as f32;
    let bottom = vl::private::CHARGE_SHOT_MAX_FRAME as f32 - vl::private::CHARGE_SHOT_DELAY_CHARGE_FRAME as f32;
    let ratio = top / bottom;
    WorkModule::set_float(fighter.module_accessor, ratio, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE);
    fighter.sub_shift_status_main(L2CValue::Ptr(rockman_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn rockman_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return 0.into();
        }
    }
    if StatusModule::is_changing(fighter.module_accessor) || StatusModule::is_situation_changed(fighter.module_accessor) {
        rockman_special_motion_helper(
            fighter,
            hash40("buster_charge_shot").into(),
            hash40("buster_air_charge_shot").into(),
            FIGHTER_KINETIC_TYPE_MOTION.into(),
            FIGHTER_KINETIC_TYPE_MOTION_FALL.into(),
            FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_FIRST.into(),
            GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK.into()
        );
        if sit == *SITUATION_KIND_GROUND {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ATTACK_HI3_LANDING);
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if sit == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "rockman", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn rockman_special_n_end(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    install_status_scripts!(
        rockman_special_n_pre, rockman_special_n_main, rockman_special_n_end
    );
}