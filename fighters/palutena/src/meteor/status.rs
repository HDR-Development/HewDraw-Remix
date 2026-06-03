use super::*;

unsafe extern "C" fn move_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    
    return 0.into();
}

unsafe extern "C" fn move_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let palutena = utils::util::get_battle_object_from_id(owner_id);
    let palutena_boma = &mut *(*palutena).module_accessor;
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_meteor"), hash40("life_s"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let owner_pos_x = PostureModule::pos_x(palutena_boma);
    let owner_pos_y = PostureModule::pos_y(palutena_boma);
    let owner_pos_z = PostureModule::pos_z(palutena_boma);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    ModelModule::set_scale(weapon.module_accessor, 2.65);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f::new(owner_pos_x + 5.0 * lr, owner_pos_y + 40.0, owner_pos_z));
    speed_reset(weapon);

    return 0.into();
}

unsafe extern "C" fn move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    VarModule::on_flag(weapon.object(), vars::common::status::NO_POCKET);
    weapon.fastshift(L2CValue::Ptr(move_main_loop as *const () as _))
}

unsafe extern "C" fn move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos = *PostureModule::pos(weapon.module_accessor);
    if life <= 0 {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f::new(pos.x, pos.y, pos.z), &Vector3f::zero(), 1.0, 0, -1, false, 0);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_crown_collision"), &Vector3f::new(pos.x, pos.y - 3.0, pos.z), &Vector3f::zero(), 0.75, 0, -1, false, 0);
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f::new(pos.x, pos.y, pos.z), &Vector3f::zero(), 1.0, 0, -1, false, 0);
        let handle = SoundModule::play_se(weapon.module_accessor, Hash40::new("se_item_pasaran_landing"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(weapon.module_accessor, handle as i32, 2.0, 0);
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_common_down_soil_ss"), true, false, false, false, app::enSEType(0));
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    else if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f::new(pos.x, pos.y, pos.z), &Vector3f::zero(), 1.0, 0, -1, false, 0);
        let handle = SoundModule::play_se(weapon.module_accessor, Hash40::new("se_item_pasaran_landing"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(weapon.module_accessor, handle as i32, 2.0, 0);
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_common_down_soil_ss"), true, false, false, false, app::enSEType(0));
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }

    return 0.into();
}

unsafe extern "C" fn move_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if KineticModule::get_sum_speed_y(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL).abs() < 0.1 {
        speed_reset(weapon);
    }
    return 0.into();
}

unsafe extern "C" fn move_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn speed_reset(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let angle: f32 = 66.0;
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_meteor"), hash40("speed_s"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_x = angle.to_radians().cos() * speed_max * lr;
    let speed_y = angle.to_radians().sin() * speed_max;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::palutena_meteor::MOVE, move_pre);
    agent.status(Init, statuses::palutena_meteor::MOVE, move_init);
    agent.status(Main, statuses::palutena_meteor::MOVE, move_main);
    agent.status(Exec, statuses::palutena_meteor::MOVE, move_exec);
    agent.status(End, statuses::palutena_meteor::MOVE, move_end);
}