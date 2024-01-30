use super::*;
use globals::*;
utils::import!(common::djc::attack_air_main_status);
// status script import

pub fn install() {
    install_status_scripts!(
        //lucas_attack_lw4_main,
        attack_air,
        move_exec,
        special_hi_hold_end,
        special_hi_attack, 
        //lucas_special_s_pre,
        lucas_special_n_pre,
        lucas_special_n_hold_main, // notably, do not try to install the main loop
        attack_lw4,
        pre_specialhi,
        pre_specialhi_end,
        //lucas_special_n_end
        //lucas_special_s_main
    );

    install_agent_resets!(lucas_reset);
}

// FIGHTER RESET //

#[fighter_reset]
fn lucas_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_LUCAS {
            let charge_time = ParamModule::get_int(fighter.object(), ParamType::Agent, "attack_up_charge_time");
            VarModule::set_int(fighter.object(), charge_time, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL);
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
        }
    }
}

// AttackLw4 //

#[status_script(agent = "lucas", status = FIGHTER_STATUS_KIND_ATTACK_LW4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucas_attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.main_shift(lucas_attack_lw_4_main_loop)
}

unsafe extern "C" fn lucas_attack_lw_4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    1.into()
}

// SPECIAL N //

#[status_script(agent = "lucas", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn lucas_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue{  
    if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        fighter.change_status(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE.into(), false.into());
        return 0.into();
    }
    else {
        original!(fighter)
    }
}

// SPECIAL N HOLD //

#[status_script(agent = "lucas", status = FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucas_special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //
    // OLD SPECIAL N STATUS MAIN CODE //
    //
    // let time = fighter.get_param_int("param_special_n", "time");
    // let nobang_time = fighter.get_param_int("param_special_n", "nobang_time");
    // fighter.set_int(time, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_TIME);
    // fighter.set_int(nobang_time, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_NOBANG_TIME);
    // fighter.off_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_MOT_CHANGE);

    // if !StopModule::is_stop(fighter.module_accessor) {
    //     lucas_special_n_hold_main_sub_status(fighter, false.into());
    // }
    // fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr( lucas_special_n_hold_main_sub_status as *const () as _));
    // fighter.main_shift(lucas_special_n_hold_main_loop)
    if !StopModule::is_stop(fighter.module_accessor) {
        lucas_special_n_hold_main_sub_status(fighter, false.into());
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hold"), 0.0, 1.0, false, 0.0, false, false);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr( lucas_special_n_hold_main_sub_status as *const () as _));
    fighter.main_shift(lucas_special_n_hold_main_loop)
    
}

unsafe fn lucas_special_n_check_explosion(fighter: &mut L2CFighterCommon) {
    //
    // OLD PK FREEZE EXPLOSION CODE //
    //
    // if fighter.is_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE) 
    //     && !fighter.is_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_ALREADY_GENERATED)
    // {
    //     fighter.off_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    //     fighter.on_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_ALREADY_GENERATED);
    //     ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE, false, -1);
    // }

    // if !fighter.is_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_ALREADY_GENERATED) {
    //     return;
    // }
    // WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_TIME, 0);
    // WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_NOBANG_TIME, 0);
    // if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE) {
    //     return;
    // }
    // if fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_NOBANG_TIME) <= 0 {
    //     if fighter.is_button_off(Buttons::Special) {
    //         ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), fighter.is_button_off(Buttons::Special));
    //         return;
    //     }
    // }
    // if dbg!(fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_TIME)) <= 0 {
    //     ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), fighter.is_button_off(Buttons::Special));
    //     return;
    // }
}

unsafe extern "C" fn lucas_special_n_hold_main_sub_status(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    // 
    // OLD SPECIAL N SUBSTATUS CODE //
    // 
    // if arg.get_bool() {
    //     lucas_special_n_check_explosion(fighter);
    // }
    // if !fighter.is_situation(*SITUATION_KIND_GROUND) {
    //     if !arg.get_bool() {
    //         WorkModule::dec_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME);
    //     }
    //     if fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME) > 0 {
    //         KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    //     } else {
    //         KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    //     }
    // }
    // 0.into()

    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        if !arg.get_bool() {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME);
        }
        if fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME) > 0 {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        } else {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    0.into()
}

