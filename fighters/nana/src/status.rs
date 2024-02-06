use super::*;
use globals::*;
utils::import!(popo::{ics_dash});
// status script import
 


// FIGHTER_STATUS_KIND_DASH //


pub unsafe extern "C" fn dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    popo::ics_dash(fighter)
}


pub unsafe extern "C" fn throw(fighter: &mut L2CFighterCommon) -> L2CValue {

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
    return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Throw_Main as *const () as _));
}


unsafe extern "C" fn catchwait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(catchwait_main_loop as *const () as _))
}

unsafe extern "C" fn catchwait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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

// uncomment this to prevent buffering pummels with nana
// #[status_script(agent = "nana", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn catchattack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.sub_shift_status_main(L2CValue::Ptr(catchattack_main_loop as *const () as _))
// }

// unsafe extern "C" fn catchattack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
//     0.into()
// }

pub fn install() {
    smashline::Agent::new("nana")
        .status(Main, *FIGHTER_STATUS_KIND_DASH, dash)
        .status(Main, *FIGHTER_STATUS_KIND_THROW, throw)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, catchwait_main)
        .install();
}