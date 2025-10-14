use utils::{
    *,
    ext::*,
    consts::*
};
use utils::consts::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;

//=================================================================
//== DITCIT
//=================================================================
// unsafe fn ditcit(boma: &mut BattleObjectModuleAccessor) {
//     if boma.is_status(*FIGHTER_STATUS_KIND_ITEM_THROW_DASH) {
//         if boma.status_frame() > 2 && boma.status_frame() < 6
//             && ((boma.is_cat_flag(Cat1::AttackHi4))
//              || (boma.is_cat_flag(Cat1::AttackLw4))
//              || (boma.is_cat_flag(Cat1::AttackS4))
//              || (boma.is_cat_flag(Cat1::AttackHi3))
//              || (boma.is_cat_flag(Cat1::AttackLw3))
//              || (boma.is_cat_flag(Cat1::AttackS3))) {
//             StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, false);
//         }
//     }
    
//     if boma.is_status(*FIGHTER_STATUS_KIND_ITEM_THROW)
//     && boma.is_prev_status(*FIGHTER_STATUS_KIND_ITEM_THROW_DASH)
//     && StatusModule::is_changing(boma) {
//         let added_speed_x = 2.5 * (MotionModule::end_frame(boma) - MotionModule::frame(boma)) / MotionModule::end_frame(boma);
//         let added_speed = Vector3f{x: added_speed_x, y: 0.0, z: 0.0};

//         KineticModule::add_speed(boma, &added_speed);
//     }
// }

//=================================================================
//== ANTI-FOOTSTOOL DEGENERACY TECH
//=================================================================
unsafe fn footstool_defense(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    // Shield cancel grounded footstool recoil after being ground footstooled and then receiving
    // histun beforehand
    let prev_status_0 = StatusModule::prev_status_kind(boma, 0);
    let prev_status_1 = StatusModule::prev_status_kind(boma, 1);
    let prev_status_2 = StatusModule::prev_status_kind(boma, 2);
    let prev_status_3 = StatusModule::prev_status_kind(boma, 3);
    if (status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV && situation_kind == *SITUATION_KIND_GROUND)
        && (prev_status_1 == *FIGHTER_STATUS_KIND_DAMAGE)
          || (prev_status_2 == *FIGHTER_STATUS_KIND_DAMAGE_AIR && prev_status_1 == *FIGHTER_STATUS_KIND_DAMAGE)
          || (prev_status_3 == *FIGHTER_STATUS_KIND_DAMAGE && prev_status_2 == *FIGHTER_STATUS_KIND_DAMAGE_AIR && prev_status_1 == *FIGHTER_STATUS_KIND_DAMAGE) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            if situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
            }
        }
    }

    if status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV {
        // DamageModule::add_damage(boma, 100.0, 0);
    }

    
    // Prevent airdodging after a footstool until after F20
    if (status_kind == *FIGHTER_STATUS_KIND_JUMP && prev_status_0 == *FIGHTER_STATUS_KIND_TREAD_JUMP)
        || (status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL && prev_status_0 == *FIGHTER_STATUS_KIND_JUMP && prev_status_1 == *FIGHTER_STATUS_KIND_TREAD_JUMP)
        && boma.status_frame() < 20 {
        VarModule::on_flag(boma.object(), vars::common::instance::FOOTSTOOL_AIRDODGE_LOCKOUT);
    } else if VarModule::is_flag(boma.object(), vars::common::instance::FOOTSTOOL_AIRDODGE_LOCKOUT) {
        VarModule::off_flag(boma.object(), vars::common::instance::FOOTSTOOL_AIRDODGE_LOCKOUT);
    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    //jump_cancel_airdodge(boma, cat[0], status_kind, fighter_kind); // experimental, must be called before jcgrab
    // jump_cancel_grab(boma, cat[0], status_kind, fighter_kind);
    // airdodge_cancels(boma, cat[1], cat[2], status_kind, fighter_kind, facing, stick_x);
    // ditcit(boma); // original = ditcit(boma, cat1, status_kind, motion_value, motion_vec, facing);
    //dacus(boma, cat[0], status_kind, stick_y);
    footstool_defense(boma, status_kind, situation_kind);
}