unsafe extern "C" fn lucas_special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //
    // OLD SPECIAL N MAIN LOOP STATUS SCRIPT CODE //
    //
    // let nobang_time = fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_NOBANG_TIME);
    // if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE) 
    //     && nobang_time <= 0
    // {
    //     fighter.change_status(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE.into(), false.into());
    //     return 1.into();
    // }
    // let wait_mtrans_kind = fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_WAIT_MTRANS_KIND);
    // if StatusModule::is_changing(fighter.module_accessor) || fighter.is_situation(wait_mtrans_kind) {
    //     // else block
    //     lucas_special_n_hold_transition_g2a_kind(fighter, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_WAIT_MTRANS_KIND, *FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_MOT_CHANGE, 
    //         *FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_LUCAS_AIR_STOP_SPECIAL_N, Hash40::new("special_n_hold"), 
    //         Hash40::new("special_air_n_hold"), *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    // }
    // 1.into()

    /// let charge = "attack_up_charge_time"
    /// <is_ready_go>
    /// set_charge(charge)
    /// <once per frame>
    ///     dec_charge()
    ///     if charge == 0 then IS_ATTACK_UP = true && set_charge(charge)

    if !VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) && fighter.is_button_on(Buttons::Special) {
        if VarModule::countdown_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0) {
            let charge_time = ParamModule::get_int(fighter.object(), ParamType::Agent, "attack_up_charge_time");
            VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, charge_time);
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            VarModule::on_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
            fighter.change_status(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE.into(), false.into());
            return 1.into();
        }
    } else {
        fighter.change_status(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_END.into(), false.into());
        return 1.into();
    }
    let wait_mtrans_kind = fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_WAIT_MTRANS_KIND);
    if StatusModule::is_changing(fighter.module_accessor) || fighter.is_situation(wait_mtrans_kind) {
        // else block
        lucas_special_n_hold_transition_g2a_kind(fighter, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_WAIT_MTRANS_KIND, *FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_MOT_CHANGE, 
            *FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_LUCAS_AIR_STOP_SPECIAL_N, Hash40::new("special_n_hold"), 
            Hash40::new("special_air_n_hold"), *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    1.into()
}

unsafe extern "C" fn lucas_special_n_hold_transition_g2a_kind(
    fighter: &mut L2CFighterCommon,
    mtrans_kind_work_id: i32,
    flag_work_id: i32,
    ground_kinetic_type: i32,
    air_kinetic_type: i32,
    ground_motion_kind: Hash40,
    aerial_motion_kind: Hash40,
    ground_correct_kind: i32
) {
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, air_kinetic_type);
        if !fighter.is_flag(flag_work_id) {
            MotionModule::change_motion(fighter.module_accessor, aerial_motion_kind, 0.0, 1.0, false, 0.0, false, false);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, aerial_motion_kind, -1.0, 1.0, 0.0, false, false);
        }
        fighter.set_int(*SITUATION_KIND_GROUND, mtrans_kind_work_id);
    } else {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(ground_correct_kind));
        KineticModule::change_kinetic(fighter.module_accessor, ground_kinetic_type);
        if !fighter.is_flag(flag_work_id) {
            MotionModule::change_motion(fighter.module_accessor, ground_motion_kind, 0.0, 1.0, false, 0.0, false, false);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, ground_motion_kind, -1.0, 1.0, 0.0, false, false);
        }
        fighter.set_int(*SITUATION_KIND_AIR, mtrans_kind_work_id);
    }
    fighter.on_flag(flag_work_id);
}

// FIGHTER_STATUS_KIND_ATTACK_AIR //

#[status_script(agent = "lucas", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::djc::attack_air_main_status(fighter)
}

// WEAPON_LUCAS_PK_THUNDER_STATUS_KIND_MOVE //

#[status_script(agent = "lucas_pkthunder", status = WEAPON_LUCAS_PK_THUNDER_STATUS_KIND_MOVE, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn move_exec(weapon: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(weapon.object(), vars::lucas::status::THUNDER_LOOSE) {
        if LinkModule::get_parent_status_kind(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) as i32 != *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD {
            VarModule::on_flag(weapon.object(), vars::lucas::status::THUNDER_LOOSE);
            MotionModule::change_motion_force_inherit_frame(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, 1.0);
            return 0.into();
        }
        original!(weapon);
    }
    0.into() 
}

// FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD //

#[status_script(agent = "lucas", status = FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_hi_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if LinkModule::is_link(fighter.module_accessor, *FIGHTER_LUCAS_LINK_NO_PK_THUNDER) {
        LinkModule::unlink(fighter.module_accessor, *FIGHTER_LUCAS_LINK_NO_PK_THUNDER);
    }
    if [*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_REFLECT].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_THUNDER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    if fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_GUIDE_EFFECT_HANDLE) != 0 {
        EffectModule::detach(fighter.module_accessor, fighter.get_int(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_GUIDE_EFFECT_HANDLE) as u32, 5);
        fighter.set_int(0, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_INT_GUIDE_EFFECT_HANDLE);
    }
    0.into() 
}

// FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK //

#[status_script(agent = "lucas", status = FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64;
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        mask_flag,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

#[status_script(agent = "lucas", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_specialhi(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64;
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND as u32, //Repair later for ledge slipoffs? Anyone?
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        mask_flag,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

#[status_script(agent = "lucas", status = FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_WALL_BRAKE);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_hi"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false,
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_special_hi_attack(fighter);
    }
    fighter.global_table[SUB_STATUS]
        .assign(&L2CValue::Ptr(sub_special_hi_attack as *const () as _));
    fighter.main_shift(special_hi_attack_main)
}

unsafe extern "C" fn sub_special_hi_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    // this does...something?
    if !fighter.is_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_FALL_START) {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    } else {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::enable_energy(
                fighter.module_accessor,
                *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            );
        } else {
            KineticModule::unable_energy(
                fighter.module_accessor,
                *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            );
        }
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        AttackModule::clear_inflict_kind_status(fighter.module_accessor);
    }
    0.into()
}

unsafe extern "C" fn special_hi_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.is_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_END) {
        fighter.off_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_END);
        let attack_end_brake = fighter.get_param_float("param_special_hi", "attack_end_brake");
        let special_hi_angle = fighter.get_float(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_FLOAT_DIR);
        let brake_x =
            attack_end_brake * special_hi_angle.cos() * PostureModule::lr(fighter.module_accessor);
        let brake_y = attack_end_brake * special_hi_angle.sin();

        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, brake_x, brake_y);
        app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    }
    /*
    if fighter.is_prev_situation(*SITUATION_KIND_AIR) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            let attack_ground_speed_xmul = fighter.get_param_float("param_special_hi", "attack_ground_speed_xmul");

            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, attack_ground_speed_xmul, attack_ground_speed_xmul, attack_ground_speed_xmul);
            app::sv_kinetic_energy::mul_speed(fighter.lua_state_agent);
        }
    }
    */
    if !MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE)
            && fighter.is_situation(*SITUATION_KIND_GROUND)
        {
            let attack_ground_end_speed_xmul =
                fighter.get_param_float("param_special_hi", "attack_ground_end_speed_xmul");

            fighter.clear_lua_stack();
            lua_args!(
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                attack_ground_end_speed_xmul,
                attack_ground_end_speed_xmul,
                attack_ground_end_speed_xmul
            );
            app::sv_kinetic_energy::mul_speed(fighter.lua_state_agent);

            fighter.change_status(
                FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(),
                false.into(),
            );
            return 1.into();
        }
        // [insert stubbed redirection/bonk function here]
        // LOL good riddance fucker
        0.into()
    } else {
        fighter.set_int64(
            0x19ea19ce46,
            *FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_SPECIAL_AIR_END_MOTION,
        );
        fighter.change_status(
            FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END.into(),
            false.into(),
        );
        1.into()
    }
}


#[status_script(agent = "lucas", status = FIGHTER_STATUS_KIND_ATTACK_LW4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_lw4(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw4_common();
    fighter.main_shift(status_attacklw4_main_param)
}

unsafe extern "C" fn status_attacklw4_main_param(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo = ComboModule::count(fighter.module_accessor) as i32;
        let lw4_combo_max = 2;
        if combo < lw4_combo_max && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)  {
            ComboModule::set(fighter.module_accessor, 2);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw4_2"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}