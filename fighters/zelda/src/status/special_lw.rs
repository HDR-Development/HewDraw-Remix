use super::*;
use globals::*;

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    ControlModule::reset_flick_y(fighter.module_accessor);//reset flick for platdrop
    let phantom_object_id = VarModule::get_int(fighter.battle_object, vars::zelda::instance::PHANTOM_OBJECT_ID) as u32;
    let phantom_battle_object = utils::util::get_battle_object_from_id(phantom_object_id);
    let phantom_boma = &mut *(*phantom_battle_object).module_accessor;

    if sv_battle_object::is_active(phantom_object_id)
    && sv_battle_object::category(phantom_object_id) == *BATTLE_OBJECT_CATEGORY_WEAPON
    && sv_battle_object::kind(phantom_object_id) == *WEAPON_KIND_ZELDA_PHANTOM {
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw_attack").into(), Hash40::new("special_air_lw_attack").into(), false.into());
        if !phantom_boma.is_status(*WEAPON_ZELDA_PHANTOM_STATUS_KIND_BUILD) {
            fighter.on_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_LW_FLAG_FAIL);
        } else {
            ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_ZELDA_GENERATE_ARTICLE_PHANTOM, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), FIGHTER_LOG_DATA_INT_SHOOT_NUM);
        }
    } else {
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw").into(), Hash40::new("special_air_lw").into(), false.into());
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ZELDA_GENERATE_ARTICLE_PHANTOM, false, -1);
    }
    fighter.off_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_LW_FLAG_ATTACK_PRECEDE);
    special_lw_motion(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn phantom_button_checks(fighter: &mut L2CFighterCommon) -> L2CValue {
    let pass_thresh = fighter.get_param_float("common", "pass_stick_y");
    let phantom_object_id: u32 = VarModule::get_int(fighter.battle_object, vars::zelda::instance::PHANTOM_OBJECT_ID) as u32;
    let phantom_battle_object = utils::util::get_battle_object_from_id(phantom_object_id);
    let phantom_boma = &mut *(*phantom_battle_object).module_accessor;
    let attack_frame: f32 = fighter.get_param_int("param_special_lw", "attack_frame") as f32;//can't cancel/attack until x frames pass
    let frame = MotionModule::frame(fighter.module_accessor);
    if GroundModule::is_passable_ground(fighter.module_accessor)
    && fighter.left_stick_y() <= pass_thresh
    && ControlModule::get_flick_y(fighter.module_accessor) < 4 {
        GroundModule::pass_floor(fighter.module_accessor);
    }//platdrop
    if fighter.is_button_trigger(Buttons::Special) && fighter.is_cat_flag(Cat1::SpecialLw) { //filter non d-special inputs??
        fighter.on_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_LW_FLAG_ATTACK_PRECEDE);
    }
    //checks if phantom is alive and hers, also frame gate
    if sv_battle_object::is_active(phantom_object_id)
    && sv_battle_object::category(phantom_object_id) == *BATTLE_OBJECT_CATEGORY_WEAPON
    && sv_battle_object::kind(phantom_object_id) == *WEAPON_KIND_ZELDA_PHANTOM
    && frame >= attack_frame {
        //if phantom is building
        if phantom_boma.is_status(*WEAPON_ZELDA_PHANTOM_STATUS_KIND_BUILD) {
            //attack input
            if fighter.is_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_LW_FLAG_ATTACK_PRECEDE) {
                fighter.sub_change_motion_by_situation(Hash40::new("special_lw_attack").into(), Hash40::new("special_air_lw_attack").into(), false.into());
                ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_ZELDA_GENERATE_ARTICLE_PHANTOM, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), FIGHTER_LOG_DATA_INT_SHOOT_NUM);
            } else {
                //cancel handling
                if !VarModule::is_flag(phantom_battle_object, vars::zelda::status::PHANTOM_NO_BUILD)
                && MotionModule::frame(fighter.module_accessor) < 58.0 //before full build
                && (fighter.is_button_on(Buttons::Guard) || fighter.is_button_trigger(Buttons::Special)) {//cancel input 
                    LinkModule::send_event_nodes(fighter.module_accessor, *LINK_NO_ARTICLE, Hash40::new("fighter_zelda_remove_constraint"), 0); //disconnects phantom from her?
                    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 40.0, true, true, false);
                    VarModule::on_flag(phantom_battle_object, vars::zelda::status::PHANTOM_NO_BUILD); //should pause building
                    //shield cancel to stop building phantom, 30f of lag
                }
            }
        } else if !phantom_boma.is_status(*WEAPON_ZELDA_PHANTOM_STATUS_KIND_ATTACK) 
        && !VarModule::is_flag(phantom_battle_object, vars::zelda::status::PHANTOM_NO_BUILD) 
        && frame < 58.0 {
            //if phantom is not building or attacking
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 40.0, true, true, false);
            VarModule::on_flag(phantom_battle_object, vars::zelda::status::PHANTOM_NO_BUILD);
            //if phantom dies before full charge put her in consistent 30f of lag + frames until you can attack
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
          return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into())
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into())
        }
    }
    if fighter.is_motion_one_of(&[Hash40::new("special_lw"), Hash40::new("special_air_lw")])
    && !StatusModule::is_changing(fighter.module_accessor) {
        phantom_button_checks(fighter);
    } //handles platdrop, attack, and shield cancel
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        special_lw_motion(fighter);
    }
    //cut gr speed?
    if StatusModule::is_changing(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            let ground_speed_mul_x = fighter.get_param_float("param_special_lw", "ground_speed_mul_x");
            let speed_x = ground_speed_mul_x * sum_speed_x;
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_motion(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_motion_one_of(&[Hash40::new("special_lw"), Hash40::new("special_air_lw")]) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            if StatusModule::is_situation_changed(fighter.module_accessor) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, true, true);
            }
        } else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_ZELDA_SPECIAL_LW_AIR);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if StatusModule::is_situation_changed(fighter.module_accessor) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, true, true);
            }
        }
    } else {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));//enable ledge slip
            if StatusModule::is_situation_changed(fighter.module_accessor) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_attack"), -1.0, 1.0, 0.0, true, true);
            }
        } else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_ZELDA_SPECIAL_LW_AIR);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if StatusModule::is_situation_changed(fighter.module_accessor) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_attack"), -1.0, 1.0, 0.0, true, true);
            }
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
}