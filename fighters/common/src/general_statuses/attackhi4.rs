// status imports
use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    install_status_scripts!(
        status_end_AttackHi4Start
    );
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_attackhi4start_common,
            status_AttackHi4Start_Main
        );
    }
}

// FIGHTER_STATUS_KIND_ATTACK_HI4_START

#[skyline::hook(replace = L2CFighterCommon_status_pre_AttackHi4Start_common)]
unsafe fn status_pre_attackhi4start_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32,
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
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        param_1.get_i32() as u32,
        0
    );
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_AttackHi4Start_Main)]
unsafe fn status_AttackHi4Start_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sha_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);

    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
            L2CValue::Bool(false)
        );
        return L2CValue::I32(1);
    }

    if !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_DACUS) {
        if sha_frame > 0 && !StopModule::is_stop(fighter.module_accessor) {
            if fighter.sub_check_button_jump().get_bool() {
                let script_name = fighter.status_attack()[0xf40d7b92u64]["attack_hi4"].get_hash();
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, script_name, -1);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                fighter.change_status_jump_mini_attack(L2CValue::Bool(true));
                return L2CValue::I32(1);
            }
        }
    }
    let log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if sha_frame == 1 && !fighter.global_table[8].get_bool() && log_attack_kind > 0 {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log_attack_kind);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD),
                    L2CValue::Bool(true)
                );
                return L2CValue::I32(1);
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_ATTACK_HI4),
                L2CValue::Bool(false)
            );
            return L2CValue::I32(1);
        }
    }
    return 0.into()
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ATTACK_HI4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon25status_end_AttackHi4StartEv")]
unsafe fn status_end_AttackHi4Start(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_DACUS);
    fighter.status_end_AttackXX4Start();
    0.into()
}