use super::*;
use globals::*;

pub const WEAPON_INKLING_INKBULLET_STATUS_KIND_MAX: i32 = 0x2;
pub const MAX_BULLET_SPEED_MUL: f32 = 1.0;
pub unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let kinetic = KineticModule::get_kinetic_type(fighter.module_accessor);

    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_INKLING_SPECIAL_N} else {*FIGHTER_STATUS_KIND_SPECIAL_N};
    let to_return = smashline::original_status(Main, fighter, original_status)(fighter);

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        let lr = PostureModule::lr(fighter.module_accessor);
        KineticModule::change_kinetic(fighter.module_accessor, kinetic);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST_NO_CAP,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            speed_y,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }

    to_return
}
pub unsafe extern "C" fn special_n_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_INKLING_SPECIAL_N_SHOOT} else {*FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_SHOOT};
    let toreturn = smashline::original_status(Main, fighter, original_status)(fighter);
    let current_count = WorkModule::get_int(fighter.module_accessor,*FIGHTER_INKLING_STATUS_SPECIAL_N_WORK_INT_BULLET_NUM);
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && current_count == 0 {
        WorkModule::set_int(fighter.module_accessor, -1,*FIGHTER_INKLING_STATUS_SPECIAL_N_WORK_INT_BULLET_NUM);
        let fighter_ptr = fighter.global_table[0x4].get_ptr() as *mut Fighter;
        let ink_cost = FighterSpecializer_Inkling::get_sub_ink_special_n(fighter_ptr);
        let can_max_shot = super::spend_ink(fighter,ink_cost);
        if can_max_shot {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_INKBULLET, false, -1);

            let new_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_INKLING_SPECIAL_N_END} else {*FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_END};
            fighter.change_status(new_status.into(),false.into());
            return 1.into()
        }
    }

    toreturn
}

pub unsafe extern "C" fn bullet_fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = &mut *sv_battle_object::module_accessor(owner);
    let spit_check = WorkModule::get_int(owner_boma, *FIGHTER_INKLING_STATUS_SPECIAL_N_WORK_INT_BULLET_NUM) == -1;
    if spit_check {
        weapon.change_status(WEAPON_INKLING_INKBULLET_STATUS_KIND_MAX.into(),false.into());
    }
    
    smashline::original_status(Main, weapon, *WEAPON_INKLING_INKBULLET_STATUS_KIND_FLY)(weapon)

}



pub unsafe extern "C" fn bullet_max_pre(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
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

pub unsafe extern "C" fn bullet_max_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
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
unsafe extern "C" fn bullet_max_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}


pub fn install() {
    smashline::Agent::new("inkling")
        .status(Main, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_main)
    .install();
    Agent::new("inkling_inkbullet")
        .status(Main, *WEAPON_INKLING_INKBULLET_STATUS_KIND_FLY, bullet_fly_main)
        .status(Pre, WEAPON_INKLING_INKBULLET_STATUS_KIND_MAX, bullet_max_pre)
        .status(Main, WEAPON_INKLING_INKBULLET_STATUS_KIND_MAX, bullet_max_main)
        .status(End, WEAPON_INKLING_INKBULLET_STATUS_KIND_MAX, bullet_max_end)
    .install();
}
