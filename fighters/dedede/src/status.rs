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
    }
    return original!(weapon);
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
        dedede_special_hi_status_main,
    );
}