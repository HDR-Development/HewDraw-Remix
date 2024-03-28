use super::*;
use globals::*;
// status script import

unsafe extern "C" fn ics_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[OBJECT_ID] == FIGHTER_KIND_NANA {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_INSTANCE_WORK_ID_FLAG_ENABLE_TRANSITION_FORBID) {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
    }
    fighter.status_Dash_Main();
    0.into()
}

// FIGHTER_STATUS_KIND_DASH //

pub unsafe extern "C" fn dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Dash_Sub();
    fighter.sub_shift_status_main(L2CValue::Ptr(ics_dash_main as *const () as _))
}

// FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP //

pub unsafe extern "C" fn special_hi_jump_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn throw_nana(fighter: &mut L2CFighterCommon) -> L2CValue {

    // TODO: this check has issues.
    // - if we grab on a platform far off stage (like smashville) this check fails
    let is_near_cliff = GroundModule::is_near_cliff(fighter.boma(),30.0,30.0);

    let throw_name;
    if is_near_cliff {
        // the side of the stage she's on 
        // TODO: don't assume that's the direction of the closest ledge 
        let center_x = GroundModule::get_center_pos(fighter.boma());
        let side = center_x.signum();

        let facing = PostureModule::lr(fighter.boma());

        let selected = app::sv_math::rand(hash40("fighter"), 100); 
        throw_name = match selected {
            0..=59 => match side == facing {
                true => "throw_f", // if she's facing the edge
                false => "throw_b", // if she's facing away from the edge
            },
            _ => "throw_lw"
        };
    } else {
        // any other scenario, random weighted throw
        let selected = app::sv_math::rand(hash40("fighter"), 100);
        throw_name = match selected {
            0..=14 => "throw_b",
            15..=29 => "throw_f",
            30..=49 => "throw_lw",
            _ => "throw_hi"
        };
    }

    // change into the selected motion
    MotionModule::change_motion(fighter.boma(), Hash40::new(throw_name), 0.0, 1.0, false, 0.0, false, false);

    // shift into the L2CFighterCommon's throw impl (instead of nana's default, modified impl)
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Throw_Main as *const () as _))
}

unsafe extern "C" fn catchwait_nana_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(catchwait_nana_main_loop as *const () as _))
}

unsafe extern "C" fn catchwait_nana_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // let boma = fighter.boma();
    // let opponent_boma = boma.get_grabbed_opponent_boma();
    // let damage = DamageModule::damage(opponent_boma, 0);
    // if damage > 50.0 {
    //     fighter.change_status(FIGHTER_STATUS_KIND_CATCH_ATTACK.into(), false.into());
    //     return 0.into();
    // }
    fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_DASH, dash);
    agent.status(Exit, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_exit);
}

pub fn install_nana(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_THROW, throw_nana);
    agent.status(Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, catchwait_nana_main);
}
