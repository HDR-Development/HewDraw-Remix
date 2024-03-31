use super::*;

// BUDDY_BAYONET_END

/// pre status for bayonet
/// handles initialization
pub unsafe extern "C" fn bayonet_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_BIND,
        *GROUND_CORRECT_KIND_GROUND as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );

    0.into()
}

/// main status loop for bayonet_end
unsafe extern "C" fn bayonet_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // exit if the animation is not done yet
    if !MotionModule::is_end(fighter.module_accessor) {
        return 0.into();
    }

    // if the animation is over, transition to shoot
    fighter.change_status(FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
    1.into()
}

pub unsafe extern "C" fn bayonet_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // change summon anim depending on LR
    let frame = 26.0;
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_shoot_start"), frame, 1.0, false, 0.0, false, false);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, Hash40::new("special_n_start"), false, frame);

    fighter.main_shift(bayonet_end_main_loop)
}

pub unsafe extern "C" fn bayonet_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // re-enable energies and remove the screenwide effect
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    let next_status = StatusModule::status_kind_next(fighter.module_accessor);
    let is_still_blasting = [*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_F,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_B,
    *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_TURN,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP_SQUAT,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_LANDING,
    *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP_AERIAL,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_AIR,
    *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_FALL,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_AIR_TURN,*FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_END].contains(&next_status);

    if !is_still_blasting {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, ArticleOperationTarget(0));
        ItemModule::set_change_status_event(fighter.module_accessor, true);
        WorkModule::set_int(fighter.module_accessor, 0,*FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::buddy::BUDDY_BAYONET_END, bayonet_end_pre);
    agent.status(Main, statuses::buddy::BUDDY_BAYONET_END, bayonet_end_main);
    agent.status(End, statuses::buddy::BUDDY_BAYONET_END, bayonet_end_end);
}