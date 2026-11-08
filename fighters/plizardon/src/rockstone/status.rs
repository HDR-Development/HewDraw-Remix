use super::*;

pub unsafe extern "C" fn start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        *FS_SUCCEEDS_KEEP_ATTACK as i32,
    );

    return 0.into();
}

pub unsafe extern "C" fn start_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    VarModule::on_flag(weapon.battle_object, vars::common::status::NO_POCKET);
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    // Random angle between 300 and 100
    let num_rock = WorkModule::get_int(owner, *FIGHTER_PLIZARDON_STATUS_BREATH_WORK_INT_GENERATE_COUNT);
    let rock_updown = if num_rock % 2 == 0 { 1 } else { -1 };
    let rand_angle = sv_math::rand(hash40("fighter"), 80) as i32;
    let mut angle = 20 + rand_angle * rock_updown;
    VarModule::set_int(weapon.battle_object, vars::plizardon_rockstone::instance::ANGLE, angle);

    //Rot
    let rand_rot = sv_math::rand(hash40("fighter"), 360) as i32;
    VarModule::set_int(weapon.battle_object, vars::plizardon_rockstone::instance::ROT, rand_rot);

    //Snap to throw position
    let mut owner_pos = Vector3f::zero();
    let mut article_pos = Vector3f::zero();
    let mut offset_add = Vector3f::zero();
    let lr = PostureModule::lr(owner);
    let owner_offset = ModelModule::joint_global_offset_from_top(owner, Hash40{hash: hash40("throw")}, &mut owner_pos);
    let newPos = Vector3f::new(
        PostureModule::pos_x(owner) + owner_pos.x - article_pos.x + (offset_add.x * lr),
        PostureModule::pos_y(owner) + owner_pos.y - (article_pos.y) + offset_add.y,
        PostureModule::pos_z(owner) + owner_pos.z - article_pos.z);
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    return 0.into();
}

pub unsafe extern "C" fn start_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = 1;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("stay"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(start_main_loop as *const () as _))
}

unsafe extern "C" fn start_main_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        StatusModule::change_status_force(weapon.module_accessor, statuses::plizardon_rockstone::MOVE, false);
        return 0.into();
    }

    return 0.into();
}

pub unsafe extern "C" fn move_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK as i32,
    );
    
    return 0.into();
}

pub unsafe extern "C" fn move_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let angle = VarModule::get_int(weapon.battle_object, vars::plizardon_rockstone::instance::ANGLE) as f32;
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed = 2.0;
    let speed_x = (angle.to_radians()).cos() * speed;
    let speed_y = (angle.to_radians()).sin() * speed;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x * lr, speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -speed_x * lr * 0.05, -speed_y * 0.05);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);

    return 0.into();
}

pub unsafe extern "C" fn move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    VarModule::on_flag(weapon.battle_object, vars::common::status::NO_POCKET);
    let life = 16; //WorkModule::get_param_int(weapon.module_accessor, hash40("param_rockstone"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED);
    
    if StopModule::is_stop(weapon.module_accessor) {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(move_main_loop as *const () as _))
}

unsafe extern "C" fn move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let rot = VarModule::get_int(weapon.battle_object, vars::plizardon_rockstone::instance::ROT) as f32;
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("needle"), &Vector3f::new(rot, rot, rot), MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        rockstone_remove(weapon);
        return 0.into();
    }

    //Change LR
    let time_active = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE) - life;
    if time_active >= 4 {
        let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        PostureModule::set_lr(weapon.module_accessor, speed_x.signum());
    }

    //Check for reflect
    let reflected = AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR);
    let was_reflected = VarModule::is_flag(weapon.battle_object, vars::plizardon_rockstone::status::INFLICTED);
    if (reflected && !was_reflected) {
        KineticModule::reflect_speed(weapon.module_accessor, &Vector3f::new(0.75, 0.75, 0.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::mul_accel(weapon.module_accessor, &Vector3f::new(0.0, 0.0, 0.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        VarModule::on_flag(weapon.battle_object, vars::plizardon_rockstone::status::INFLICTED);
        return 0.into();
    } 
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        rockstone_remove(weapon);
    }

    return 0.into();
}

pub unsafe extern "C" fn move_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn move_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn rockstone_remove(weapon: &mut L2CWeaponCommon) {
    let pos = PostureModule::pos(weapon.module_accessor);
    let eff = EffectModule::req(weapon.module_accessor, Hash40::new("sys_misfire"), pos, &Vector3f::zero(), 1.0, 0, -1, false, 0) as u32;
    EffectModule::set_rgb(weapon.module_accessor, eff, 0.5, 0.5, 0.5);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::plizardon_rockstone::START, start_pre);
    agent.status(Init, statuses::plizardon_rockstone::START, start_init);
    agent.status(Main, statuses::plizardon_rockstone::START, start_main);
    agent.status(End, statuses::plizardon_rockstone::START, move_end);

    agent.status(Pre, statuses::plizardon_rockstone::MOVE, move_pre);
    agent.status(Init, statuses::plizardon_rockstone::MOVE, move_init);
    agent.status(Main, statuses::plizardon_rockstone::MOVE, move_main);
    agent.status(Exec, statuses::plizardon_rockstone::MOVE, move_exec);
    agent.status(End, statuses::plizardon_rockstone::MOVE, move_end);
}