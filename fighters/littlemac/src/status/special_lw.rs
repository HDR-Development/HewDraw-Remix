use super::*;
use globals::*;

#[status_script(agent = "littlemac", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn special_lw_old_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START);
    return 1.into()
}

#[status_script(agent = "littlemac", status = FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe extern "C" fn special_lw_hit_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into()
}

#[status_script(agent = "littlemac", status = FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    0.into()
}

#[status_script(agent = "littlemac", status = FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);
    let sum_spd_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);    //l70
    let sum_spd_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);    //l80
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let charge_motion_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_max_charge_motion_rate_"));
    MotionModule::change_motion(
        fighter.module_accessor,
        if fighter.is_situation(*SITUATION_KIND_GROUND) { Hash40::new("special_air_start") } else { Hash40::new("special_air_n_start") },
        0.0, charge_motion_rate, false, 0.0, false, false
    );
    let reset_type = if fighter.is_situation(*SITUATION_KIND_GROUND) { ENERGY_STOP_RESET_TYPE_GROUND } else { ENERGY_STOP_RESET_TYPE_AIR };
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, reset_type, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_spd_x, 0.0);
    let facing = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -0.01 * facing, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    let reac_mul_power = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), 0x22bac6d06f);
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode { _address: *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER as u8 }, -1.0, reac_mul_power, -1);
    if !StopModule::is_stop(fighter.module_accessor) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_INT_CHARGE_FRAME);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_lw_substatus as *const () as _));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01) + -1);

    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into()
}

unsafe extern "C" fn special_lw_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_INT_CHARGE_FRAME);
    }
    return 0.into()
}

pub fn install() {
    install_status_scripts!(
        special_lw_old_pre,
        special_lw_hit_init,
        special_lw_pre,
    );
}