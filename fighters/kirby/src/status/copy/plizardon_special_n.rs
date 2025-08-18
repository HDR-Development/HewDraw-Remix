use super::*;

unsafe extern "C" fn plizardon_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_n_change_motion(fighter, false);
    fighter.main_shift(plizardon_special_n_main_loop)
}

unsafe extern "C" fn plizardon_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    if fighter.is_flag(*FIGHTER_PLIZARDON_STATUS_BREATH_FLAG_GENE_BREATH) {
        fighter.off_flag(*FIGHTER_PLIZARDON_STATUS_BREATH_FLAG_GENE_BREATH);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PLIZARDON_GENERATE_ARTICLE_BREATH, false, -1);
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n_change_motion(fighter, true);
    }
    
    return 0.into();
}

unsafe extern "C" fn special_n_change_motion(fighter: &mut L2CFighterCommon, inherit: bool) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        if inherit {
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            FighterMotionModuleImpl::change_motion_kirby_copy(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        if inherit {
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            FighterMotionModuleImpl::change_motion_kirby_copy(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn plizardon_special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn plizardon_special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_steam2"), false, false);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_drown_out"), false, false);
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_PLIZARDON_SPECIAL_N, plizardon_special_n_main);
    agent.status(Exec, *FIGHTER_KIRBY_STATUS_KIND_PLIZARDON_SPECIAL_N, plizardon_special_n_exec);
    agent.status(End, *FIGHTER_KIRBY_STATUS_KIND_PLIZARDON_SPECIAL_N, plizardon_special_n_end);
}