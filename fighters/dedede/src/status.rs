use super::*;
use globals::*;

// Prevent going into air jet hammer when Special is released during Jumpsquat
#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_lw_jump_squat_exec(fighter: &mut L2CFighterCommon) -> L2CValue{
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK, true);
    }
    return 0.into();
}


// Reset Gordo recatch flags on despawn
#[status_script(agent = "dedede_gordo", status = WEAPON_DEDEDE_GORDO_STATUS_KIND_DEAD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dedede_gordo_dead_end(weapon: &mut L2CWeaponCommon) -> L2CValue{
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_DEDEDE{
        let dedede = utils::util::get_battle_object_from_id(owner_id);
        VarModule::set_flag(dedede, vars::dedede::instance::CAN_WADDLE_DASH_FLAG, true); 
        VarModule::set_int(dedede, vars::dedede::instance::RECATCH_COUNTER, 0); 
        VarModule::set_flag(dedede, vars::dedede::instance::IS_DASH_GORDO, false);
        VarModule::set_flag(dedede, vars::dedede::instance::IS_STAGE_STICK_FLAG, false);
    }
    return original!(weapon);
}

#[status_script(agent = "dedede_gordo", status = WEAPON_DEDEDE_GORDO_STATUS_KIND_WALL_STOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dedede_gordo_wall_stop_pre(weapon: &mut L2CWeaponCommon) -> L2CValue{
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let ret = original!(weapon);
    let dedede = utils::util::get_battle_object_from_id(owner_id);
    if StatusModule::prev_status_kind(owner_module_accessor, 0) == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT 
    || StatusModule::status_kind(owner_module_accessor) == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT{
        StatusModule::init_settings(
            weapon.module_accessor,
            app::SituationKind(*SITUATION_KIND_GROUND),
            *WEAPON_KINETIC_TYPE_DEDEDE_GORDO_WALL_STOP,
            *GROUND_CORRECT_KIND_GROUND as u32,
            app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
            true,
            *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
            *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
            *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
            *FS_SUCCEEDS_KEEP_ATTACK,
        );

        return 0.into();

    }
    else{
        return 0.into();

    }

}

//Removes wall stick intangibility
#[status_script(agent = "dedede_gordo", status = WEAPON_DEDEDE_GORDO_STATUS_KIND_WALL_STOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dedede_gordo_wall_stop_main(weapon: &mut L2CWeaponCommon) -> L2CValue{
    let ret = original!(weapon);
    HitModule::set_whole(weapon.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_DEDEDE{
        let dedede = utils::util::get_battle_object_from_id(owner_id);
        VarModule::set_flag(dedede, vars::dedede::instance::IS_STAGE_STICK_FLAG, true);
        
    }
    ret
}

#[status_script(agent = "dedede", status = *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dedede_special_hi_status_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);

    return 0.into();
}

pub fn install(){
    smashline::install_status_scripts!(
        special_lw_jump_squat_exec,
        dedede_gordo_dead_end,
        dedede_gordo_wall_stop_main,
        dedede_gordo_wall_stop_pre,
        dedede_special_hi_status_main,
    );
}