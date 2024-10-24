use super::*;

use vars::pickel_trolley::{
    instance::*,
    status::*
};

// WEAPON_PICKEL_TROLLEY_STATUS_KIND_PEARL_FLY

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

unsafe fn init_physics(weapon: &mut L2CWeaponCommon, lr: f32, reflect: bool) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner = utils::util::get_battle_object_from_id(owner_id as u32);
    let owner_boma = &mut *(*owner).module_accessor;

    // defines flight physics
    let init_speed_x = 2.3 * lr;
    let init_speed_y = 1.0 + if reflect { 0.0 } else { ControlModule::get_stick_y(owner_boma) * 0.95 };
    let accel_x = -0.025 * lr;
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
}

unsafe extern "C" fn pearl_fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner = utils::util::get_battle_object_from_id(owner_id as u32);
    let owner_boma = &mut *(*owner).module_accessor;
    let owner_lr = PostureModule::lr(owner_boma);
    // stores the owner id so the correct person gets teleported, even if it switches owners mid-status
    VarModule::set_int(weapon.battle_object, PEARL_OWNER_ID, owner_id);

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("pearl_fly"), 0.0, 1.0, false, 0.0, false, false);
    let offset = Vector3f {
        x: PostureModule::pos_x(owner_boma) + (2.0 * owner_lr),
        y: PostureModule::pos_y(owner_boma) + 9.0,
        z: 0.0
    };
    PostureModule::set_pos(weapon.module_accessor, &offset);

    VarModule::set_float(weapon.battle_object, PREV_LR, owner_lr);
    init_physics(weapon, owner_lr, false);
    
    // turn off minecart model
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("trolley_1"), false);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("trolley_2"), false);
    PostureModule::set_scale(weapon.module_accessor, 0.5, false);

    // prevents rails from spawning
    ArticleModule::remove_exist(weapon.module_accessor, *WEAPON_PICKEL_TROLLEY_GENERATE_ARTICLE_RAIL, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

    weapon.fastshift(L2CValue::Ptr(pearl_fly_main_loop as *const () as _))
}

unsafe extern "C" fn pearl_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // tracks the pearl's current position and travel time
    let mut pos = Vector3f { 
        x: PostureModule::pos_x(weapon.module_accessor), 
        y: PostureModule::pos_y(weapon.module_accessor),
        z: 0.0  
    };
    VarModule::inc_int(weapon.battle_object, TRAVEL_FRAMES);

    let owner_boma = weapon.boma().get_owner_boma();

    // reset trajectory if the pearl is reflected
    let reflect_count = ReflectModule::count(weapon.module_accessor) as i32;
    if reflect_count > VarModule::get_int(weapon.battle_object, REFLECT_COUNT) {
        VarModule::set_int(weapon.battle_object, REFLECT_COUNT, reflect_count);
        let lr = VarModule::get_float(weapon.battle_object, PREV_LR) * -1.0;
        VarModule::set_float(weapon.battle_object, PREV_LR, lr);
        init_physics(weapon, lr, true);

        return 0.into();
    }

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
        else if StopModule::is_hit(weapon.module_accessor)
            { "damaged" }
        else
            { "none" };

    if trigger == "none" {
        return 0.into();
    }

    // statuses where steve should not teleport
    let is_blacklist_status = owner_boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
    ]);
    
    if is_blacklist_status {
        let pos = PostureModule::pos(weapon.boma());
        EffectModule::req_2d(owner_boma, Hash40::new("pickel_erace_smoke"), pos, &Vector3f::zero(), 1.0, 0);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 1.into();
    }

    let mut is_near_floor = false;
    let mut offset = 0.0;
    if trigger == "infliction" {
        // will snap steve to the ground if he is within a certain distance above the collision
        let snap_distance = 16.0;
        let ground_distance = GroundModule::get_distance_to_floor(weapon.module_accessor, PostureModule::pos(weapon.module_accessor), snap_distance, true);
        if ground_distance != -1.0 {
            pos.y -= ground_distance;
            is_near_floor = true;
            offset = ground_distance;
        }
    }

    if trigger == "damaged" {
        // place steve more accurately to where the pearl was hit
        pos.y -= 5.0;
    }

    // play effects
    PLAY_SE(weapon, Hash40::new("se_pickel_final07"));
    ControlModule::set_rumble(owner_boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    let owner_pos = Vector2f { 
        x: PostureModule::pos_x(owner_boma), 
        y: PostureModule::pos_y(owner_boma)
    };
    let lr = PostureModule::lr(weapon.module_accessor);
    for y_pos in [7.0, 13.0] {
        // new location
        EFFECT(weapon, Hash40::new("pickel_erace_smoke"), Hash40::new("top"), 0, (y_pos - offset) * 2.0, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(weapon, 0.9, 0.2, 0.9);
        LAST_EFFECT_SET_RATE(weapon, 0.6);
        // old location
        EFFECT(weapon, Hash40::new("pickel_erace_smoke"), Hash40::new("top"), 0, (y_pos + (owner_pos.y - pos.y) - offset) * 2.0, ((owner_pos.x - pos.x) * lr) * 2.0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(weapon, 0.9, 0.2, 0.9);
        LAST_EFFECT_SET_RATE(weapon, 0.6);
    }

    // let go of any opponents steve has grabbed so that they do not teleport with him
    if owner_boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_CATCH_PULL,
        *FIGHTER_STATUS_KIND_CATCH_WAIT,
        *FIGHTER_STATUS_KIND_CATCH_ATTACK
    ]) 
    || (owner_boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_THROW,
        *FIGHTER_STATUS_KIND_CATCH
        ]) && CatchModule::is_catch(owner_boma))
    {
        let opponent = owner_boma.get_grabbed_opponent_boma() as *mut BattleObjectModuleAccessor;
        StatusModule::change_status_force(opponent, *FIGHTER_STATUS_KIND_CAPTURE_CUT, true);
        if ArticleModule::is_exist(owner_boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE) {
            ArticleModule::remove_exist(owner_boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        }
        StatusModule::change_status_force(owner_boma, *FIGHTER_STATUS_KIND_CATCH_CUT, true);
        StatusModule::change_status_force(opponent, *FIGHTER_STATUS_KIND_CAPTURE_CUT, true);
    }

    // initiate teleport
    PostureModule::set_pos(owner_boma, &pos);
    PostureModule::init_pos(owner_boma, &pos, true, true);
    if trigger == "ground" || is_near_floor {
        GroundModule::correct(owner_boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }

    // inflict damage relative to how long the pearl traveled
    let travel_frames = VarModule::get_int(weapon.battle_object, TRAVEL_FRAMES) as f32;
    let added_damage = if travel_frames <= 15.0 { 0.0 } else { (travel_frames - 15.0) / 3.0 };
    DamageModule::add_damage(owner_boma, 1.0 + added_damage, 0);

    // change steve into the respective status, if applicable
    let new_status = if !is_near_floor { match trigger {
        "wall" => *FIGHTER_STATUS_KIND_FALL_SPECIAL,
        "infliction" => *FIGHTER_STATUS_KIND_FALL,
        "damaged" => *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        _ => 0
    }} else { 0 };
    if new_status != 0 {
        StatusModule::change_status_force(owner_boma, new_status, true); 
        if trigger == "infliction" {
            // apply a slight vertical nudge to steve
            KineticModule::clear_speed_all(owner_boma);
            KineticModule::add_speed(owner_boma, &Vector3f::new(0.0, 0.4, 0.0));
        }
    }

    // remove the article
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));

    return 1.into();
}

