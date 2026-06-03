use super::*;
use globals::*;
use smash::app::smashball::*;

utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn special_n_hold(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) {
        if fighter.check_hold_input(0, 8, Buttons::SpecialAll) {
            VarModule::on_flag(fighter.battle_object, vars::cloud::status::SPECIAL_N_HOLD);
        }
    }
}

unsafe fn dspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_INT_CANCEL_STATUS);
    }
}

// Fixes bug where touching ground during upB puts you into special fall for 1f before landing
unsafe fn up_special_proper_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL);
    }
}

unsafe fn training_mode_limit(fighter: &mut L2CFighterCommon) {
    if !is_training_mode() { return; }

    let limit = fighter.get_float(*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
    if fighter.is_status(*FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE) 
    && fighter.is_button_on(Buttons::AppealAll)
    && limit != 100.0 {
        fighter.set_float(100.0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2_FALL,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe extern "C" fn cloud_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);

    special_n_hold(fighter);
    dspecial_cancels(fighter);
    up_special_proper_landing(fighter);
    training_mode_limit(fighter);
    fastfall_specials(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, cloud_frame_wrapper);
}