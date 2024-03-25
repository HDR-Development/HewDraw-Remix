use super::*;
 
// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    if (fighter.is_situation(*SITUATION_KIND_GROUND) ) {
        let feathers_g = WorkModule::get_int(fighter.module_accessor,*FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
        if feathers_g <= 0 {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL);
            PLAY_SE(fighter, Hash40::new("se_buddy_special_s04_02"));
            return 1.into();
        }
    }

    if (fighter.is_situation(*SITUATION_KIND_AIR) )
    {
        
        StatusModule::init_settings(
            fighter.module_accessor,
            app::SituationKind(*SITUATION_KIND_AIR),
            *FIGHTER_KINETIC_TYPE_UNIQ,
            *GROUND_CORRECT_KIND_KEEP as u32,
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
            (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK 
                | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            *FIGHTER_STATUS_ATTR_START_TURN as u32,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
            0
        );

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
        //GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        //GroundModule::set_attach_ground(fighter.module_accessor, false);
        if (VarModule::get_float(fighter.battle_object, vars::buddy::instance::FEATHERS_RED_COOLDOWN)>0.0)
        {
            //fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL.into(), false.into());
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL);
            PLAY_SE(fighter, Hash40::new("se_buddy_special_s04_02"));
            return 1.into();
        }
        else{
            VarModule::on_flag(fighter.battle_object, vars::buddy::instance::FLUTTER_ENABLED);
        }
        return 0.into();
    }
    return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
}

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let feathers_g = WorkModule::get_int(fighter.module_accessor,*FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
    //Bypass if transitioning into Air Fail
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if VarModule::get_float(fighter.battle_object, vars::buddy::instance::FEATHERS_RED_COOLDOWN) > 0.0 {
            return 1.into();
        }
    }
    else {
        fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
        if feathers_g <= 0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
            fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL.into(), false.into());
        }
    }
    fighter.sub_change_motion_by_situation(Hash40::new("special_s").into(), Hash40::new("special_air_s").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(false.into());
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
    
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    /* 
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }*/

    fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
    special_s_armor(fighter);

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH.into(), false.into());
        return 0.into();
    }
    0.into()
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
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL.into(), false.into());
        return true.into();
    }
    else if (fighter.is_situation(*SITUATION_KIND_AIR) )
    {
        StatusModule::init_settings(
            fighter.module_accessor,
            app::SituationKind(*SITUATION_KIND_AIR),
            *FIGHTER_KINETIC_TYPE_UNIQ,
            *GROUND_CORRECT_KIND_AIR as u32,
            app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        return 0.into();
    }
    return smashline::original_status(Pre, fighter, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH)(fighter);
}

unsafe extern "C" fn special_s_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        return smashline::original_status(Main, fighter, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH)(fighter);
    }
    //Prevents losing a gold feather
    WorkModule::add_int(fighter.module_accessor, 1, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);

    return smashline::original_status(Main, fighter, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH)(fighter);
}

// FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL

pub unsafe extern "C" fn special_s_fail_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    if (fighter.is_situation(*SITUATION_KIND_AIR))
    {
        if VarModule::is_flag(fighter.battle_object, vars::buddy::instance::FLUTTER_ENABLED)
        {
            sv_kinetic_energy!(
                clear_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY
            );
            VarModule::off_flag(fighter.battle_object, vars::buddy::instance::FLUTTER_ENABLED);
        }
    }
    else if (VarModule::get_int(fighter.battle_object, vars::buddy::instance::BEAKBOMB_FRAME) > 0){
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
    return smashline::original_status(Pre, fighter, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL)(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);

    agent.status(Pre, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, special_s_dash_pre);
    agent.status(Main, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, special_s_dash_main);

    agent.status(Pre, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, special_s_fail_pre);
}