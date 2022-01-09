use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;
use smash::app::utility::*;

use crate::utils::hdr;

use crate::vars::{ledge_pos, ledge_occupying, tether_hogged};

//=================================================================
//== LEDGE ACTIONABILITY
//=================================================================
unsafe fn ledge_act(boma: &mut BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {
    if [*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_CLIFF_WAIT].contains(&status_kind) {
        if fighter_kind != *FIGHTER_KIND_NANA {
            if MotionModule::frame(boma) > 6.0 {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
            }
        }
    }
}

//=================================================================
//== LEDGE OCCUPANCY
//=================================================================
unsafe fn occupy_ledge(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    let player_number = hdr::get_player_number(boma);
    let ledge_try_pos = GroundModule::hang_cliff_pos_3f(boma) as Vector3f;

    if status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH
        || status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE
        || status_kind == *FIGHTER_STATUS_KIND_CLIFF_WAIT {
        ledge_pos[player_number] = GroundModule::hang_cliff_pos_3f(boma);
    }

    // De-occupy ledge if not on ledge anymore
    if ![*FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
        *FIGHTER_STATUS_KIND_CLIFF_WAIT,
        *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
        *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3].contains(&status_kind) {
        ledge_pos[player_number] = Vector3f {x: 0.0, y: 0.0, z: 0.0};
    }

    // Tell the game you're currently occupying the ledge if you've been on it
    // for more than one frame to prevent false tether trumps
    if status_kind == *FIGHTER_STATUS_KIND_CLIFF_WAIT && MotionModule::frame(boma) > 0.0 {
        ledge_occupying[player_number] = true;
    } else {
        if ![*FIGHTER_STATUS_KIND_CLIFF_CATCH,
             *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
             *FIGHTER_STATUS_KIND_CLIFF_WAIT,
             *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
             *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
             *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
             *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
             *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
             *FIGHTER_STATUS_KIND_CLIFF_JUMP3].contains(&status_kind)
            && ledge_occupying[player_number] {
            ledge_occupying[player_number] = false;
        }
    }

    // Force tether chars to get ledge trumped if rewinding to a ledgehogged ledge
    /*
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    if [*FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE, *FIGHTER_STATUS_KIND_CLIFF_WAIT].contains(&status_kind)
        && !ledge_occupying[player_number]
        && [*FIGHTER_STATUS_KIND_AIR_LASSO_REWIND, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG, *FIGHTER_STATUS_KIND_AIR_LASSO_REACH].contains(&prev_status_kind) {
        println!("Tether attempting to wind to ledge... status kind: {}", status_kind);
        println!("FIGHTER_STATUS_KIND_CLIFF_CATCH: {}", *FIGHTER_STATUS_KIND_CLIFF_CATCH);
        println!("FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE: {}", *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE);
        println!("FIGHTER_STATUS_KIND_CLIFF_WAIT: {}", *FIGHTER_STATUS_KIND_CLIFF_WAIT);
        for i in 0..8 {
            if i == player_number || ledge_pos[i].x == 0.0 {
                continue;
            }

            if ledge_try_pos.x == ledge_pos[i].x && ledge_try_pos.y == ledge_pos[i].y {
                //StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, false);
            }
        }
    }
    */

    let current_position_x = PostureModule::pos_x(boma);
    let current_position_y = PostureModule::pos_y(boma);
    let current_position_z = PostureModule::pos_z(boma);
    let is_near_ledge = (current_position_x - ledge_try_pos.x).abs() < 10.0 && (current_position_y - ledge_try_pos.y).abs() < 50.0 && (current_position_z - ledge_try_pos.z).abs() < 10.0;

    //println!(" --- CURRENT PLAYER: {} ---", player_number);
    /*
    if player_number == 0 {
        println!(" --- CURRENT PLAYER: {} ---", player_number);
        println!("X position: {}", current_position_x);
        println!("Y position: {}", current_position_y);
        println!("Z position: {}", current_position_z);
        println!(" = Ledge X position: {}", ledge_try_pos.x);
        println!(" = Ledge Y position: {}", ledge_try_pos.y);
        println!(" = Ledge Z position: {}", ledge_try_pos.z);
        println!(" = = X distance: {}", (current_position_x - ledge_try_pos.x).abs());
        println!(" = = Y distance: {}", (current_position_y - ledge_try_pos.y).abs());
        println!(" = = Z distance: {}", (current_position_z - ledge_try_pos.z).abs());
    }
    */

    /*
    if (current_position_x - ledge_try_pos.x) < 2.0 && (current_position_y - ledge_try_pos.y) < 2.0 && (current_position_z - ledge_try_pos.z) < 2.0 {
        println!("Near the ledge! Player number: {}", player_number);
        println!("X position: {}", current_position_x);
        println!("Ledge position: {}", ledge_try_pos.x);
    }
    */
    /*
    if is_near_ledge {
        println!("Near ledge verified for player {}.", player_number);
    }
    */


    if [*FIGHTER_STATUS_KIND_AIR_LASSO_REWIND].contains(&status_kind) && is_near_ledge{
        /*
       println!("Tether attempting to wind to ledge... status kind: {}", status_kind);
        println!("FIGHTER_STATUS_KIND_CLIFF_CATCH: {}", *FIGHTER_STATUS_KIND_CLIFF_CATCH);
        println!("FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE: {}", *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE);
        println!("FIGHTER_STATUS_KIND_CLIFF_WAIT: {}", *FIGHTER_STATUS_KIND_CLIFF_WAIT);
        */

        for i in 0..8 {
            if i == player_number || ledge_pos[i].x == 0.0 {
                continue;
            }

            if ledge_try_pos.x == ledge_pos[i].x && ledge_try_pos.y == ledge_pos[i].y {
                println!("Sending tethering player {} into ledge trump as they are trying to rewind to an occupied ledge.", player_number);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, false);
            }
        }
    }

    /*
    if !ledge_occupying[player_number] {
        println!("Ledge not occupied: Player {}", player_number);
    }
    println!("Ledge position X: {}", ledge_pos[player_number].x);
    println!("Player: {}", player_number);
    */
}

//=================================================================
//== TETHER TRUMP LANDING LAG
//=================================================================
unsafe fn tether_trump_landing(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    let player_number = hdr::get_player_number(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);

    if status_kind == *FIGHTER_STATUS_KIND_CLIFF_ROBBED {
        tether_hogged[player_number] = true;
    }

    // Go into special fall after one action after trump
    /*
    if situation_kind == *SITUATION_KIND_AIR && prev_status_kind == *FIGHTER_STATUS_KIND_CLIFF_ROBBED {
        tether_hogged[player_number] = false;
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
    }
    */

    // Increased landing lag (special fall landing) if landing right after being tether hogged
    if /*prev_status_kind == *FIGHTER_STATUS_KIND_CLIFF_ROBBED &&*/ tether_hogged[player_number] && situation_kind == *SITUATION_KIND_GROUND {
        tether_hogged[player_number] = false;
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
    }

    if situation_kind == *SITUATION_KIND_CLIFF {
        tether_hogged[player_number] = false;
    }

    if [*FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        /* *FIGHTER_STATUS_KIND_STANDBY*/].contains(&status_kind) {
        tether_hogged[player_number] = false;
    }

    /*
    if prev_status_kind != *FIGHTER_STATUS_KIND_CLIFF_ROBBED {
        tether_hogged[player_number] = false;
    }
    */
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    ledge_act(boma, status_kind, fighter_kind);
    occupy_ledge(boma, status_kind, situation_kind, fighter_kind);
    tether_trump_landing(boma, status_kind, situation_kind);
}
