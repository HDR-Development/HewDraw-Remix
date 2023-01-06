use super::*;
use globals::*;
utils::import!(popo::{ics_dash});
// status script import
 
pub fn install() {
    install_status_scripts!(
        dash,
        throw
    );
}

// FIGHTER_STATUS_KIND_DASH //

#[status_script(agent = "nana", status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    popo::ics_dash(fighter)
}

#[status_script(agent = "nana", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn throw(fighter: &mut L2CFighterCommon) -> L2CValue {

    // check if we are near a ledge
    let is_near_cliff = GroundModule::is_near_cliff(fighter.boma(),35.0,100.0);

    // if near a ledge, then throw towards ledge, else throw randomly
    let throw_name;
    if is_near_cliff {
        // the side of the stage she's on 
        // TODO: don't assume that's the direction of the closest ledge 
        let center_x = GroundModule::get_center_pos(fighter.boma());
        let side = center_x.signum();

        // the direction she's facing
        let facing = PostureModule::lr(fighter.boma());

        throw_name = match side == facing {
            true => "throw_f", // if she's facing the edge
            false => "throw_b", // if she's facing away from the edge
        }
    } else {
        // force nana into a random throw
        let selected = app::sv_math::rand(hash40("fighter"), 100);
        throw_name = match selected {
            0..=24 => "throw_b",
            25..=49 => "throw_f",
            50..=74 => "throw_lw",
            _ => "throw_hi"
        };
    }

    // change into the selected motion
    MotionModule::change_motion(fighter.boma(), Hash40::new(throw_name), 0.0, 1.0, false, 0.0, false, false);

    // shift into the L2CFighterCommon's throw impl (instead of nana's default, modified impl)
    return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Throw_Main as *const () as _));
}