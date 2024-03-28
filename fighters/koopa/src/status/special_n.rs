use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_N

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let can_fireball = VarModule::get_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME) <= 0;
    if (!can_fireball){
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
    else{
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_max").into(), Hash40::new("special_air_n_max").into(), false.into());
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }

        fighter.sub_shift_status_main(L2CValue::Ptr(specialnmax_main_loop as *const () as _))
    }
}

unsafe extern "C" fn specialnmax_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return L2CValue::I32(0)
    }
    
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_max").into(), Hash40::new("special_air_n_max").into(), true.into());

        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
        }
        else{
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
        }
        return 0.into();
    }

    WorkModule::set_float(fighter.module_accessor, 361.0, *FIGHTER_KOOPA_STATUS_BREATH_WORK_FLOAT_GENE_ANGLE);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_START){
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_START);
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, false, -1);
        ArticleModule::set_float(fighter.module_accessor,*FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, 361.0, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_FLOAT_ANGLE);
    }
    

    0.into()
}

unsafe extern "C" fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let can_fireball =  VarModule::get_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME) <= 0;
    if (!can_fireball){
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE_START)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE_END)
        {
            if VarModule::get_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME) < MAX_COOLDOWN {
                VarModule::inc_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME);
            }
        }
        return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
    else{
        return 0.into();
    }
}

unsafe extern "C" fn special_n_execstop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let can_fireball =  VarModule::get_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME) <= 0;
    if (!can_fireball){
        return smashline::original_status(ExecStop, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
    else{
        return 0.into();
    }
}

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
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_exec);
    agent.status(ExecStop, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_execstop);
    agent.status(Main, *WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE, breath_move_main);
}