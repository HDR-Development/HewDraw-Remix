use super::*;

// FIGHTER_STATUS_KIND_AIR_LASSO

unsafe extern "C" fn air_lasso_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_AirLasso()
}

pub unsafe extern "C" fn air_lasso_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("air_catch"), 0.0, 1.0, false, 0.0, false, false);
    
    // potentially a way to have the motion only affect the upper body, like aerials
    // dunno how to get this to actually work for now thpugh
    //
    // MotionModule::add_motion_partial(
    //     fighter.module_accessor, 
    //     *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 
    //     Hash40::new("air_catch_upper"), 
    //     0.0, 1.0, false, false, 0.0, false, true, true
    // );

    fighter.sub_shift_status_main(L2CValue::Ptr(air_lasso_main_loop as *const () as _))
}

unsafe extern "C" fn air_lasso_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING, false);
            }
        }
    }

    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD) {
        let rod = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD);
        let rod_id = smash::app::lua_bind::Article::get_battle_object_id(rod) as u32;
        let rod_boma = sv_battle_object::module_accessor(rod_id);
        if AttackModule::is_infliction(rod_boma, (*COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)) {
            AttackModule::clear_all(rod_boma);
            if fighter.motion_frame() < 18.0 {
                MotionModule::set_frame(fighter.module_accessor, 18.0, true);
            }
        }
    }

    fighter.status_air_lasso_main();
    0.into()
}

unsafe extern "C" fn air_lasso_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AirLasso()
}

unsafe extern "C" fn air_lasso_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_status(*FIGHTER_STATUS_KIND_AIR_LASSO_LANDING) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }

    return 0.into();
}

// FIGHTER_STATUS_KIND_AIR_LASSO_LANDING

unsafe extern "C" fn air_lasso_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_AirLassoLanding()
}

pub unsafe extern "C" fn air_lasso_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("landing");
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("air_catch_landing"), 0.0, 1.0, false, 0.0, false, false);
    ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, true, *WEAPON_PICKEL_FISHINGROD_INSTANCE_WORK_ID_FLAG_ENABLE_REWIND);
    fighter.sub_shift_status_main(L2CValue::Ptr(air_lasso_landing_main_loop as *const () as _))
}

unsafe extern "C" fn air_lasso_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_air_lasso_landing_main();
    0.into()
}

unsafe extern "C" fn air_lasso_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    fighter.status_end_AirLassoLanding()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_AIR_LASSO, air_lasso_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO, air_lasso_main);
    agent.status(End, *FIGHTER_STATUS_KIND_AIR_LASSO, air_lasso_end);
    //agent.status(Exit, *FIGHTER_STATUS_KIND_AIR_LASSO, air_lasso_exit);

    agent.status(Pre, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING, air_lasso_landing_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING, air_lasso_landing_main);
    agent.status(End, *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING, air_lasso_landing_end);
}