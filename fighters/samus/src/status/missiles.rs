use super::*;
use globals::*;

#[status_script(agent = "samus_missile", status = WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn missile_homing_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //set ice
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = utils::util::get_battle_object_from_id(owner_id);
    let owner_boma = (*owner).boma();
    let owner_kind = utility::get_kind(owner_boma);
    let is_ice = owner_kind == *FIGHTER_KIND_SAMUS && VarModule::is_flag(owner, vars::samus::instance::ICE_MODE);

    let toReturn = original!(weapon);
    if !is_ice  {
        return toReturn;
    }

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("ice"), 0.0, 1.0, false, 0.0, false, false);
    return toReturn;
}

#[status_script(agent = "samus_missile", status = WEAPON_SAMUS_MISSILE_STATUS_KIND_H_BURST, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn missile_burst_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let is_ice = MotionModule::motion_kind(weapon.module_accessor) == Hash40::new("ice").hash;

    let toreturn = original!(weapon);
    if is_ice  {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("i_burst"), 0.0, 1.0, false, 0.0, false, false);
    }
    return toreturn;
}


#[status_script(agent = "samus_supermissile", status = WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_READY, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn supermissile_ready_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);  
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if owner_kind == *FIGHTER_KIND_SAMUS {
        let samus_status = StatusModule::status_kind(owner_boma);
        if samus_status == *FIGHTER_STATUS_KIND_THROW {
            PostureModule::set_rot(weapon.module_accessor,
            &Vector3f{x:-90.0,y:0.0,z:0.0},
            0
            );
        }
    }

    return 0.into();
}

#[status_script(agent = "samus_supermissile", status = WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_READY, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn supermissile_ready_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if PostureModule::rot_x(weapon.module_accessor, 0) == -90.0 {
        WorkModule::set_int(weapon.module_accessor,0,*WEAPON_SAMUS_SUPERMISSILE_STATUS_READY_WORK_ID_INT_FRAME);
    }

    return 0.into();
}

pub fn install() {
    install_status_scripts!(
        missile_homing_main,
        missile_burst_main,
        
        supermissile_ready_init,
        supermissile_ready_exec,
    );

}