use super::*;
use globals::*;

unsafe extern "C" fn special_lw_yeet_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CORRECT_KIND_AIR),
        false,
        0,
        0,
        0,
        0
    );

    0.into()
}

unsafe extern "C" fn special_lw_yeet_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, 90, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_lw_yeet"), 0.0, 1.0, false, 0.0, false, false);
    let facing = PostureModule::lr(weapon.module_accessor);
    //sv_kinetic_energy!(reset_energy, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 2.0 * facing, 0.0);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 2.0, 0.0);
    sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 2.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    weapon.fastshift(L2CValue::Ptr(special_lw_yeet_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_yeet_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let speed_x = sv_kinetic_energy::get_speed_x(weapon.lua_state_agent);
    println!("speed: {}", speed_x);
    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let stspeed_x = sv_kinetic_energy::get_stable_speed_x(weapon.lua_state_agent);
    weapon.clear_lua_stack();
    println!("stable speed: {}", stspeed_x);
    lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let lmspeed_x = sv_kinetic_energy::get_limit_speed_x(weapon.lua_state_agent);
    println!("limit speed: {}", lmspeed_x);
    println!();
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::master_axe::SPECIAL_LW_YEET, special_lw_yeet_pre);
    agent.status(Main, statuses::master_axe::SPECIAL_LW_YEET, special_lw_yeet_main);
}