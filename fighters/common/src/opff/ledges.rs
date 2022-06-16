use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f, Hash40};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;
use smash::app::utility::*;

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
    let player_number = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let ledge_try_pos = GroundModule::hang_cliff_pos_3f(boma) as Vector3f;

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
        *FIGHTER_STATUS_KIND_CLIFF_WAIT]) {
        VarModule::set_vec3(boma.object(), vars::common::instance::LEDGE_POS, GroundModule::hang_cliff_pos_3f(boma));
    }

    // De-occupy ledge if not on ledge anymore
    if !boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
        *FIGHTER_STATUS_KIND_CLIFF_WAIT,
        *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
        *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3]) {
        VarModule::set_vec3(boma.object(), vars::common::instance::LEDGE_POS, Vector3f {x: 0.0, y: 0.0, z: 0.0});
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
        *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3])
    && MotionModule::frame(boma) > (FighterMotionModuleImpl::get_cancel_frame(boma, Hash40::new_raw(MotionModule::motion_kind(boma)), true) * 0.75) {
        VarModule::set_vec3(boma.object(), vars::common::instance::LEDGE_POS, Vector3f {x: 0.0, y: 0.0, z: 0.0});
    }

    let current_position_x = PostureModule::pos_x(boma);
    let current_position_y = PostureModule::pos_y(boma);
    let current_position_z = PostureModule::pos_z(boma);
    let is_near_ledge = (current_position_x - ledge_try_pos.x).abs() < 10.0 && (current_position_y - ledge_try_pos.y).abs() < 50.0 && (current_position_z - ledge_try_pos.z).abs() < 10.0;


    if status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND {

        for i in 0..8 {
            if let Some(object_id) = ::utils::util::get_active_battle_object_id_from_entry_id(i) {
                let object = ::utils::util::get_battle_object_from_id(object_id);
                if !object.is_null() {
                    if i == player_number || VarModule::get_float(object, vars::common::instance::LEDGE_POS_X) == 0.0 {
                        continue;
                    }

                    if ledge_try_pos.x == VarModule::get_float(object, vars::common::instance::LEDGE_POS_X) && ledge_try_pos.y == VarModule::get_float(object, vars::common::instance::LEDGE_POS_Y) {
                        VarModule::on_flag(boma.object(), vars::common::instance::SHOULD_TRUMP_TETHER);
                    }
                    else{
                        VarModule::off_flag(boma.object(), vars::common::instance::SHOULD_TRUMP_TETHER);
                    }

                    let module_accessor = &mut *(*object).module_accessor;
                    if module_accessor.is_fighter()
                    && module_accessor.kind() == *FIGHTER_KIND_POPO {
                        let nana_object_id = WorkModule::get_int(module_accessor, *FIGHTER_POPO_INSTANCE_WORK_ID_INT_PARTNER_OBJECT_ID) as u32;
                        let object = ::utils::util::get_battle_object_from_id(nana_object_id);
                        if !object.is_null() {
                            if ledge_try_pos.x == VarModule::get_float(object, vars::common::instance::LEDGE_POS_X) && ledge_try_pos.y == VarModule::get_float(object, vars::common::instance::LEDGE_POS_Y) {
                                VarModule::on_flag(boma.object(), vars::common::instance::SHOULD_TRUMP_TETHER);
                            }
                            else{
                                VarModule::off_flag(boma.object(), vars::common::instance::SHOULD_TRUMP_TETHER);
                            }
                        }
                    }
                }
            }
        }
    }

    /*
    println!("Ledge position X: {}", VarModule::get_float(boma.object(), vars::common::instance::LEDGE_POS).x);
    println!("Player: {}", player_number);
    */
}

//=================================================================
//== TETHER TRUMP LANDING LAG
//=================================================================
unsafe fn tether_trump_landing(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    let player_number = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);

    if status_kind == *FIGHTER_STATUS_KIND_CLIFF_ROBBED {
        VarModule::on_flag(boma.object(), vars::common::instance::TETHER_HOGGED);
    }

    // Go into special fall after one action after trump
    /*
    if situation_kind == *SITUATION_KIND_AIR && prev_status_kind == *FIGHTER_STATUS_KIND_CLIFF_ROBBED {
        VarModule::off_flag(boma.object(), vars::common::instance::TETHER_HOGGED);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
    }
    */

    // Increased landing lag (special fall landing) if landing right after being tether hogged
    if /*prev_status_kind == *FIGHTER_STATUS_KIND_CLIFF_ROBBED &&*/ VarModule::is_flag(boma.object(), vars::common::instance::TETHER_HOGGED) && situation_kind == *SITUATION_KIND_GROUND {
        VarModule::off_flag(boma.object(), vars::common::instance::TETHER_HOGGED);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
    }

    if situation_kind == *SITUATION_KIND_CLIFF {
        VarModule::off_flag(boma.object(), vars::common::instance::TETHER_HOGGED);
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
        VarModule::off_flag(boma.object(), vars::common::instance::TETHER_HOGGED);
    }

    /*
    if prev_status_kind != *FIGHTER_STATUS_KIND_CLIFF_ROBBED {
        VarModule::off_flag(boma.object(), vars::common::instance::TETHER_HOGGED);
    }
    */
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    ledge_act(boma, status_kind, fighter_kind);
    occupy_ledge(boma, status_kind, situation_kind, fighter_kind);
    tether_trump_landing(boma, status_kind, situation_kind);
}