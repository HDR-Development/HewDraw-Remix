use super::*;
use globals::*;
// status script import
 
#[status_script(agent = "buddy", status = FIGHTER_STATUS_KIND_RUN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn end_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, false);
    fighter.status_end_Run();
    0.into()
}

#[status_script(agent = "buddy", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn buddy_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue{

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
    return original!(fighter);
}

#[status_script(agent = "buddy", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn buddy_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Bypass if transitioning into Air Fail
    if (VarModule::get_float(fighter.battle_object, vars::buddy::instance::FEATHERS_RED_COOLDOWN)>0.0)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    {
        return 1.into();
    }
    else {
        fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
        let feathers_g = WorkModule::get_int(fighter.module_accessor,*FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
        if feathers_g < 0 && fighter.is_situation(*SITUATION_KIND_GROUND) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
        }
        fighter.sub_change_motion_by_situation(Hash40::new("special_s").into(), Hash40::new("special_air_s").into(), false.into());
        fighter.sub_set_ground_correct_by_situation(false.into());
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_SUPER_ARMOR_EQUIP);
        fighter.sub_shift_status_main(L2CValue::Ptr(buddy_special_s_main_loop as *const () as _))
    }
    
}

unsafe extern "C" fn buddy_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    /* 
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }*/

    fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
    buddy_special_s_armor(fighter);

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH.into(), false.into());
        return 0.into();
    }
    0.into()
}
unsafe extern "C" fn buddy_special_s_armor(fighter: &mut L2CFighterCommon) {
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

#[status_script(agent = "buddy", status = FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn buddy_special_s_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
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
    return original!(fighter);
}


#[status_script(agent = "buddy", status = FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn buddy_special_s_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        return original!(fighter);
    }
    //Prevents losing a gold feather
    WorkModule::add_int(fighter.module_accessor, 1, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);

    return original!(fighter);
}

#[status_script(agent = "buddy", status = FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn buddy_special_s_fail_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
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
    return original!(fighter);
}

/// pre status for bayonet
/// handles initialization
pub unsafe extern "C" fn bayonet_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_BIND,
        *GROUND_CORRECT_KIND_GROUND as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );

    0.into()
}

/// main status loop for bayonet_end
unsafe extern "C" fn bayonet_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // exit if the animation is not done yet
    if !MotionModule::is_end(fighter.module_accessor) {
        return 0.into();
    }

    // if the animation is over, transition to shoot
    fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
    1.into()
}

pub unsafe extern "C" fn bayonet_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // change summon anim depending on LR
    let frame = 26.0;
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_shoot_start"), frame, 1.0, false, 0.0, false, false);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, Hash40::new("special_n_start"), false, frame);

    fighter.main_shift(bayonet_end_main_loop)
}
pub unsafe extern "C" fn bayonet_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // re-enable energies and remove the screenwide effect
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    let next_status = StatusModule::status_kind_next(fighter.module_accessor);
    let is_still_blasting = [*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_F,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_B,
    *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_TURN,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP_SQUAT,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_LANDING,
    *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP_AERIAL,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_AIR,
    *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_FALL,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_AIR_TURN,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_END].contains(&next_status);

    if !is_still_blasting {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, ArticleOperationTarget(0));
        ItemModule::set_change_status_event(fighter.module_accessor, true);
        WorkModule::set_int(fighter.module_accessor, 0,*FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
    }

    0.into()
}

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)){ 
        VarModule::on_flag(fighter.battle_object, vars::buddy::instance::FLUTTER_ENABLED);
    }
    return true.into();
}

#[smashline::fighter_init]
fn buddy_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_BUDDY {
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}


pub fn install() {
    smashline::install_agent_init_callbacks!(buddy_init);
    install_status_scripts!(
        end_run,
        buddy_special_s_pre,
        //buddy_special_s_dash_pre,
        buddy_special_s_dash_main,
        buddy_special_s_fail_pre,
        buddy_special_s_main,
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_buddy"),
        statuses::buddy::BUDDY_BAYONET_END,
        StatusInfo::new()
            .with_pre(bayonet_end_pre)
            .with_main(bayonet_end_main)
            .with_end(bayonet_end_end)
    );
}