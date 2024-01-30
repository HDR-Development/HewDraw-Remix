use super::*;
mod special_n;

#[status_script(agent = "luigi", status = FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn special_s_charge_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CHARGE_MELEE_NO_RANDOM) {
        let should_do_effect = if VarModule::is_flag(fighter.battle_object, vars::luigi::instance::IS_MISFIRE_STORED) {
            VarModule::off_flag(fighter.battle_object, vars::luigi::instance::IS_MISFIRE_STORED);
            true
        } else if VarModule::countdown_int(fighter.battle_object, vars::luigi::instance::REMAINING_SPECIAL_S_UNTIL_MISFIRE, 0) {
            super::calculate_misfire_number(fighter);
            true
        } else {
            false
        };
        if should_do_effect {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FLASHING);
            EffectModule::req_common(fighter.module_accessor, Hash40::new("charge_max"), 0.0) as u32;
        }
    }
    0.into()
}

unsafe extern "C" fn special_s_charge_uniq_process(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    let charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));

    if charge_frame <= charge {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FLASHING) {
            EffectModule::req_common(fighter.module_accessor, Hash40::new("charge_max"), 0.0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FLASHING);
        }
        MotionModule::set_rate(fighter.module_accessor, 0.0);
    }

    WorkModule::add_float(fighter.module_accessor, WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_speed_mul")), *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    0.into()
}

unsafe fn special_s_charge_motion_check(fighter: &mut L2CFighterCommon) {
    let (kinetic, gc_kind, motion, mtrans) = if fighter.global_table[globals::SITUATION_KIND] == SITUATION_KIND_GROUND {
        (*FIGHTER_KINETIC_TYPE_GROUND_STOP, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK), Hash40::new("special_s_hold"), *SITUATION_KIND_AIR)
    } else {
        (*FIGHTER_KINETIC_TYPE_AIR_STOP, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR), Hash40::new("special_air_s_hold"), *SITUATION_KIND_GROUND)
    };
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    GroundModule::correct(fighter.module_accessor, gc_kind);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST) {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
    } else {
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST);
    }
    let charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    let charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
    if charge_frame <= charge {
        MotionModule::set_rate(fighter.module_accessor, 0.0);
    }
    WorkModule::set_int(fighter.module_accessor, mtrans, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_INT_MTRANS);
}

unsafe extern "C" fn special_s_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_button_off(Buttons::Special) {
        let charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
        let charge_frame_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame_max"));
        if charge_frame_max <= charge && fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_BREATH.into(), false.into());
            return 0.into();
        }
        if !StatusModule::is_changing(fighter.module_accessor) {
            let mtrans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_INT_MTRANS);
            if mtrans != fighter.global_table[globals::PREV_SITUATION_KIND].get_i32() && mtrans == fighter.global_table[globals::SITUATION_KIND].get_i32() {
                special_s_charge_motion_check(fighter);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE)
        && fighter.global_table[globals::CURRENT_FRAME].get_i32() >= 3
        && fighter.is_button_on(Buttons::Guard | Buttons::GuardHold) {
            VarModule::on_flag(fighter.battle_object, vars::luigi::instance::IS_MISFIRE_STORED);
            let mult = VarModule::get_float(fighter.battle_object, vars::luigi::instance::MISFIRE_DAMAGE_MULTIPLIER);
            let diminish_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "misfire.storage_diminish_mul");
            let diminish_min = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "misfire.storage_diminish_min");
            VarModule::set_float(fighter.battle_object, vars::luigi::instance::MISFIRE_DAMAGE_MULTIPLIER, (mult * diminish_mul).max(diminish_min));
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
            let smoke_eff = VarModule::get_int(fighter.battle_object, vars::luigi::instance::CHARGE_SMOKE_EFFECT_HANDLE);
            let pulse_eff = VarModule::get_int(fighter.battle_object, vars::luigi::instance::CHARGE_PULSE_EFFECT_HANDLE);
            if smoke_eff != -1 {
                EffectModule::kill(fighter.module_accessor, smoke_eff as u32, true, true);
                VarModule::set_int(fighter.battle_object, vars::luigi::instance::CHARGE_SMOKE_EFFECT_HANDLE, -1);
            }
            if pulse_eff != -1 {
                EffectModule::kill(fighter.module_accessor, pulse_eff as u32, true, true);
                VarModule::set_int(fighter.battle_object, vars::luigi::instance::CHARGE_PULSE_EFFECT_HANDLE, -1);
                EFFECT_FOLLOW(fighter, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 6, 0,  0, 1, 0, 1, true);
            }
        }
    } else {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM.into(), true.into());
    }
    0.into()
}

#[status_script(agent = "luigi", status = FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.battle_object, vars::luigi::instance::CHARGE_SMOKE_EFFECT_HANDLE, -1); 
    VarModule::set_int(fighter.battle_object, vars::luigi::instance::CHARGE_PULSE_EFFECT_HANDLE, -1);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_BONUS) {
        WorkModule::set_float(fighter.module_accessor, WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_bonus")), *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    } else {
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    }

    if !StopModule::is_stop(fighter.module_accessor) {
        special_s_charge_uniq_process(fighter);
    }
    special_s_charge_motion_check(fighter);
    fighter.global_table[globals::SUB_STATUS2].assign(&L2CValue::Ptr(special_s_charge_uniq_process as *const () as _));
    fighter.main_shift(special_s_charge_main_loop)
}

#[status_script(agent = "luigi", status = FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_s_charge_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
    0.into()
}

#[status_script(agent = "luigi", status = FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn special_s_charge_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FLASHING);
    0.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        special_s_charge_init,
        special_s_charge_main,
        special_s_charge_end,
        special_s_charge_exit
    );
    special_n::install();
}