use super::*;

// WEAPON_PICKEL_PLATE_STATUS_KIND_PEARL_FLY

unsafe extern "C" fn pearl_fly_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
        0
    );

    0.into()
}

const FIGHTER_TEAM_2ND_PICKEL_TROLLEY: i32 = 0x1f;

unsafe extern "C" fn pearl_fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner = utils::util::get_battle_object_from_id(owner_id as u32);
    let owner_boma = &mut *(*owner).module_accessor;
    let owner_lr = PostureModule::lr(owner_boma);
    // stores the owner id so the correct person gets teleported, even if it switches owners mid-status
    VarModule::set_int(weapon.battle_object, vars::pickel_trolley::instance::OWNER_ID, owner_id);

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("pearl_fly"), 0.0, 1.0, false, 0.0, false, false);
    let offset = Vector3f {
        x: PostureModule::pos_x(owner_boma) + (2.0 * owner_lr),
        y: PostureModule::pos_y(owner_boma) + 9.0,
        z: 0.0
    };
    PostureModule::set_pos(weapon.module_accessor, &offset);
    
    // defines flight physics
    let init_speed_x = 2.3 * owner_lr;
    let init_speed_y = 1.0 + (ControlModule::get_stick_y(owner_boma) * 0.95);
    let accel_x = -0.025 * owner_lr;
    let accel_y = -0.08;
    let speed_x_max = 2.3;
    let speed_y_max = 3.0;
    let speed_x_brake = 2.0;
    let speed_y_brake = 1.0;
    sv_kinetic_energy!(reset_energy, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, ENERGY_CONTROLLER_RESET_TYPE_MOVE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, init_speed_x, init_speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x, accel_y);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x_max, speed_y_max);
    sv_kinetic_energy!(set_brake, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x_brake, speed_y_brake);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    
    // turn off minecart model
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("trolley_1"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("trolley_2"), false);
    
    // // set the pearl's team so steve cannot hit it
    // let entry_id = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    // let team = entry_id + FIGHTER_TEAM_2ND_PICKEL_TROLLEY;
    // TeamModule::set_team_second(owner_boma, team);
    // TeamModule::set_hit_team_second(owner_boma, team);

    // enable jostle (for windboxes)
    JostleModule::set_status(weapon.module_accessor, true);
    JostleModule::set_weight(weapon.module_accessor, 10.0); // will have to play with this

    // prevent rails from spawning
    ArticleModule::remove_exist(weapon.module_accessor, *WEAPON_PICKEL_TROLLEY_GENERATE_ARTICLE_RAIL, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

    weapon.fastshift(L2CValue::Ptr(pearl_fly_main_loop as *const () as _))
}

unsafe extern "C" fn pearl_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // tracks pearl's current position
    let mut pos = Vector3f { 
        x: PostureModule::pos_x(weapon.module_accessor), 
        y: PostureModule::pos_y(weapon.module_accessor),
        z: 0.0  
    };

    let owner_id = VarModule::get_int(weapon.battle_object, vars::pickel_trolley::instance::OWNER_ID) as u32;
    let owner = utils::util::get_battle_object_from_id(owner_id);
    let owner_boma = &mut *(*owner).module_accessor;

    // conditions for triggering the teleport
    let trigger = 
        if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT)
            { "infliction" }
        else if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) 
            { "ground" }
        else if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32)
        || GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32)
        || AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_SHIELD)
            { "wall" }
        else if DamageModule::damage(weapon.module_accessor, 0) > 0.0
            { "damaged" }
        else
            { "none" };

    if trigger == "none" {
        return 0.into();
    }

    // play effects
    PLAY_SE(weapon, Hash40::new("se_pickel_final07"));
    EFFECT(weapon, Hash40::new("pickel_erace_smoke"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 0.82, 0, 0, 0, -90, 0, 0, true);
    LAST_EFFECT_SET_COLOR(weapon, 0.9, 0.2, 0.9);
    // teleport and inflict damage
    //if trigger == "infliction" { pos.y -= 5.0 }; // position slightly lower on hit (has clipping issues, re-assess later)
    PostureModule::set_pos(owner_boma, &pos);
    PostureModule::init_pos(owner_boma, &pos, true, true);
    DamageModule::add_damage(owner_boma, 4.0, 0);
    if trigger == "ground" {
        GroundModule::correct(owner_boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    let new_status = match trigger {
        "wall" => *FIGHTER_STATUS_KIND_FALL_SPECIAL,
        "infliction" => *FIGHTER_STATUS_KIND_FALL,
        "damaged" => *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        _ => 0
    };
    if new_status != 0 {
        StatusModule::change_status_force(owner_boma, new_status, true); 
    }

    // remove the article
    weapon.clear_lua_stack();
    weapon.push_lua_stack(&mut L2CValue::new_int(0x199c462b5d));
    app::sv_battle_object::notify_event_msc_cmd(weapon.lua_state_agent);
    weapon.pop_lua_stack(1).get_bool();

    return 1.into();
}

unsafe extern "C" fn pearl_fly_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // re-enables the minecart's model
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("trolley_1"), true);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("trolley_2"), true);
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, WEAPON_PICKEL_TROLLEY_STATUS_KIND_PEARL_FLY, pearl_fly_pre);
    agent.status(Main, WEAPON_PICKEL_TROLLEY_STATUS_KIND_PEARL_FLY, pearl_fly_main);
    agent.status(End, WEAPON_PICKEL_TROLLEY_STATUS_KIND_PEARL_FLY, pearl_fly_end);
}