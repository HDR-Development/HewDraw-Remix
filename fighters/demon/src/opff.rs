// opff import
utils::import_noreturn!(common::opff::{fighter_common_opff});
use super::*;
use globals::*;

unsafe fn korean_back_dash(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_DEMON_STATUS_KIND_DASH_BACK)
    && boma.left_stick_y() < WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y")) {
        boma.change_status_req(*FIGHTER_STATUS_KIND_SQUAT, false);
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
    ])
    && boma.is_cat_flag(Cat1::TurnDash) && boma.left_stick_y() > WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y")) {
        boma.change_status_req(*FIGHTER_DEMON_STATUS_KIND_DASH_BACK, false);
    }
}

unsafe fn enable_both_recovery_specials(boma: &mut BattleObjectModuleAccessor) {
    // if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_LW_FALL]) && boma.is_situation(*SITUATION_KIND_AIR) {
    //     VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
    // }
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_AIR_END]) && boma.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    }
    // if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) && !VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL) {
    //     WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
    // }
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S) && !VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DEMON_STATUS_KIND_SPECIAL_N_AIR_SHOOT,
        *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_AIR_END,
        *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE,
        *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_FALL,
        ])
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

unsafe fn camera_lockout(fighter: &mut L2CFighterCommon) {
    let lockout = VarModule::get_int(fighter.battle_object, vars::demon::instance::CAMERA_LOCKOUT_TIMER);
    VarModule::set_int(fighter.battle_object, vars::demon::instance::CAMERA_LOCKOUT_TIMER, (lockout - 1).max(0));
}

unsafe fn check_step_cancel(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, vars::demon::status::ENABLE_STEP_CANCEL)
    && !fighter.global_table[IS_STOPPING].get_bool() {
        if fighter.is_cat_flag(Cat4::Command623NB)
        && ControlModule::get_special_command_lr(fighter.module_accessor, *FIGHTER_PAD_CMD_CAT4_COMMAND_623NB) == PostureModule::lr(fighter.module_accessor) {
            fighter.change_status(statuses::demon::CANCEL_STEP.into(), false.into());
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    korean_back_dash(boma);
    enable_both_recovery_specials(boma);
    fastfall_specials(fighter);
    camera_lockout(fighter);
    check_step_cancel(fighter);
}

pub extern "C" fn demon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		demon_frame(fighter)
    }
}

pub unsafe fn demon_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, demon_frame_wrapper);
}
