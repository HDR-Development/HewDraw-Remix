use super::*;

// FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD //

unsafe extern "C" fn special_hi_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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

pub unsafe extern "C" fn pre_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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

pub unsafe extern "C" fn pre_specialhi(fighter: &mut L2CFighterCommon) -> L2CValue {
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

pub unsafe extern "C" fn special_hi_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
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
            hash40("special_air_hi_attack_end") as i64,
            *FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_SPECIAL_AIR_END_MOTION,
        );
        fighter.change_status(
            FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END.into(),
            false.into(),
        );
        1.into()
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD, special_hi_hold_end);
    agent.status(Pre, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, pre_specialhi_end);
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, pre_specialhi);
    agent.status(Main, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK, special_hi_attack);
}