use super::*;

// FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let can_fireball = VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN) <= 0;
    if !can_fireball {
        return smashline::original_status(Main, fighter, *FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N)(fighter);
    }
    else{
        fighter.sub_change_motion_by_situation(Hash40::new("koopa_special_n_max").into(), Hash40::new("koopa_special_air_n_max").into(), false.into());
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }

        fighter.sub_shift_status_main(L2CValue::Ptr(special_n_max_main_loop as *const () as _))
    }
}

unsafe extern "C" fn special_n_max_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("koopa_special_n_max").into(), Hash40::new("koopa_special_air_n_max").into(), true.into());

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
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
        else{
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
        return 0.into();
    }

    WorkModule::set_float(fighter.module_accessor, 361.0, *FIGHTER_KOOPA_STATUS_BREATH_WORK_FLOAT_GENE_ANGLE);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_START){
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_START);
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, false, -1);
        ArticleModule::set_float(fighter.module_accessor,*FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, 361.0, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_FLOAT_ANGLE);
    }

    0.into()
}

unsafe extern "C" fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let can_fireball =  VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN) <= 0;
    if (!can_fireball){
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE_START)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE_END)
        {
            if VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN) < KOOPA_MAX_COOLDOWN {
                VarModule::inc_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN);
            }
        }
        return smashline::original_status(Exec, fighter, *FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N)(fighter);
    }
    else{
        return 0.into();
    }
}

unsafe extern "C" fn special_n_execstop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let can_fireball =  VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN) <= 0;
    if (!can_fireball){
        return smashline::original_status(ExecStop, fighter, *FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N)(fighter);
    }
    else{
        return 0.into();
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N, special_n_main);
    agent.status(Exec, *FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N, special_n_exec);
    agent.status(ExecStop, *FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N, special_n_execstop);
}
