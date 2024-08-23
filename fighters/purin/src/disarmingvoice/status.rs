use super::*;

unsafe extern "C" fn shoot_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    (0).into()
}

unsafe extern "C" fn shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(
        weapon.module_accessor,
        *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER
    ) as u32;
    let purin = utils::util::get_battle_object_from_id(owner_id);
    let purin_boma = &mut *(*purin).module_accessor;

    weapon.set_int_from_param(*WEAPON_INSTANCE_WORK_ID_INT_LIFE, "param_disarmingvoice", "life");
    weapon.set_int_from_param(*WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE, "param_disarmingvoice", "life");
    ModelModule::set_scale(weapon.module_accessor, 0.001);

    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_max = weapon.get_param_float("param_disarmingvoice", "speed_max");
    let speed_purin = KineticModule::get_sum_speed_x(purin_boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_including_momentum = lr * speed_max + speed_purin / 2.0;
    weapon.clear_lua_stack();
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_including_momentum,
        0.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_including_momentum,
        0.0
    );
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);

    let owner_pos_x = PostureModule::pos_x(purin_boma);
    let owner_pos_y = PostureModule::pos_y(purin_boma);
    let owner_pos_z = PostureModule::pos_z(purin_boma);
    PostureModule::set_pos(
        weapon.module_accessor,
        &(Vector3f { x: owner_pos_x + lr * 3.0, y: owner_pos_y + 6.0, z: owner_pos_z })
    );

    VarModule::on_flag(weapon.battle_object, vars::common::status::HIT_EFFECT_DROP_ITEM);
    return false.into();
}

unsafe extern "C" fn shoot_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("shoot"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    weapon.fastshift(L2CValue::Ptr(shoot_main_loop as *const () as _))
}

unsafe extern "C" fn shoot_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos = *PostureModule::pos(weapon.module_accessor);
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) || life <= 0 {
        EffectModule::req(
            weapon.module_accessor,
            Hash40::new("sys_erace_smoke"),
            &(Vector3f { x: pos.x, y: pos.y, z: pos.z + 5.0 }),
            &Vector3f::zero(),
            1.0,
            0,
            -1,
            false,
            0
        );
        EffectModule::kill_kind(
            weapon.module_accessor,
            Hash40::new("poke_meloetta_bullet"),
            false,
            false
        );
        EffectModule::kill_kind(
            weapon.module_accessor,
            Hash40::new("rosetta_ring_erase"),
            false,
            false
        );
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    if StopModule::is_stop(weapon.module_accessor) {
        EffectModule::req(
            weapon.module_accessor,
            Hash40::new("sys_flash"),
            &(Vector3f { x: pos.x, y: pos.y, z: pos.z + 5.0 }),
            &Vector3f::zero(),
            1.0,
            0,
            -1,
            false,
            0
        );
        EffectModule::kill_kind(
            weapon.module_accessor,
            Hash40::new("poke_meloetta_bullet"),
            false,
            false
        );
        EffectModule::kill_kind(
            weapon.module_accessor,
            Hash40::new("rosetta_ring_erase"),
            false,
            false
        );
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    return false.into();
}

unsafe extern "C" fn shoot_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    return false.into();
}

unsafe extern "C" fn shoot_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(
        weapon.module_accessor,
        Hash40::new("poke_meloetta_bullet"),
        false,
        false
    );
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::purin_disarming_voice::SHOOT, shoot_pre);
    agent.status(Init, statuses::purin_disarming_voice::SHOOT, shoot_init);
    agent.status(Main, statuses::purin_disarming_voice::SHOOT, shoot_main);
    agent.status(Exec, statuses::purin_disarming_voice::SHOOT, shoot_exec);
    agent.status(End, statuses::purin_disarming_voice::SHOOT, shoot_end);
}
