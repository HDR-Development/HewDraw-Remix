use super::*;

// BUDDY_BAYONET

/// pre status for bayonet
/// handles initialization
pub unsafe extern "C" fn bayonet_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
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
    let attack_hi3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi3_stick_y"));
    let attack_lw3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_y"));

    let mut motion = hash40("special_n_attack_s");
    if (stick_y >= attack_hi3_stick_y) {
        motion = hash40("special_n_attack_hi");
    }
    else if (stick_y <= attack_lw3_stick_y) {
        motion = hash40("special_n_attack_lw");
    }
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, false);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);

    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, 0);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, Hash40::new("attack_s3_s"), false, 0.0);

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_START_PRECEDE_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_TURN);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_SHOOT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_SHOOT_TURN);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_RESERVED_SHOOT_TURN);

    fighter.main_shift(bayonet_main_loop)
}

/// main status loop for bayonet
unsafe extern "C" fn bayonet_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        return 0.into();
    }

    /*
    //Go into fall if sliding off or ground below disappears 
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_FALL.into(), false.into());
        return 1.into();
    } */
    0.into()
}

pub unsafe extern "C" fn bayonet_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_SHOOT);
    return smashline::original_status(End, fighter, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT)(fighter);
}

pub unsafe extern "C" fn bayonet_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    return smashline::original_status(Exit, fighter, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT)(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::buddy::SPECIAL_N_BAYONET, bayonet_pre);
    agent.status(Main, statuses::buddy::SPECIAL_N_BAYONET, bayonet_main);
    agent.status(End, statuses::buddy::SPECIAL_N_BAYONET, bayonet_end);
    agent.status(Exit, statuses::buddy::SPECIAL_N_BAYONET, bayonet_exit);
}