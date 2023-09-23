use super::*;

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPECIAL_LW_CRAWL) {
        return original!(fighter);
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_ST_INIT);

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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    
    let new_ice = !VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE);
    VarModule::set_flag(fighter.battle_object, vars::samus::instance::ICE_MODE, new_ice);
    suit_effect(fighter.module_accessor,fighter.battle_object);
    
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
    EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
    let eff_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX);
    effect!(fighter, MA_MSC_EFFECT_REMOVE, eff_max, 1);

    return 0.into();
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPECIAL_LW_CRAWL) {
        return original!(fighter);
    }
    let motion_g = if PostureModule::lr(fighter.module_accessor) < 0.0 {Hash40::new("special_lw_l")} else {Hash40::new("special_lw_r")};
    let motion_a = if PostureModule::lr(fighter.module_accessor) < 0.0 {Hash40::new("special_air_lw_l")} else {Hash40::new("special_air_lw_r")};
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {motion_g} else {motion_a};
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let motion_g = if PostureModule::lr(fighter.module_accessor) < 0.0 {Hash40::new("special_lw_l")} else {Hash40::new("special_lw_r")};
        let motion_a = if PostureModule::lr(fighter.module_accessor) < 0.0 {Hash40::new("special_air_lw_l")} else {Hash40::new("special_air_lw_r")};
        fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), true.into());

        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) {FIGHTER_STATUS_KIND_WAIT}  else {FIGHTER_STATUS_KIND_FALL};
        fighter.change_status(status.into(),false.into());
        return 0.into();
    }
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        1.0,
        0.0
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        0.0
    );
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_ON_CHANGE_LR)]
unsafe fn special_lw_lr(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPECIAL_LW_CRAWL) {
        return 0.into();
    }
    let motion_g = if PostureModule::lr(fighter.module_accessor) < 0.0 {Hash40::new("special_lw_l")} else {Hash40::new("special_lw_r")};
    let motion_a = if PostureModule::lr(fighter.module_accessor) < 0.0 {Hash40::new("special_air_lw_l")} else {Hash40::new("special_air_lw_r")};
    fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), true.into());

    0.into()
}

pub fn install() {
    install_status_scripts!(
        special_lw_pre,
        special_lw_main,
        special_lw_lr,
    );
}