use super::*;
use globals::*;

pub const WEAPON_INKLING_INKBULLET_STATUS_KIND_MAX: i32 = 0x2;
pub const MAX_BULLET_SPEED_MUL: f32 = 1.0;

// WEAPON_INKLING_INKBULLET_STATUS_KIND_FLY

pub unsafe extern "C" fn fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = &mut *sv_battle_object::module_accessor(owner);
    let spit_check = WorkModule::get_int(owner_boma, *FIGHTER_INKLING_STATUS_SPECIAL_N_WORK_INT_BULLET_NUM) == -1;
    if spit_check {
        weapon.change_status(WEAPON_INKLING_INKBULLET_STATUS_KIND_MAX.into(),false.into());
    }
    
    smashline::original_status(Main, weapon, *WEAPON_INKLING_INKBULLET_STATUS_KIND_FLY)(weapon)

}

// WEAPON_INKLING_INKBULLET_STATUS_KIND_MAX

pub unsafe extern "C" fn max_pre(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}

pub unsafe extern "C" fn max_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_inkbullet"), hash40("max_life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    let max_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_inkbullet"), hash40("default_speed"))*MAX_BULLET_SPEED_MUL*2.0;
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_x = max_speed*-lr;
    let speed_y = max_speed;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        0.0
    );

    if StopModule::is_stop(weapon.module_accessor) {
        WorkModule::dec_int(weapon.module_accessor,  *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("max"), 0.0, 1.0, false, 0.0, false, false);
    weapon.global_table[0x15].assign(&L2CValue::Ptr(bullet_max_substatus as *const () as _));
    return weapon.fastshift(L2CValue::Ptr(bullet_max_fastshift as *const () as _));
}
unsafe extern "C" fn bullet_max_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn bullet_max_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ALL)
    || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        let lr = PostureModule::lr(weapon.module_accessor);
        let pos = *PostureModule::pos(weapon.module_accessor);
        EffectModule::detach_all(weapon.module_accessor, 5);
        let eff = EffectModule::req(
            weapon.module_accessor,
            Hash40::new("inkling_splashooter_hit"),
            &Vector3f{x: pos.x, y: pos.y, z:pos.z},
            &Vector3f{x: 0.0, y: 0.0, z:0.0},
            1.0,
            0,
            -1,
            false,
            0
        ) as u32; 
        EffectModule::set_rot(weapon.module_accessor, eff, &Vector3f{x: 0.0, y: -90.0*(lr-1.0), z:0.0});
        MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_EFFECT, Hash40::new("effect_set_ink_color"), -1);
        WorkModule::set_int(weapon.module_accessor, 0,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life_spent = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE)-life;
    if life <= 0  {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 1.into();
    }
    let speed_y = {
        weapon.clear_lua_stack();
        lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        app::sv_kinetic_energy::get_speed_y(weapon.lua_state_agent)
    };
    if speed_y.abs() > 0.0 {
        let speed_x = {
            weapon.clear_lua_stack();
            lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
            app::sv_kinetic_energy::get_speed_x(weapon.lua_state_agent)
        };
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            speed_x,
            0.0
        );
    }

    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ALL)
    || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        weapon.change_status(WEAPON_INKLING_INKBULLET_STATUS_KIND_HIT.into(),false.into());
    }

    0.into()
}

unsafe extern "C" fn max_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent)  {
    agent.status(Main, *WEAPON_INKLING_INKBULLET_STATUS_KIND_FLY, fly_main);

    agent.status(Pre, WEAPON_INKLING_INKBULLET_STATUS_KIND_MAX, max_pre);
    agent.status(Main, WEAPON_INKLING_INKBULLET_STATUS_KIND_MAX, max_main);
    agent.status(End, WEAPON_INKLING_INKBULLET_STATUS_KIND_MAX, max_end);
}
