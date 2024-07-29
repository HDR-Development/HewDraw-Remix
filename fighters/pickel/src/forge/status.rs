use super::*;

// WEAPON_PICKEL_FORGE_STATUS_KIND_PEARL_FLY

unsafe extern "C" fn pearl_fly_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        0,
        true,
        0,
        0,
        0,
        0
    );

    0.into()
}

unsafe extern "C" fn pearl_fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // defines flight physics
    let init_speed_x = 4.0;
    let init_speed_y = 3.0; // should change based on stick angle
    let accel_x = 1.0;
    let accel_y = 1.0;
    let speed_x_max = 6.0;
    let speed_y_max = 4.0;
    let speed_x_brake = 1.0;
    let speed_y_brake = 1.0;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, init_speed_x, init_speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, accel_y);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x_max, speed_y_max);
    sv_kinetic_energy!(set_brake, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x_brake, speed_y_brake);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("pearl_fly"), 0.0, 1.0, false, 0.0, false, false);
    VisibilityModule::set_whole(weapon.module_accessor, false);

    // stores the owner id so the correct person gets teleported, even if it switches owners mid-status
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    VarModule::set_int(weapon.battle_object, vars::pickel_forge::instance::OWNER_ID, owner_id);

    //weapon.off_flag(*WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_FLAG_INIT_ATTACK);

    // enable jostle (for windboxes)
    JostleModule::set_status(weapon.module_accessor, true);
    JostleModule::set_weight(weapon.module_accessor, 10.0); // will have to play with this

    weapon.fastshift(L2CValue::Ptr(pearl_fly_main_loop as *const () as _))
}

unsafe extern "C" fn pearl_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // original main_loop for the FALL status. probably won't end up using any of this, but translated just in case
    //
    // if !weapon.is_situation(*SITUATION_KIND_GROUND) {
    //     weapon.change_status(WEAPON_PICKEL_FORGE_STATUS_KIND_WAIT.into(), false.into());

    //     return 0.into();
    // }
    // if weapon.is_flag(*WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_FLAG_REFLECT)
    // || !weapon.is_flag(*WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_FLAG_UPDATE_ATTACK) { 
        
    //     return 0.into();
    // }
    // let land_num = weapon.get_int(*WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_INT_LANDING_NUM);
    // let land_num_prev = weapon.get_int(*WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_INT_LANDING_NUM_PREV);
    // if land_num == land_num_prev 
    // && weapon.is_flag(*WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_FLAG_INIT_ATTACK) {
        
    //     return 0.into();
    // }
    // if !weapon.is_flag(*WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_FLAG_INIT_ATTACK) {
    //     AttackModule::clear_impl(weapon.module_accessor, 1 ,false);
    // }
    // weapon.on_flag(*WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_FLAG_INIT_ATTACK);
    // weapon.set_int(landing_num, WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_INT_LANDING_NUM_PREV);
    // if landing_num != 1 {
    //     MotionAnimcmdModule::call_script_single(weapon.module_accessor, *FIGHTER_ANIMCMD_GAME, 0xfa7e751b6, -1);
    // } else {
    //     MotionAnimcmdModule::call_script_single(weapon.module_accessor, *FIGHTER_ANIMCMD_GAME, 0x1397d77a71, -1);
    // }

    // tracks pearl's current position
    let pos = Vector2f { 
        x: PostureModule::pos_x(weapon.module_accessor), 
        y: PostureModule::pos_x(weapon.module_accessor) ,  
    };
  
    // pearl touches the ground = teleport
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        let owner_id = VarModule::get_int(weapon.battle_object, vars::pickel_forge::instance::OWNER_ID);
        let owner = utils::util::get_battle_object_from_id(owner_id);
        let owner_boma = &mut *(*owner).module_accessor;

        PostureModule::set_pos(owner_boma, &pos);
        PostureModule::init_pos(owner_boma, &pos, true, true);

        weapon.clear_lua_stack();
        weapon.push_lua_stack(&mut L2CValue::new_int(0x199c462b5d));
        app::sv_battle_object::notify_event_msc_cmd(weapon.lua_state_agent);
        weapon.pop_lua_stack(1).get_bool();
        return 1.into();
    }

    // pearl touches a wall OR hits shield = bonk into freefall
        // sweetspots ledge

    // pearl hits something = teleport with actionability

    // pearl gets hit = teleport and take the damage

    // despawns after an amount of time. may need a separate timer to the anvil's 6 second lifetime


    return 0.into();
}

unsafe extern "C" fn pearl_fly_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, WEAPON_PICKEL_FORGE_STATUS_KIND_PEARL_FLY, pearl_fly_pre);
    agent.status(Main, WEAPON_PICKEL_FORGE_STATUS_KIND_PEARL_FLY, pearl_fly_main);
    agent.status(End, WEAPON_PICKEL_FORGE_STATUS_KIND_PEARL_FLY, pearl_fly_end);
}