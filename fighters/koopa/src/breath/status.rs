use super::*;

// FIREBREATH
unsafe extern "C" fn breath_move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.boma();
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let is_fireball =  WorkModule::get_float(owner_boma,*FIGHTER_KOOPA_STATUS_BREATH_WORK_FLOAT_GENE_ANGLE) > 360.0;
    if (!is_fireball){
        return smashline::original_status(Main, weapon, *WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE)(weapon);
    }
    else{
        WorkModule::set_customize_no(weapon.module_accessor, 1, 0);
        PostureModule::set_scale(weapon.module_accessor, 1.0, false);

        MotionModule::change_motion(
            weapon.module_accessor,
            Hash40::new("max"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        let param_life =  WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("life")) as i32;
        WorkModule::set_int(weapon.module_accessor, param_life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        WorkModule::set_int(weapon.module_accessor, param_life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        
        let param_hit_decrease = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("hit_frames")) as i32;
        WorkModule::set_int(weapon.module_accessor, param_hit_decrease, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_INT_HIT_FRAME);

        KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        let lr = PostureModule::lr(weapon.module_accessor);
        let param_speed =  WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("max_speed"));
        WorkModule::set_float(weapon.module_accessor, param_speed, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_FLOAT_SPEED_MUL);
        
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            param_speed * lr,
            0.0
          );
        sv_kinetic_energy!(
            set_stable_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            -1.0,
            -1.0
        );
        sv_kinetic_energy!(
            set_accel,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            0.0
        );
        
        weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(breath_move_max_substatus as *const () as _));
        weapon.fastshift(L2CValue::Ptr(breath_move_max_main_loop as *const () as _));
    }
    0.into()
}

unsafe extern "C" fn breath_move_max_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn breath_move_max_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT)
    {
        let param_hit_decrease = WorkModule::get_int(weapon.module_accessor, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_INT_HIT_FRAME);
        WorkModule::sub_int(weapon.module_accessor, param_hit_decrease,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    let life = WorkModule::get_int(weapon.module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        EFFECT_OFF_KIND(weapon,Hash40::new("koopa_breath_m_fire"),false,false);
        EFFECT_OFF_KIND(weapon,Hash40::new("sys_damage_fire_fly"),false,false);
        AttackModule::clear_all(weapon.module_accessor);

        MotionModule::change_motion(
            weapon.module_accessor,
            Hash40::new("end"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        StatusModule::change_status_force(weapon.module_accessor, WEAPON_KOOPA_BREATH_STATUS_KIND_NONE.into(), false.into());
        return 0.into();
    }
    
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE, breath_move_main);
}