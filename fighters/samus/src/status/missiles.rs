use super::*;


#[status_script(agent = "samus_missile", status = WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn missile_homing_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);  
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if owner_kind == *FIGHTER_KIND_SAMUS {
        let samus_status = StatusModule::status_kind(owner_boma);
        if samus_status == *FIGHTER_STATUS_KIND_THROW {
            PostureModule::set_rot(weapon.module_accessor,
            &Vector3f{x:-90.90,y:0.0,z:0.0},
            0
            );

            WorkModule::set_float(weapon.module_accessor, -90.0,*WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_FLOAT_ROT);
            let rot_x = PostureModule::rot_x(weapon.module_accessor, 0);
            return original!(weapon);
        }
    }

    return original!(weapon);
}

#[status_script(agent = "samus_missile", status = WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn missile_homing_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);  
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if owner_kind == *FIGHTER_KIND_SAMUS {
        let samus_status = StatusModule::status_kind(owner_boma);
        if samus_status == *FIGHTER_STATUS_KIND_THROW {
            StatusModule::init_settings(
                weapon.module_accessor,
                app::SituationKind(*SITUATION_KIND_AIR),
                *WEAPON_KINETIC_TYPE_NONE,
                *GROUND_CORRECT_KIND_NONE as u32,
                app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
                false,
                0,
                0,
                0,
                0
            );
            return 0.into();
        }
    }
    return original!(weapon);
}
#[status_script(agent = "samus_missile", status = WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn missile_homing_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let remaining_life = WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_INT_LIFE);
    if remaining_life < 10 {
        macros::EFFECT_OFF_KIND(weapon, Hash40::new("samusd_missile_homing"), false, true);
    }
    //let rot_x = PostureModule::rot_x(weapon.module_accessor, 0);
    let rot_x = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_FLOAT_ROT);
    if rot_x >= -89.0  {
        return original!(weapon);
    }
    WorkModule::set_float(weapon.module_accessor, -90.0,*WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_FLOAT_ROT);
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("top"),  &Vector3f{x: -90.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    return 0.into();
}
#[status_script(agent = "samus_missile", status = WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn missile_homing_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //let rot_x = PostureModule::rot_x(weapon.module_accessor, 0);
    let rot_x = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_FLOAT_ROT);

    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_missile"), hash40("h_life"));
    WorkModule::set_int(weapon.module_accessor, life,*WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    
    if rot_x >= -89.0  {
        return original!(weapon);
    }
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("homing"), 0.0, 1.0, false, 0.0, false, false);
    weapon.global_table[globals::SUB_STATUS].assign(&L2CValue::Ptr(missile_homing_substatus as *const () as _));
    weapon.fastshift(L2CValue::Ptr(missile_homing_main_loop as *const () as _))
}

unsafe extern "C" fn missile_homing_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    0.into()
}

unsafe extern "C" fn missile_homing_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let despawn = WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE,0);
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) ||
    despawn {
        weapon.change_status(WEAPON_SAMUS_MISSILE_STATUS_KIND_H_BURST.into(),false.into());
        return 0.into();
    }
    PostureModule::add_pos(weapon.module_accessor, &Vector3f{x:0.0,y:2.75,z:0.0});
    //AttackModule::set_size(weapon.module_accessor, 0, 4.0);
    KineticModule::clear_speed_all(weapon.module_accessor);
    return 0.into();
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

#[status_script(agent = "samus_supermissile", status = WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_STRAIGHT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn supermissile_straight_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if PostureModule::rot_x(weapon.module_accessor, 0) == -90.0 {
        //AttackModule::set_size(weapon.module_accessor, 0, 4.0);
    }

    return 0.into();
}

pub fn install() {
    install_status_scripts!(
        missile_homing_init,
        //missile_homing_pre,
        missile_homing_main,
        missile_homing_exec,
        
        supermissile_ready_init,
        supermissile_ready_exec,
        supermissile_straight_exec,
    );

}