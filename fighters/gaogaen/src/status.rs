use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        attack_lw4_main,
        attack_lw4_map_correction,
        gaogaen_special_n_pre,
        exec_special_n,
    );
}

// FIGHTER_STATUS_KIND_ATTACK_LW4 //

#[status_script(agent = "gaogaen", status = FIGHTER_STATUS_KIND_ATTACK_LW4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    fighter.attack_lw4_mtrans();
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.status_ThrowKirby_Uniq(L2CValue::Bool(false));
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_status_ThrowKirby_Uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_attack_lw4_main_loop as *const () as _))
}

pub unsafe extern "C" fn gaogaen_attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    && !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND)
    && !MotionModule::is_end(fighter.module_accessor) {
        fighter.sub_status_uniq_process_ThrowKirby_execFixPos();
        return 0.into()
    }
    fighter.status_AttackLw4_Main()
}

#[status_script(agent = "gaogaen", status = FIGHTER_STATUS_KIND_ATTACK_LW4, condition = LUA_SCRIPT_STATUS_FUNC_MAP_CORRECTION)]
pub unsafe fn attack_lw4_map_correction(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let prev_frame = MotionModule::prev_frame(fighter.module_accessor);
    let start_air_frame = 2.0;
    let fall_start_frame = 16.0;
    let fall_stop_frame = 17.0;
    let landing_frame = 19.0;

    if prev_frame < start_air_frame && frame >= start_air_frame {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    if frame <= fall_start_frame {
        return 0.into()
    }
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        if prev_frame < fall_stop_frame && frame >= fall_stop_frame {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -12.0);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            app::sv_kinetic_energy::set_accel_x_mul(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            app::sv_kinetic_energy::set_accel_x_add(fighter.lua_state_agent);
            MotionModule::set_frame(fighter.module_accessor, fall_stop_frame, true);
            MotionModule::set_rate(fighter.module_accessor, 0.0);
        }
    }
    else {
        if frame < landing_frame {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
            MotionModule::set_frame(fighter.module_accessor, landing_frame, true);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    }
    0.into()
}

#[status_script(agent = "gaogaen", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn gaogaen_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    let mask_flag = if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64
    } else {
        0 as u64
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        mask_flag,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    0.into()
    
}

#[status_script(agent = "gaogaen", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn exec_special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }

    return 0.into()
}

    // SVar5 = lib::L2CValue::as_integer(SITUATION_KIND_NONE);
    // iVar6 = lib::L2CValue::as_integer(FIGHTER_KINETIC_TYPE_UNIQUE);
    // uVar7 = lib::L2CValue::as_integer(GROUND_CORRECT_KIND_KEEP);
    // GVar8 = lib::L2CValue::as_integer(GROUND_CLIFF_CHECK_KIND_NONE);
    // bVar1 = lib::L2CValue::as_bool(true);
    // iVar9 = lib::L2CValue::as_integer(FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG);
    // iVar10 = lib::L2CValue::as_integer(FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT);
    // iVar11 = lib::L2CValue::as_integer(0);

    // init_settings_impl(boma, SITUATION_KIND_NONE, FIHGTER_KINETIC_TYPE_UNIQUE,
    // GROUND_CORRECT_KIND_KEEP, GROUND_CLIFF_CHECK_KIND_NONE, true,
    // FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
    // 0, 0)

    // L2CValue(aLStack200, FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N |
    // FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) //array?

    // bVar1 = lib::L2CValue::as_bool(false);
    // iVar6 = lib::L2CValue::as_integer(FIHGTER_TREADED_KIND_NO_REAC);
    // bVar2 = lib::L2CValue::as_bool(false);
    // bVar3 = lib::L2CValue::as_bool(false);
    // bVar4 = lib::L2CValue::as_bool(false);
    // uVar13 = lib::L2CValue::as_integer(array);
    // uVar7 = lib::L2CValue::as_integer(FIHGTER_STATUS_ATTR_START_TURN);
    // uVar12 = lib::L2CValue::as_integer(FIHGTER_POWER_UP_ATTACK_BIT_SPECIAL_N);

    // set_fighter_status_data_impl(boma, false, FIGHTER_TREADED_KIND_NO_REAC, false, false,
    // false, array, FIGHTER_STATUS_ATTR_START_TURN, FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N)



    // if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
    //     KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    //     GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_KEEP));
    //     GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
    //     WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_GRAVITY_DEFAULT);
    // }
    // if GroundModule::can_entry_cliff(fighter.module_accessor) == 1 && KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_FALL && fighter.global_table[STICK_Y].get_f32() > -0.66 {
    //     fighter.change_status(
    //         L2CValue::I32(*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE),
    //         L2CValue::Bool(false)
    //     );
    //     return 1.into()
    // }
    // return 0.into()
