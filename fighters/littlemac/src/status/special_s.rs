use super::*;
use globals::*;


// FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP

#[status_script(agent = "littlemac", status = FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn special_s_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
    original!(fighter)
}

// FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP_END

#[status_script(agent = "littlemac", status = FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn special_s_jump_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_jump_end"), 0.0, 1.0, false, 0.0, false, false);
    fighter.main_shift(special_s_jump_end_main_loop)
}

unsafe extern "C" fn special_s_jump_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
        }
        else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
        return 1.into();
    }
    // <HDR>
    if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    // </HDR>
    0.into()
}

// FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END

#[status_script(agent = "littlemac", status = FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn special_s_blow_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    0.into()
}

#[status_script(agent = "littlemac", status = FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn special_s_blow_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);

    let prev_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_GROUND,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        prev_speed_x,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    let mut prev_motion_frame = MotionModule::frame(fighter.module_accessor);
    if prev_motion_frame - 10.0 < 0.0 {
        prev_motion_frame = 0.0;
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_blow_end"), prev_motion_frame, 1.0, false, 0.0, false, false);
    fighter.main_shift(special_s_blow_end_main_loop)
}

unsafe extern "C" fn special_s_blow_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_blow_landing_frame_"));
    if MotionModule::is_end(fighter.module_accessor)
    && fighter.global_table[CURRENT_FRAME].get_i32() >= landing_frame {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
        }
        else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
        return 1.into();
    }
    // <HDR>
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    // </HDR>
    0.into()
}

pub fn install() {
    install_status_scripts!(
        special_s_jump_main,
        special_s_jump_end_main,
        special_s_blow_end_pre,
        special_s_blow_end_main
    );
}