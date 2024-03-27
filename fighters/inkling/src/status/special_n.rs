use super::*;
use globals::*;

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
        let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_inkbullet"), hash40("max_life"));
        WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

        if StopModule::is_stop(weapon.module_accessor) {
            WorkModule::dec_int(weapon.module_accessor,  *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        }
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("max"), 0.0, 1.0, false, 0.0, false, false);
       // weapon.change_status(WEAPON_INKLING_INKBULLET_STATUS_KIND_NUM.into(),false.into());
        weapon.global_table[0x15].assign(&L2CValue::Ptr(bullet_max_substatus as *const () as _));
        return weapon.fastshift(L2CValue::Ptr(bullet_max_fastshift as *const () as _));
    }
    
    smashline::original_status(Main, weapon, *WEAPON_INKLING_INKBULLET_STATUS_KIND_FLY)(weapon)

}

unsafe extern "C" fn bullet_max_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn bullet_max_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life_spent = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE)-life;
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    let max_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_inkbullet"), hash40("default_speed"));
    let speed = KineticModule::get_sum_speed(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if (speed < max_speed && life_spent < 2) {
        let degree: f32 = 3.0;
        let lr = PostureModule::lr(weapon.module_accessor);
        let speed_x = degree.cos()*max_speed*-lr;
        let speed_y = degree.sin()*max_speed;
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    }
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ALL)
    || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        weapon.change_status(WEAPON_INKLING_INKBULLET_STATUS_KIND_HIT.into(),false.into());
    }

    0.into()
}

pub fn install() {
    smashline::Agent::new("inkling")
        .status(Main, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_main)
    .install();
    Agent::new("inkling_inkbullet")
        .status(Main, *WEAPON_INKLING_INKBULLET_STATUS_KIND_FLY, bullet_fly_main)
    .install();
}
