use super::*;


#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn koopa_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let can_fireball = VarModule::get_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME) <= 0;
    if (!can_fireball){
        return original!(fighter);
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

        fighter.sub_shift_status_main(L2CValue::Ptr(koopa_specialnmax_main_loop as *const () as _))
    }
}

unsafe extern "C" fn koopa_specialnmax_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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


#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn koopa_special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let can_fireball =  VarModule::get_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME) <= 0;
    if (!can_fireball){
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE_START)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE_END)
        {
            if VarModule::get_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME) < KOOPA_MAX_COOLDOWN {
                VarModule::inc_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME);
            }
        }
        return original!(fighter);
    }
    else{
        return 0.into();
    }
}

#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STOP)]
unsafe fn koopa_special_n_execstop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let can_fireball =  VarModule::get_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME) <= 0;
    if (!can_fireball){
        return original!(fighter);
    }
    else{
        return 0.into();
    }
}



pub fn install() {
    smashline::install_status_scripts!(
        koopa_special_n_main,
        koopa_special_n_exec,
        koopa_special_n_execstop,
    );
}