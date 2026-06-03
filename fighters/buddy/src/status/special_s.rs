use super::*;
 
// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    if (fighter.is_situation(*SITUATION_KIND_GROUND) ) {
        let feathers_g = WorkModule::get_int(fighter.module_accessor,*FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
        if feathers_g <= 0 {
            fighter.set_status_kind_interrupt(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL);
            PLAY_SE(fighter, Hash40::new("se_buddy_special_s04_02"));
            return 1.into();
        }
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    }
    if (VarModule::get_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN)>0.0)
    {
        fighter.set_status_kind_interrupt(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL);
        PLAY_SE(fighter, Hash40::new("se_buddy_special_s04_02"));
        return 1.into();
    }

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK 
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    VarModule::on_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_S_FAIL_ENABLE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
    return 0.into();
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_BOUNCE_TYPE, vars::buddy::instance::BOUNCE_TYPE_NORMAL);
    let original = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        return original;
    }
    let has_red_feather = VarModule::get_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN)<=0.0;
    WorkModule::set_flag(fighter.module_accessor, !has_red_feather,*FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
    WorkModule::set_flag(fighter.module_accessor, has_red_feather,vars::buddy::instance::SPECIAL_S_BEAKBOMB_ACTIVE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);

    if has_red_feather {
        VarModule::set_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN, super::super::opff::FEATHERS_RED_COOLDOWN_MAX);
    }

    original
}

unsafe extern "C" fn special_s_armor(fighter: &mut L2CFighterCommon) {
    let needsarmor_on = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
    let needsarmor_off = !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR) && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
    if needsarmor_on {
        HitModule::set_total_status_disguise(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
    }
    else if needsarmor_off {
        HitModule::set_total_status_disguise(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
    }
}

// FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH

pub unsafe extern "C" fn special_s_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    if (fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.is_prev_situation(*SITUATION_KIND_AIR))
    {
        fighter.set_status_kind_interrupt(*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL);
        return 1.into();
    }
    else if (fighter.is_situation(*SITUATION_KIND_GROUND)) {
        return smashline::original_status(Pre, fighter, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH)(fighter);
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_s_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Prevents losing a gold feather
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        WorkModule::add_int(fighter.module_accessor, 1, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
    }
    VarModule::set_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_BOUNCE_TYPE,0);
    return smashline::original_status(Main, fighter, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH)(fighter);
}
unsafe extern "C" fn special_s_dash_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_HIT_FIGHTER)
        && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            // VarModule::set_float(fighter.battle_object, vars::buddy::instance::SPECIAL_S_RED_FEATHER_COOLDOWN, 0.0);
            // super::super::opff::buddy_meter_update_HUD(fighter, true);
            // VarModule::set_int(fighter.battle_object, vars::buddy::instance::HUD_DISPLAY_TIME, 45);
            // app::FighterUtil::flash_eye_info(fighter.module_accessor);

            VarModule::set_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_BOUNCE_TYPE, vars::buddy::instance::BOUNCE_TYPE_ATTACK);
            fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL.into(), false.into());
        }
    }

    // Skip to end on shield
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_PARRY) {
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 1.into();
    }

    return smashline::original_status(Exec, fighter, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH)(fighter);
}

// FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL

pub unsafe extern "C" fn special_s_fail_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (fighter.is_situation(*SITUATION_KIND_AIR))
    {
        if VarModule::is_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_S_FAIL_ENABLE)
        {
            sv_kinetic_energy!(
                clear_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY
            );
            VarModule::off_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_S_FAIL_ENABLE);
        }
    }
    else if (VarModule::get_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_FRAME) > 0){
        let ground_brake = sv_fighter_util::get_default_fighter_param_ground_brake(fighter.lua_state_agent);
        KineticModule::clear_speed_all(fighter.module_accessor);
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ground_brake,
            0.0
        );
    }
    0.into()
}

// FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END

unsafe extern "C" fn special_s_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK)  as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    
    0.into()
}

unsafe extern "C" fn special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Reduce speed on shield
    let prev_inflict_status = VarModule::get_int(fighter.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
    if prev_inflict_status & *COLLISION_KIND_MASK_SHIELD != 0 || prev_inflict_status & *COLLISION_KIND_MASK_PARRY != 0 {
        let shield_hit_end_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.shield_hit_end_speed_x");
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            shield_hit_end_speed_x * lr,
            0.0
        );
    }
    
    smashline::original_status(Main, fighter, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END)(fighter)
}

// FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL

unsafe extern "C" fn special_s_wall_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK)  as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    
    0.into()
}

unsafe extern "C" fn special_s_wall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Main, fighter, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL)(fighter);

    let bounce_type = VarModule::get_int(fighter.battle_object, vars::buddy::instance::SPECIAL_S_BEAKBOMB_BOUNCE_TYPE);
    if bounce_type == vars::buddy::instance::BOUNCE_TYPE_NORMAL || fighter.is_situation(*SITUATION_KIND_GROUND){
        return original;
    }

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_jump"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY);
    
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        2.25
    );

    let speed_x = PostureModule::lr(fighter.module_accessor) *-1.0;
    let stable_x = PostureModule::lr(fighter.module_accessor) *-0.25;
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let air_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_brake_x"), 0);
    let accel_x_mul = 0.5;
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        mul_x_accel_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        accel_x_mul
    );
    sv_kinetic_energy!(
        mul_x_accel_add,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        accel_x_mul
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable,
        0.0
    );

    KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_AIR_BRAKE,
        speed_x,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_brake,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        air_brake_x*2.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        stable_x
    );


    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_wall_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_wall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue { 
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    let air_start = true;//MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_s_wall");
    if !air_start {
        if fighter.is_situation(*SITUATION_KIND_AIR){
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_LANDING_HEAVY) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
        if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.is_prev_situation(*SITUATION_KIND_AIR) {
            //if FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING enabled
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
        //WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    }
	if !StatusModule::is_changing(fighter.module_accessor)
	&& StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(hash40("special_s_wall").into(), hash40("special_air_s_wall").into(), false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);

    //agent.status(Pre, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, special_s_dash_pre);
    agent.status(Main, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, special_s_dash_main);
    agent.status(Exec, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, special_s_dash_exec);

    agent.status(Init, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, special_s_fail_init);

    agent.status(Pre, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, special_s_end_pre);
    agent.status(Main, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, special_s_end_main);

    agent.status(Pre, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, special_s_wall_pre);
    agent.status(Main, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, special_s_wall_main);
}