use super::*;

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dk_specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_AIR_STOP,
        *GROUND_CORRECT_KIND_AIR_PASSABLE_OFF as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    return false.into();
}

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn dk_specialhi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = StatusModule::situation_kind(fighter.module_accessor);

    //Freeze in place//
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    //Spawn barrel//
    let mut barrelBoma = fighter.module_accessor;
    VisibilityModule::set_model_visible(fighter.module_accessor, false);

    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL,true,0);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
        barrelBoma = get_article_boma(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);

        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        PostureModule::set_lr(barrelBoma, 1.0);
        PostureModule::update_rot_y_lr(barrelBoma);
        PostureModule::set_scale(barrelBoma,PostureModule::scale(fighter.module_accessor)*1.2,false);
        PostureModule::add_pos(barrelBoma, &Vector3f{x: 0.0, y:8.6*PostureModule::scale(fighter.module_accessor), z:0.0});
    }

    return false.into();
}

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dk_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, 
        Hash40::new("special_air_hi"),true, 0.0);
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, 1.0);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(specialhi_main_loop as *const () as _))
}

unsafe extern "C" fn specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE){
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return 1.into();
        }

        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
        
        if MotionModule::is_end(fighter.module_accessor) 
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_SPINEND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
    }
    
    return 0.into();
}

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn dk_specialhi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_visibility(fighter.module_accessor, false);
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        dk_specialhi_landing_exec(fighter);
    }
    if (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_SPINEND))
    {
        dk_specialhi_end_exec(fighter);
    }
    else{
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE){
            dk_specialhi_launch_exec(fighter);
        }
        else
        {
            dk_specialhi_start_exec(fighter);
        }
    }
    return false.into();
}

unsafe fn dk_specialhi_start_exec(fighter: &mut L2CFighterCommon) {

    //Freeze in place//
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    let frame = MotionModule::frame(fighter.module_accessor);
    //Go into special fall if barrel could not be spawned for whatever reason
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL){
        if frame > 2.0 {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
        return;
    }
    let barrelBoma = get_article_boma(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);
    let barrelMotion = MotionModule::motion_kind(barrelBoma);
    let barrelFrame = MotionModule::frame(barrelBoma);
    let canLaunch = barrelMotion == Hash40::new("special_air_hi_aim").hash;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_START)
    {
        //Either being launch, or rotate barrel
        if canLaunch
        {
            if MotionModule::is_end(barrelBoma)
            || barrelFrame <= 0.0
            || barrelFrame >= 90.0
            {
                let rate = MotionModule::rate(barrelBoma);
                ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, -rate);
            }

            if (ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
            || (ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW))
            {
                MotionModule::set_frame(fighter.module_accessor, 10.0, false);
                MotionModule::set_rate(fighter.module_accessor, 1.0);

                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, 
                    Hash40::new("special_air_hi_launch"),
                true, 0.0);
                ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, 1.0);
                
                let angle = barrelFrame-45.0;
                //println!("Request launch at angle {}",angle);

                WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_WORK_FLOAT_TERM);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_START);
                PostureModule::set_rot(barrelBoma, &Vector3f{x: angle, y:0.0, z:0.0}, 0);
            }
        }
        //Begin rotation
        else if MotionModule::is_end(barrelBoma)
        || barrelFrame >= 12.0
        {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, 
                Hash40::new("special_air_hi_aim"),
            true, 45.0);
            let lr = if (*PostureModule::pos(fighter.module_accessor)).x >= 0.0 {-1.0} else {1.0};
            ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, 3.0*lr);
            MotionModule::set_rate(fighter.module_accessor, 0.0);
        }

    }
    else {
        let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_WORK_FLOAT_TERM);

        if frame >= 28.0{
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_specialairhi"), -1);
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_specialairhi"), -1);
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new("expression_specialairhi"), -1);

            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
            VisibilityModule::set_model_visible(fighter.module_accessor, true);
            
            if (angle.abs()>1.0) {
                PostureModule::set_lr(fighter.module_accessor, angle.signum());
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }

            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SUPER_JUMP_PUNCH_AIR);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

            let speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("x_spd_max_air"));
            let speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("y_spd_air"));

            let speed_x= (angle.to_radians()).sin()*speed_x_mul;
            let speed_y= (angle.to_radians()).cos()*speed_y_mul;
            let lr = PostureModule::lr(fighter.module_accessor);
            SET_SPEED_EX(fighter,speed_x*lr,speed_y,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);

        }

    }
}

unsafe fn dk_specialhi_launch_exec(fighter: &mut L2CFighterCommon){

    let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_WORK_FLOAT_TERM);
    let speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("x_spd_max_air"));
    let speed_x= (angle.to_radians()).sin()*speed_x_mul;
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let min_y = -2.0*WorkModule::get_param_float(fighter.module_accessor, hash40("dive_speed_y"),0);
    let lr = PostureModule::lr(fighter.module_accessor);
    SET_SPEED_EX(fighter,speed_x*lr,(speed_y-0.07).max(min_y),*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let air_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_mul"), 0);
    let air_speed_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_add"), 0);
    
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let mut add_speed = Vector3f{x: stick_x*(air_speed_x_stable*air_speed_x_mul), y: 0.0, z: 0.0};
    KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &add_speed);

    if speed_y < 1.0 {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32){
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND),false);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 61.0, 1.0, false, 0.0, false, false);
    }
    /* 
    else if GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) 
    || GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP_SIDE as u32) {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
    }*/
}

unsafe fn dk_specialhi_landing_exec(fighter: &mut L2CFighterCommon){
    fighter.sub_set_ground_correct_by_situation(false.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    if (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND){
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 61.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));

        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_DASH,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );

        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_SPINEND);
    }
}

unsafe fn dk_specialhi_end_exec(fighter: &mut L2CFighterCommon){
}

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn dk_specialhi_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_model_visible(fighter.module_accessor,  true);

    let launched = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
    if !launched {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_specialairhi"), -1);
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_specialairhi"), -1);
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        };
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        dk_specialhi_pre,
        dk_specialhi_init,
        dk_specialhi_main,
        dk_specialhi_exec,
        dk_specialhi_exit,
    );
}