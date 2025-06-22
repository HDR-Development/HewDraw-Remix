use super::*;

// BUDDY_BUDDY_BAYONET_END

pub unsafe extern "C" fn bayonet_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, vars::kirby::instance::BUDDY_SPECIAL_N_BAYONET_ACTIVE) {
        return smashline::original_status(Pre, fighter, *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_JUMP_SQUAT)(fighter);
    }

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_BUDDY_STATUS_WORK_KEEP_FLAG_SPECIAL_N_FLAG,
        *FIGHTER_BUDDY_STATUS_WORK_KEEP_FLAG_SPECIAL_N_INT,
        *FIGHTER_BUDDY_STATUS_WORK_KEEP_FLAG_SPECIAL_N_FLOAT,
        (*FS_SUCCEEDS_KEEP_SLOPE | *FS_SUCCEEDS_KEEP_VISIBILITY) as i32
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    0.into()
}

pub unsafe extern "C" fn bayonet_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_y = fighter.global_table[STICK_Y].get_f32();

    let original = smashline::original_status(Main, fighter, *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_JUMP_SQUAT)(fighter);
    if !VarModule::is_flag(fighter.battle_object, vars::kirby::instance::BUDDY_SPECIAL_N_BAYONET_ACTIVE) {
        return original;
    }

    let attack_hi3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi3_stick_y"));
    let attack_lw3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_y"));
    ControlModule::clear_command(fighter.module_accessor, false);

    let mot = hash40("buddy_special_n_attack_s");
    FighterMotionModuleImpl::change_motion_kirby_copy(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let mut motion = hash40("buddy_special_n_attack_s");
    if (stick_y >= attack_hi3_stick_y) {
        motion = hash40("buddy_special_n_attack_hi");
    }
    else if (stick_y <= attack_lw3_stick_y) {
        motion = hash40("buddy_special_n_attack_lw");
    }
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, false);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_START_PRECEDE_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_TURN);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_SHOOT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_SHOOT_TURN);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_RESERVED_SHOOT_TURN);

    fighter.sub_shift_status_main(L2CValue::Ptr(bayonet_main_loop as *const () as _))
}

/// main status loop for bayonet
unsafe extern "C" fn bayonet_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // exit if the animation is not done yet
    if MotionModule::motion_kind(fighter.module_accessor) != hash40("buddy_special_n_attack_end") {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_END) {
            let start_frame = 26.0;
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_END);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("buddy_special_n_attack_end"), start_frame, 1.0, false, 0.0, false, false);
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAT) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAT, Hash40::new("special_n_start"), false, start_frame);
            }
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, Hash40::new("special_n_start"), false, start_frame);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn bayonet_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::kirby::instance::BUDDY_SPECIAL_N_BAYONET_ACTIVE);
    return smashline::original_status(End, fighter, *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_JUMP_SQUAT)(fighter);
}

pub unsafe extern "C" fn bayonet_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::kirby::instance::BUDDY_SPECIAL_N_BAYONET_ACTIVE);
    return smashline::original_status(Exit, fighter, *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_JUMP_SQUAT)(fighter);
}

pub fn install(agent: &mut Agent) {
    /*
    agent.status(Pre, statuses::kirby::BUDDY_SPECIAL_N_BAYONET, bayonet_pre);
    agent.status(Main, statuses::kirby::BUDDY_SPECIAL_N_BAYONET, bayonet_main);
    agent.status(End, statuses::kirby::BUDDY_SPECIAL_N_BAYONET, bayonet_end);
    agent.status(Exit, statuses::kirby::BUDDY_SPECIAL_N_BAYONET, bayonet_exit); 
    */
    agent.status(Pre, *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_JUMP_SQUAT, bayonet_pre);
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_JUMP_SQUAT, bayonet_main);
    agent.status(End, *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_JUMP_SQUAT, bayonet_end);
    agent.status(Exit, *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_JUMP_SQUAT, bayonet_exit); 
}