use super::*;

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::edge::status::SPECIAL_LW_CHECK_HOLD);
    fighter.change_motion_by_situation("special_lw", "special_air_lw", 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_lw").into());
    special_lw_set_kinetic(fighter, true.into());

    // The Load-Bearing Scintilla Placement
    let shield_max = fighter.get_param_float("param_special_lw", "shield_max");
    fighter.set_float(shield_max, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLOAT_SHIELD);

    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        VarModule::off_flag(fighter.battle_object, vars::edge::status::SPECIAL_LW_CHECK_HOLD);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
    }
    fighter.sub_change_motion_by_situation(Hash40::new("special_lw").into(), Hash40::new("special_air_lw").into(), true.into());

    return 0.into();
}

unsafe fn special_lw_set_kinetic(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if !param_1.get_bool() && fighter.is_prev_situation(*SITUATION_KIND_AIR) {
            return;
        }
        sv_kinetic_energy!(set_needs_set_param, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, false);
        if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            sv_kinetic_energy!(set_needs_set_param, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, false);
        }
    }
}

unsafe extern "C" fn special_lw_hit_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    // we are in exec, so this will pass on status frame 6
    if VarModule::is_flag(fighter.battle_object, vars::edge::status::SPECIAL_LW_CHECK_HOLD)
    && fighter.check_hold_input(0, 5, Buttons::SpecialAll) {
        VarModule::off_flag(fighter.battle_object, vars::edge::status::SPECIAL_LW_CHECK_HOLD);
        VarModule::on_flag(fighter.battle_object, vars::edge::status::SPECIAL_LW_HOLD);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    agent.status(Exec, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT, special_lw_hit_exec);
}
