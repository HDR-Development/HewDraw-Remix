use super::*;
use globals::*;

pub unsafe extern "C" fn meteor_start_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner = &mut *sv_battle_object::module_accessor(owner_id);
    let cloud = utils::util::get_battle_object_from_id(owner_id);

    //Angle
    let current_rock = VarModule::get_int(cloud, vars::cloud::instance::METEOR_SPAWN_NUM);
    WorkModule::set_int(weapon.module_accessor, current_rock, WEAPON_CLOUD_METEOR_INSTANCE_WORK_ID_INT_NUM);
    
    let upper_limit = METEOR_ANGLE_MAX-METEOR_ANGLE_MIN;
    let rand_angle = if (upper_limit == 0) {0} else {sv_math::rand(hash40("fighter"), upper_limit) as i32};
    let mut angle = (METEOR_ANGLE_MIN+rand_angle);

    WorkModule::set_int(weapon.module_accessor, angle, WEAPON_CLOUD_METEOR_INSTANCE_WORK_ID_INT_ANGLE);
    
    //Snap to throw position
    let offset_x = METEOR_OFFSET_X[(current_rock-1).min(METEOR_AMOUNT-1) as usize];
    let offset_y = METEOR_OFFSET_Y[(current_rock-1).min(METEOR_AMOUNT-1) as usize];
    let spawn_x = VarModule::get_float(cloud, vars::cloud::instance::METEOR_SPAWN_X)+offset_x;
    let spawn_y = VarModule::get_float(cloud, vars::cloud::instance::METEOR_SPAWN_Y)+offset_y;
    let newPos = Vector3f{x: spawn_x,y:spawn_y,z:0.0};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    0.into()
}
pub unsafe extern "C" fn meteor_start_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
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
    0.into()
}
pub unsafe extern "C" fn meteor_start_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    //Life
    let life = 1;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    //Set Motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("stay"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(meteor_start_main_status_loop as *const () as _))
}

unsafe extern "C" fn meteor_start_main_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    0.into()
}

unsafe extern "C" fn meteor_start_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        weapon.change_status(statuses::cloud::METEOR_STATUS_KIND_MOVE.into(), false.into());
        return 0.into();
    }
    0.into()
}

pub unsafe extern "C" fn meteor_move_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
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
        0,
    );
    0.into()
}

pub unsafe extern "C" fn meteor_move_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let angle = WorkModule::get_int(weapon.module_accessor, WEAPON_CLOUD_METEOR_INSTANCE_WORK_ID_INT_ANGLE) as f32;

    //Kinetics
    let lr = PostureModule::lr(weapon.module_accessor);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let speed = 2.5; //WorkModule::get_param_float(weapon.module_accessor, hash40("param_meteor"), hash40("speed"));
    let speed_x = (angle.to_radians()).cos()*speed;
    let speed_y = (angle.to_radians()).sin()*speed;
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x*lr,
        speed_y
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        -0.1
    );


    0.into()
}

pub unsafe extern "C" fn meteor_move_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    //Init life
    let life = METEOR_LIFE; //WorkModule::get_param_int(weapon.module_accessor, hash40("param_meteor"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    WorkModule::off_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED);
    
    if StopModule::is_stop(weapon.module_accessor){
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);

    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(meteor_move_main_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(meteor_move_main_status_loop as *const () as _))
}

unsafe extern "C" fn meteor_move_main_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    0.into()
}

unsafe extern "C" fn meteor_move_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    //Life
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life < 0 {
        StatusModule::change_status_force(weapon.module_accessor, statuses::cloud::METEOR_STATUS_KIND_END, false);
        return 1.into();
    }

    //Check for reflect
    let reflected = AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR);
    let was_reflected = WorkModule::is_flag(weapon.module_accessor, *WEAPON_SHEIK_NEEDLE_STATUS_WORK_FLAG_INFLICT);
    if (reflected && !was_reflected) {
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: 0.75, y: 0.75, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::mul_accel(weapon.module_accessor,  &Vector3f{x: 0.0, y: 0.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        
        //let new_life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        //WorkModule::set_int(weapon.module_accessor, new_life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

        WorkModule::on_flag(weapon.module_accessor, *WEAPON_SHEIK_NEEDLE_STATUS_WORK_FLAG_INFLICT);
        return 0.into();
    } 
    let has_hit_player = AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT);
    let has_hit_wall = GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_DOWN) as u32);
    if has_hit_wall || (has_hit_player && !METEOR_PENETRATE) {
        StatusModule::change_status_force(weapon.module_accessor, statuses::cloud::METEOR_STATUS_KIND_END, false);
        return 1.into();
    }

    0.into()
}
pub unsafe extern "C" fn meteor_move_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
pub unsafe extern "C" fn meteor_move_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub unsafe extern "C" fn meteor_end_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}
pub unsafe extern "C" fn meteor_end_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    VisibilityModule::set_whole(weapon.module_accessor, false);
    KineticModule::clear_speed_all(weapon.module_accessor);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("end"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(meteor_end_main_status_loop as *const () as _))
}
unsafe extern "C" fn meteor_end_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {    
    Agent::new("cloud_meteor")
        .status(Init, statuses::cloud::METEOR_STATUS_KIND_START, meteor_start_init)
        .status(Pre, statuses::cloud::METEOR_STATUS_KIND_START, meteor_start_pre)
        .status(Main, statuses::cloud::METEOR_STATUS_KIND_START, meteor_start_main)
        .status(End, statuses::cloud::METEOR_STATUS_KIND_START, meteor_move_end)

        .status(Init, statuses::cloud::METEOR_STATUS_KIND_MOVE, meteor_move_init)
        .status(Pre, statuses::cloud::METEOR_STATUS_KIND_MOVE, meteor_move_pre)
        .status(Main, statuses::cloud::METEOR_STATUS_KIND_MOVE, meteor_move_main)
        .status(Exec, statuses::cloud::METEOR_STATUS_KIND_MOVE, meteor_move_exec)
        .status(End, statuses::cloud::METEOR_STATUS_KIND_MOVE, meteor_move_end)
        
        .status(Pre, statuses::cloud::METEOR_STATUS_KIND_END, meteor_end_pre)
        .status(Main, statuses::cloud::METEOR_STATUS_KIND_END, meteor_end_main)
        .status(End, statuses::cloud::METEOR_STATUS_KIND_END, meteor_move_end)
        .install();
}
