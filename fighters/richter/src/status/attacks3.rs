use super::*;
use globals::*;


// FIGHTER_STATUS_KIND_ATTACK_S3

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS3Common();
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_HOLD_INPUT);
    fighter.main_shift(attack_s3_main_loop)
}

unsafe extern "C" fn attack_s3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // <HDR>
    if fighter.is_motion(Hash40::new("attack_squat_s3"))
    && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SQUAT_WAIT, true);
        return 1.into();
    }
    // </HDR>

    if fighter.status_AttackS3_Main().get_bool() {
        // <HDR>
        if StatusModule::status_kind_next(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SQUAT {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_SQUAT_WAIT, true);
        }
        // </HDR>
        return 1.into();
    }

    // <HDR>
    if StatusModule::is_changing(fighter.module_accessor)
    && fighter.stick_y() <= -0.5 {
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_squat_s3"), 0.0, 1.0, false, 0.0, false, false);
    }
    // </HDR>
    
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_HOLD_INPUT);
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_HOLD_INPUT)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_HOLD) {
        fighter.change_status_req(*FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD_START, false);
        return 1.into();
    }

    0.into()
}

pub fn install() {
    install_status_scripts!(
        attack_s3_main
    );
}