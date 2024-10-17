use super::*;


pub unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {

    let mut keep_flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG;

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_GORDO_GET){
        if !ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_GORDO){
            fighter.set_status_kind_interrupt(*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_MISS);
            return 1.into();
        }
    }

    if StatusModule::status_kind(fighter.module_accessor) == FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_GET{
        keep_flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_DEDEDE_SPECIAL_S_FLAG;
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        keep_flag,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DEDEDE_SPECIAL_S_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DEDEDE_SPECIAL_S_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        0,
        (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0
    );

    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_WORK_FLOAT_FRAME);

    return 0.into()
}


pub unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_CONTINUE_MOT);

    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_GORDO, false, -1);
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_GORDO_THROW_NUM);

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_GENERATE);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT_OK);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_PERSONAL);

    if !ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_GORDO){
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT_FAILURE);
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_GORDO_GET){
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_get").into(), Hash40::new("special_air_s_get").into(), true.into());
    }
    else{
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_start").into(), Hash40::new("special_air_s_start").into(), false.into());
    }

    if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_GROUND{
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_CONTINUE_MOT);

    //Recatch momentum
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_GORDO_GET){   
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 1.8, y: 0.0, z:1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_get"), 0.0, 1.0, false, 0.0, false, false);
        }
        else{
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 1.2, y: 0.0, z:1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_get"), 0.0, 1.0, false, 0.0, false, false);
        }
    } 

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}


pub unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor){
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool(){
             return 0.into();
        }
    }
 
    if StatusModule::is_situation_changed(fighter.module_accessor){
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_GORDO_GET){
            fighter.sub_change_motion_by_situation(Hash40::new("special_s_get").into(), Hash40::new("special_air_s_get").into(), true.into());
        }
        else{
            fighter.sub_change_motion_by_situation(Hash40::new("special_s_start").into(), Hash40::new("special_air_s_start").into(), true.into());
        }
 
        if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_GROUND{
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
     
    }
    //Recatch and Gordodash
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_GORDO_GET){
        //Prevents Turnarounds/B-reversing while in the recatch
        if MotionModule::frame(fighter.module_accessor) < 4.0{
            ControlModule::reset_main_stick(fighter.module_accessor); 
            let old_char_lr = VarModule::get_float(fighter.battle_object, vars::dedede::instance::SPECIAL_S_TOSS_LR);
            let new_char_lr = PostureModule::lr(fighter.module_accessor);

            if old_char_lr != new_char_lr{
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::reverse_rot_y_lr(fighter.module_accessor);
            }
        }
    }

    if MotionModule::frame(fighter.module_accessor) == 12.0{
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 16.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, false, 0, 0, 0, 0, 0, false, true);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW){
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_GORDO, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT_OK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT_FAILURE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW);
    }
    if MotionModule::is_end(fighter.module_accessor){
        if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_GROUND{
            StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        }
        else if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_AIR{
            StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
        }
        return 0.into()
    }

    return 0.into()

}

pub unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::dedede::instance::SPECIAL_S_GORDO_DASH_DISABLE);

    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);

}
