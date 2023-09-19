use super::*;
use globals::*;

// This file contains code for smash attacks

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    install_status_scripts!(
        status_end_AttackHi4Start,
        status_end_AttackLw4Start,
    );
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_end_attackxx4start,
            //status_AttackHi4Start,
            status_AttackHi4Start_Main,
            //status_AttackHi4Start_Common,
            status_AttackLw4Start_Main,
        );
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackXX4Start)]
unsafe fn status_end_attackxx4start(fighter: &mut L2CFighterCommon) {
    let restart_frame = MotionModule::frame(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, restart_frame, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            0.0,
            0.0
        );
    }
}

// FIGHTER_STATUS_KIND_ATTACK_HI4_START

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
    else {
        // This should only reduce the highest speed the dacus will reach (the first frame); it should still decrease at the same rate as those with a 1.0 mul
        if StatusModule::is_changing(fighter.module_accessor) {
            let spdx = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = VarModule::get_float(fighter.object(), vars::common::instance::DACUS_TRANSITION_SPEED);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
            let spx = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
        // let mut speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        // println!("DACUS MOTION: {}", speed_x);
        // speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_STOP);
        // println!("DACUS STOP: {}", speed_x);
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

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_AttackHi4Start)]
unsafe fn status_AttackHi4Start(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = Hash40::new("attack_hi4");
    fighter.status_AttackHi4Start_common(L2CValue::Hash40(motion));
    return 0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_AttackHi4Start_common)]
unsafe fn status_AttackHi4Start_Common(fighter: &mut L2CFighterCommon, motion: L2CValue) {
    fighter.sub_status_AttackHi4Start_common(motion);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackHi4Start_Main as *const () as _));
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ATTACK_HI4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon25status_end_AttackHi4StartEv")]
unsafe fn status_end_AttackHi4Start(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_DACUS);
    fighter.status_end_AttackXX4Start();
    0.into()
}

// FIGHTER_STATUS_KIND_ATTACK_LW4_START

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_AttackLw4Start_Main)]
unsafe fn status_AttackLw4Start_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
                let script_name = fighter.status_attack()[0xf40d7b92u64]["attack_lw4"].get_hash();
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, script_name, -1);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                fighter.change_status_jump_mini_attack(L2CValue::Bool(true));
                return L2CValue::I32(1);
            }
        }
    }
    else {
        // This should only reduce the highest speed the dacds will reach (the first frame); it should still decrease at the same rate as those with a 1.0 mul
        if StatusModule::is_changing(fighter.module_accessor) {
            //let spdx = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_MOTION);
            //println!("old motion speed: {}", spdx);
            let speed_x = VarModule::get_float(fighter.object(), vars::common::instance::DACUS_TRANSITION_SPEED);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_x, 0.0);
            //let spx = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_MOTION);
            //println!("new motion speed: {}", spx);
        }
        // Account for added STOP energy (only if the dacds has a non-default multiplier)
        let dacds_mul = ParamModule::get_float(fighter.object(), ParamType::Shared, "dacds_mul");
        if dacds_mul != 1.0 {
            let mut stop = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_STOP);
            if stop.abs() > 0.0 {
                //println!("extra stop energy: {}", stop);
                let min_speed = ParamModule::get_float(fighter.battle_object, ParamType::Common, "dacus.min_speed");
                let max_speed = ParamModule::get_float(fighter.battle_object, ParamType::Common, "dacus.max_speed");
                if !(min_speed..max_speed).contains(&stop.abs()) {
                    stop = stop.abs().clamp(min_speed, max_speed) * PostureModule::lr(fighter.module_accessor);
                }
                stop = stop * dacds_mul;
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, stop, 0.0);
            }
        }
        // let mut speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_MOTION);
        // println!("DACDS MOTION: {}", speed_x);
        // speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_STOP);
        // println!("DACDS STOP: {}", speed_x);
    }
    let log_attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if sha_frame == 1 && !fighter.global_table[8].get_bool() && log_attack_kind > 0 {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log_attack_kind);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD),
                    L2CValue::Bool(true)
                );
                return L2CValue::I32(1);
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_ATTACK_LW4),
                L2CValue::Bool(false)
            );
            return L2CValue::I32(1);
        }
    }
    return 0.into()
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ATTACK_LW4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon25status_end_AttackLw4StartEv")]
unsafe fn status_end_AttackLw4Start(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_DACUS);
    fighter.status_end_AttackXX4Start();
    0.into()
}