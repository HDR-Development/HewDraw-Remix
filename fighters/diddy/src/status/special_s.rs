use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::plizardon::instance::DISABLE_SPECIAL_S);
    0.into()
}

pub unsafe extern "C" fn special_s_kick_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_kick"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    
    fighter.main_shift(special_s_kick_main_loop)
}

pub unsafe extern "C" fn special_s_kick_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK_LANDING.into(), false.into());
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_PARRY) {
        fighter.change_status(FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_FLIP_FALL.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn special_s_flip_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Reduce speed on shield
    let prev_inflict_status = VarModule::get_int(fighter.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
    if prev_inflict_status & *COLLISION_KIND_MASK_SHIELD != 0 || prev_inflict_status & *COLLISION_KIND_MASK_PARRY != 0 {
        app::KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let shield_hit_end_speed_x = 0.5;//ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.shield_hit_end_speed_x");
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            shield_hit_end_speed_x * lr,
            0.0
        );
    }
    
    smashline::original_status(Main, fighter, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_FLIP_FALL)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Main, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK, special_s_kick_main);
    agent.status(Main, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_FLIP_FALL, special_s_flip_fall_main);
}