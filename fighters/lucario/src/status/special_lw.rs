use super::*;
use globals::*;
// status script import

pub fn install() {
    install_status_scripts!(
        lucario_special_lw_pre,
        lucario_special_lw_init,
        lucario_special_lw_main,
    );
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn lucario_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_FinalCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    return 0.into();
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucario_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_lw_main_loop as *const () as _))
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn lucario_special_lw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucario_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // check for cancels
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) 
        && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
        if fighter.is_situation(*SITUATION_KIND_AIR) 
        && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    // end
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into())
        } 
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into())
        }
    } else {
        // check if motion should be changed
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            if fighter.is_prev_situation(*SITUATION_KIND_AIR) {
                let frame = MotionModule::frame(fighter.module_accessor);
                if CancelModule::is_enable_cancel(fighter.module_accessor)
                || frame > 25.0 {
                    // WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                } else {
                    fighter.set_float(10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR.into(), false.into());
                }
                // MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
            }
        } else {
            VarModule::on_flag(fighter.battle_object, vars::lucario::instance::DISABLE_SPECIAL_LW);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            if fighter.is_prev_situation(*SITUATION_KIND_GROUND) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    0.into()
}

