use super::*;
use globals::*;
use smashline::*;

pub fn install() {
    smashline::Agent::new("ryu")
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, init_special_lw)
        .status(Pre, statuses::ryu::INSTALL, special_lw_install_pre)
        .status(Main, statuses::ryu::INSTALL, special_lw_install_main)
        .status(End, statuses::ryu::INSTALL, special_lw_install_end)
        .install();
}

// FIGHTER_STATUS_KIND_SPECIAL_LW //

pub unsafe extern "C" fn init_special_lw(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_flag(fighter.battle_object, vars::shotos::status::IS_ENABLE_MAGIC_SERIES_CANCEL, VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_ENABLE_SPECIAL_LW_INSTALL));
    VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_ENABLE_SPECIAL_LW_INSTALL);
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}


unsafe extern "C" fn special_lw_install_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    return 0.into();
}

unsafe extern "C" fn special_lw_install_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_lw_install_set_kinetic(fighter);
    // install should only have been allowed if we had max meter upon entering DSpecial, so we set meter to max just in case there was a metered DSpecial cancel
    MeterModule::add(fighter.battle_object, (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object())) - MeterModule::meter(fighter.battle_object));
    VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_thunder"), Hash40::new("handl"), &Vector3f::new(0.0, 0.0, 0.0), &Vector3f::new(0.0, 0.0, 0.0), 0.3, false, 0, 0, 0, 0, 0, false, false);
    EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_thunder"), Hash40::new("handr"), &Vector3f::new(0.0, 0.0, 0.0), &Vector3f::new(0.0, 0.0, 0.0), 0.3, false, 0, 0, 0, 0, 0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_install_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_install_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_lw_install_set_kinetic(fighter);
            return 0.into();
        }
    }
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
    }
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        // TODO: replace these with actual params
        let fighter_gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut FighterKineticEnergyGravity;
        smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(fighter_gravity, -0.03);
        smash::app::lua_bind::FighterKineticEnergyGravity::set_stable_speed(fighter_gravity, -1.6);
    }

    0.into()
}

unsafe extern "C" fn special_lw_install_end(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_lw_install_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new("special_air_lw_install"),
            -1.0,
            1.0,
            0.0,
            true,
            true
        );
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new("special_lw_install"),
            -1.0,
            1.0,
            0.0,
            true,
            true
        );
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
}