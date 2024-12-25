use super::*;

// FIGHTER_KIRBY_STATUS_KIND_PURIN_SPECIAL_N

unsafe extern "C" fn purin_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_CURRY_FACE as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return false.into();
}

pub unsafe extern "C" fn purin_special_n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

pub unsafe extern "C" fn purin_special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

pub unsafe extern "C" fn purin_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("purin_special_n"), 0.0, 1.0, false, 0.0, false, false);
    fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);

    fighter.main_shift(purin_special_n_main_loop)
}

pub unsafe extern "C" fn purin_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return false.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into();
        }
    }
    if VarModule::is_flag(fighter.battle_object, vars::kirby::status::PURIN_SPECIAL_N_HIT)
    && !VarModule::is_flag(fighter.battle_object, vars::kirby::status::PURIN_SPECIAL_N_HIT_CANCEL_OK) {
        let enable_hit_cancel_frame = VarModule::get_int(fighter.battle_object, vars::kirby::status::PURIN_SPECIAL_N_ENABLE_HIT_CANCEL_FRAME);
        if MotionModule::frame(fighter.module_accessor) > enable_hit_cancel_frame as f32 {
            CancelModule::enable_cancel(fighter.module_accessor);
            VarModule::on_flag(fighter.battle_object, vars::kirby::status::PURIN_SPECIAL_N_HIT_CANCEL_OK);
        }
    }
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && !VarModule::is_flag(fighter.battle_object, vars::kirby::status::SPECIAL_N_CLEAR_CRIT) {
        VarModule::on_flag(fighter.battle_object, vars::kirby::status::SPECIAL_N_CLEAR_CRIT);
        SlowModule::set_whole(fighter.module_accessor, 8, 25);
        PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
        EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_criticalhit"), false, true, true);
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return false.into();
    }

    return false.into();
}

#[skyline::from_offset(0xb96770)]
fn copy_ability_reset(fighter: *mut Fighter, some_miifighter_bool: bool);

pub unsafe extern "C" fn purin_special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_SUB_KIND_NONE, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_starrod_bullet"), false, false);
    if VarModule::is_flag(fighter.battle_object, vars::kirby::status::SPECIAL_N_CLEAR_CRIT) {
        SlowModule::clear_whole(fighter.module_accessor);
        EffectModule::remove_screen(fighter.module_accessor, Hash40::new("bg_criticalhit"), 0);
    }
    let kirb = fighter.battle_object.cast::<Fighter>();
    copy_ability_reset(kirb, false);
    EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("kirby_star"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    PLAY_SE(fighter, Hash40::new("se_kirby_special_n05"));
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_KIRBY_STATUS_KIND_PURIN_SPECIAL_N, purin_special_n_pre);
    agent.status(Init, *FIGHTER_KIRBY_STATUS_KIND_PURIN_SPECIAL_N, purin_special_n_init);
    agent.status(Exec, *FIGHTER_KIRBY_STATUS_KIND_PURIN_SPECIAL_N, purin_special_n_exec);
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_PURIN_SPECIAL_N, purin_special_n_main);
    agent.status(End, *FIGHTER_KIRBY_STATUS_KIND_PURIN_SPECIAL_N, purin_special_n_end);
}