unsafe extern "C" fn pearl_fly_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // re-enables the minecart's model
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("trolley_1"), true);
    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("trolley_2"), true);
    return 0.into();
}

unsafe extern "C" fn start_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.boma();

    let grounded = GroundModule::get_touch_line_raw(boma, GroundTouchID(*GROUND_TOUCH_ID_DOWN)) == 1;
    if !grounded {
        WorkModule::off_flag(boma, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_TYPE_AIR);
    } else {
        WorkModule::on_flag(boma, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_TYPE_AIR);
    }

    if !WorkModule::is_flag(boma, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_TYPE_AIR) {
        WorkModule::set_int(boma, -1, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_INT_NO_GRAVITY_COUNT);
        weapon.set_situation(L2CValue::I32(*SITUATION_KIND_GROUND));
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    } else {
        let frame_no_gravity_on_start = WorkModule::get_param_int(boma, hash40("param_trolley"), hash40("frame_no_gravity_on_start"));
        WorkModule::set_int(boma, frame_no_gravity_on_start, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_INT_NO_GRAVITY_COUNT);
        weapon.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }

    weapon.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
    
    TeamModule::set_hit_team(boma, *TEAM_NONE);

    if LinkModule::is_link(boma, *WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::set_attribute(boma, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address:*LINK_ATTRIBUTE_REFERENCE_PARENT_CONTROLLER as u8}, true);
    }

    if !WorkModule::is_flag(boma, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_TYPE_AIR) {
        GroundModule::set_attach_ground(boma, true);
    } else {
        GroundModule::set_attach_ground(boma, false);
    }

    weapon.fastshift(L2CValue::Ptr(start_main_loop as *const () as _))
}

unsafe extern "C" fn start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if StatusModule::is_changing(weapon.module_accessor) {
        return 0.into();
    }
    weapon.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND, *GROUND_CORRECT_KIND_AIR);
  
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, WEAPON_PICKEL_TROLLEY_STATUS_KIND_PEARL_FLY, pearl_fly_pre);
    agent.status(Main, WEAPON_PICKEL_TROLLEY_STATUS_KIND_PEARL_FLY, pearl_fly_main);
    agent.status(End, WEAPON_PICKEL_TROLLEY_STATUS_KIND_PEARL_FLY, pearl_fly_end);

    agent.status(Main, *WEAPON_PICKEL_TROLLEY_STATUS_KIND_START, start_main);
}