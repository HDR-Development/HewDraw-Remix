use super::*;
use globals::*;

// This file contains code for jab locks

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    skyline::install_hooks!(
        get_down_stand_fb_kind_hook
    );
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_DownDamage_Main
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DownDamage_Main)]
unsafe fn status_DownDamage_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StopModule::is_stop(fighter.module_accessor) {
        if fighter.is_cat_flag(Cat2::DownToDownStandFB) {
            VarModule::on_flag(fighter.battle_object, vars::common::status::IS_JAB_LOCK_ROLL);
            VarModule::set_int(fighter.battle_object, vars::common::status::DOWN_STAND_FB_KIND, ControlModule::get_down_stand_fb_kind(fighter.module_accessor) as i32);
        }
        else {
            VarModule::off_flag(fighter.battle_object, vars::common::status::IS_JAB_LOCK_ROLL);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DOWN_DAMAGE_FLAG_START_AIR)
    || (!WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DOWN_DAMAGE_FLAG_START_AIR)
        && (!WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) || fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR)) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL)
        && MotionModule::is_end(fighter.module_accessor) 
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_DAMAGE_FALL),
                L2CValue::Bool(false)
            );
            return 0.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN_STAND)
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        && MotionModule::is_end(fighter.module_accessor)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT)
        && WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_DOWN_WORK_FLOAT_DOWN_FRAME) <= 0.0 {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_DOWN_STAND),
                L2CValue::Bool(false)
            );
            return 0.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN_WAIT)
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        && MotionModule::is_end(fighter.module_accessor) {
            /***fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE),
                L2CValue::Bool(false)
            );
            return 0.into();
            ***/
            if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_JAB_LOCK_ROLL) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_DOWN_STAND_FB),
                    L2CValue::Bool(true)
                );
                return 0.into();
            }
            else {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_DOWN_STAND),
                    L2CValue::Bool(true)
                );
                return 0.into();
            }
        }
        fighter.sub_down_chk_reflect_wall();
        if !StatusModule::is_changing(fighter.module_accessor) {
            if (fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND && fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND)
            || fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR || fighter.global_table[PREV_SITUATION_KIND] != SITUATION_KIND_GROUND {
                    return 0.into();
                }
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DOWN_DAMAGE_FLAG_SITUATION_CHANGE);
        }
        if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DOWN_DAMAGE_FLAG_SITUATION_CHANGE) {
                return 0.into();
            }
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DOWN_DAMAGE_FLAG_SITUATION_CHANGE) {
                return 0.into();
            }
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_DAMAGE_FALL),
                L2CValue::Bool(false)
            );
            return 0.into();
        }
        else {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_MISS_FOOT),
                L2CValue::Bool(false)
            );
            return 0.into();
        }
    }
    return 0.into()
}

#[skyline::hook(replace = ControlModule::get_down_stand_fb_kind)]
unsafe fn get_down_stand_fb_kind_hook(boma: &mut BattleObjectModuleAccessor) -> u64 {
    if boma.is_prev_status(*FIGHTER_STATUS_KIND_DOWN_DAMAGE) {
        return VarModule::get_int(boma.object(), vars::common::status::DOWN_STAND_FB_KIND) as u64;
    }
    original!()(boma)
}