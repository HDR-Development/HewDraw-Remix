use super::*;
use globals::*;
use smashline::*;

// FIGHTER_STATUS_KIND_SPECIAL_N

pub unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        hash40("special_n_start")
    }
    else {
        hash40("special_air_n_start")
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    fighter.set_situation(SITUATION_KIND_AIR.into());
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_N_FALL);
    if !StopModule::is_stop(fighter.module_accessor) {
        special_n_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(special_n_substatus as *const () as _));
    fighter.main_shift(special_n_main_loop)
}

unsafe extern "C" fn special_n_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_frame_coeff = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_attack_frame_coeff"));
    let ratio = 1.0 / attack_frame_coeff;
    let mut advance_counter = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_N_WORK_FLOAT_ADVANCE_COUNTER);
    advance_counter += ratio;
    WorkModule::set_float(fighter.module_accessor, advance_counter, *FIGHTER_SONIC_STATUS_SPECIAL_N_WORK_FLOAT_ADVANCE_COUNTER);
    let auto_attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_start_auto_attack_frame"));
    if auto_attack_frame as f32 <= advance_counter {
        fighter.global_table[SUB_STATUS2].assign(&L2CValue::I32(0));
        fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
    }
    0.into()
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_n_spin"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        return fighter.main_shift(special_n_main_loop2);
    }
    0.into()
}

unsafe extern "C" fn special_n_main_loop2(fighter: &mut L2CFighterCommon) -> L2CValue {
    let advance_counter = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_N_WORK_FLOAT_ADVANCE_COUNTER);
    let enable_attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_start_enable_attack_frame"));
    let auto_attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_start_auto_attack_frame"));
    let attack_frame_diff = advance_counter - enable_attack_frame as f32;
    let mut add_power = 0.0;
    if 0.0 < attack_frame_diff {
        let auto_frame_diff = auto_attack_frame - enable_attack_frame; 
        let ratio = attack_frame_diff / auto_frame_diff as f32;
        add_power = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_add_attack_power"));
        add_power *= ratio;
    }
    WorkModule::set_float(fighter.module_accessor, add_power, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_ADD_ATTACK_POWER);
    if auto_attack_frame as f32 <= advance_counter {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START.into(), false.into());
        return 0.into();
    }
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    if 5.0 <= advance_counter {
        if pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_ADD_ATTACK_POWER);
            VarModule::on_flag(fighter.battle_object, vars::sonic::status::SPECIAL_N_BLAST_ATTACK);
            VarModule::on_flag(fighter.battle_object, vars::sonic::instance::USED_AIR_ACTION);
            fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START.into(), true.into());
        }
    }
    if enable_attack_frame as f32 <= advance_counter {
        if pad_flag & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
            fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START.into(), true.into());
        }
    }
    0.into()
}

// FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START

pub unsafe extern "C" fn special_n_homing_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let blast_attack = VarModule::is_flag(fighter.battle_object, vars::sonic::status::SPECIAL_N_BLAST_ATTACK);
    let ret = smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    VarModule::set_flag(fighter.battle_object, vars::sonic::status::SPECIAL_N_BLAST_ATTACK, blast_attack);
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    
    agent.status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START, special_n_homing_start_pre);
}
