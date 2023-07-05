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
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        GroundModule::set_attach_ground(fighter.module_accessor, false);
        if (VarModule::get_float(fighter.battle_object, vars::buddy::instance::FEATHERS_RED_COOLDOWN)>0.0)
        {
            fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL.into(), false.into());
            PLAY_SE(fighter, Hash40::new("se_buddy_special_s04_02"));
            return true.into();
        }
        else{
            VarModule::on_flag(fighter.battle_object, vars::buddy::instance::FLUTTER_ENABLED);
        }
    }
    else if (WorkModule::get_int(fighter.module_accessor,  *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN) == 0)
    {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
    }
    else
    {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
    }
    return false.into();
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
        return original!(fighter);
    }
    
}


#[status_script(agent = "buddy", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn buddy_special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    if WorkModule::get_int(fighter.module_accessor,  *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN) == 0
    && !fighter.is_situation(*SITUATION_KIND_AIR)
    {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
    }
    else
    {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_S_FLAG_FAIL);
    }
    return 0.into()
}

#[status_script(agent = "buddy", status = FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn buddy_special_s_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    if (fighter.is_situation(*SITUATION_KIND_AIR))
    {
        return false.into();
    }
    else if (fighter.is_prev_situation(*SITUATION_KIND_AIR))
    {
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL.into(), false.into());
        return true.into();
    }
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
        buddy_special_s_exec,
        buddy_special_s_dash_pre,
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