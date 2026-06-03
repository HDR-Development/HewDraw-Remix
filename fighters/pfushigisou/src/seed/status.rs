use super::*;
use globals::*;

pub unsafe extern "C" fn move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    GroundModule::set_passable_check(weapon.module_accessor, false);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(move_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(move_main_loop as *const () as _))
}

pub unsafe extern "C" fn move_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }

    return 0.into();
}

pub unsafe extern "C" fn move_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let mut life = weapon.get_param_int("param_seed", "life");
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = &mut *(*utils::util::get_battle_object_from_id(owner_id)).module_accessor;
    if [*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PLIZARDON].contains(&owner_boma.kind())
    && LinkModule::is_link(owner_boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
        //println!("owner is a pokemon, we can set the pledge state properly");
        let parent_id = LinkModule::get_parent_id(owner_boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        let pledge_state = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
        VarModule::set_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE, pledge_state);
        if [*PLEDGE_STATE_NONE, *PLEDGE_STATE_GRASS].contains(&pledge_state) {
            // No pledge
            life = 30;
        }
    }
    else if owner_boma.kind() == *FIGHTER_KIND_KIRBY {
        let pledge_state = VarModule::get_int(owner_boma.object(), vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE);
        VarModule::set_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE, pledge_state);
        if [*PLEDGE_STATE_NONE, *PLEDGE_STATE_GRASS].contains(&pledge_state) {
            // No pledge
            life = 30;
        }
    }
    else {
        //println!("ERROR: owner is not a Pokemon, things will probably crash without this failsafe");
        VarModule::set_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE, 0);
    }
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);

    let speed_x = owner_boma.stick_x() * 0.5;
    let speed_y = weapon.get_param_float("param_seed", "shoot_speed_y");
    sv_kinetic_energy!(reset_energy, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -0.102);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    
    return 0.into();
}

pub unsafe extern "C" fn move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    update_rot(weapon);
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        weapon.change_status(WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH_GROUND.into(), false.into());
        return 0.into();
    }
    if weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) == 0 {
        if VarModule::get_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE) == 3 {
            weapon.change_status(WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH.into(), false.into());
            return 0.into();
        }
        EFFECT(weapon, Hash40::new("sys_misfire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_DETACH_KIND(weapon, Hash40::new("sys_misfire"), 0);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }

    return 0.into();
}

pub unsafe extern "C" fn update_rot(weapon: &mut L2CWeaponCommon) {
    weapon.clear_lua_stack();
    weapon.push_lua_stack(&mut L2CValue::I32(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL));
    let speed_x = app::sv_kinetic_energy::get_speed_x(weapon.lua_state_agent);
    weapon.clear_lua_stack();
    weapon.push_lua_stack(&mut L2CValue::I32(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL));
    let speed_y = app::sv_kinetic_energy::get_speed_y(weapon.lua_state_agent);
    let facing = weapon.lr();
    let angle = speed_y.atan2(speed_x).to_degrees();
    PostureModule::set_rot(weapon.module_accessor, &Vector3f::new(0.0, 0.0, angle - 90.0), 0);
}

unsafe extern "C" fn move_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = weapon.get_owner_boma();
    if owner_boma.kind() == *FIGHTER_KIND_PFUSHIGISOU {
        VarModule::off_flag(owner_boma.object(), vars::pfushigisou::instance::SPECIAL_N_SEED_FIRED);
    }
    else if owner_boma.kind() == *FIGHTER_KIND_KIRBY {
        VarModule::off_flag(owner_boma.object(), vars::kirby::instance::SPECIAL_N_PFUSHIGISOU_SEED_FIRED);
    }

    return 0.into();
}

// unsafe extern "C" fn clash_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
//     StatusModule::init_settings(
//         weapon.module_accessor,
//         SituationKind(*SITUATION_KIND_NONE),
//         *WEAPON_KINETIC_TYPE_RESET,
//         *GROUND_CORRECT_KIND_NONE as u32,
//         GroundCliffCheckKind(0),
//         false,
//         0,
//         0,
//         0,
//         0
//     );

//     return 0.into();
// }

pub unsafe extern "C" fn clash_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if VarModule::get_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE) == 1 {
        // Water Pledge
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("clash_pledge_w"), 0.0, 1.0, false, 0.0, false, false);
    }
    else if VarModule::get_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE) == 3 {
        // Fire Pledge
        EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("pfushigisou_atk_hi4"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.4, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("clash_pledge_f"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("clash"), 0.0, 1.0, false, 0.0, false, false);
    }

    weapon.fastshift(L2CValue::Ptr(clash_main_loop as *const () as _))
}

pub unsafe extern "C" fn clash_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn clash_ground_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("pfushigisou_tanemg_hit"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    if VarModule::get_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE) == 1 {
        // Water Pledge
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("clash_ground"), 0.0, 1.0, false, 0.0, false, false);
        EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_splash"), Hash40::new("top"), &Vector3f::new(0.0, -2.0, 0.0), &Vector3f::zero(), 0.4, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_common_water_hit_s"), true, false, false, false, enSEType(0));
    }
    else if VarModule::get_int(weapon.battle_object, vars::pfushigisou_seed::instance::PLEDGE_TYPE) == 3 {
        // Fire Pledge
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("clash_pledge_f"), 0.0, 1.0, false, 0.0, false, false);
        EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("pfushigisou_atk_hi4"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.4, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_common_bomb_s"), true, false, false, false, enSEType(0)); 
    }
    VisibilityModule::set_whole(weapon.module_accessor, false);

    weapon.fastshift(L2CValue::Ptr(clash_ground_main_loop as *const () as _))
}

pub unsafe extern "C" fn clash_ground_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_MOVE, move_main);
    agent.status(Init, *WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_MOVE, move_init);
    agent.status(End, *WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_MOVE, move_end);

    agent.status(Main, *WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH, clash_main);

    agent.status(Main, *WEAPON_PFUSHIGISOU_SEED_STATUS_KIND_CLASH_GROUND, clash_ground_main);
}