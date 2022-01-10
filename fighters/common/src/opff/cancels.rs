use crate::opff_import::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;


//=================================================================
//== JUMP CANCEL GRABS
//=================================================================
unsafe fn jump_cancel_grab(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, fighter_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) {
            if fighter_kind == *FIGHTER_KIND_POPO {
                popo_jc_grab[hdr::get_player_number(boma)] = true;
            }
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
        }
    }
}

//=================================================================
//== AIRDODGE CANCEL ZAIR AND ITEM TOSS
//=================================================================
unsafe fn airdodge_cancels(boma: &mut BattleObjectModuleAccessor, cat2: i32, cat3: i32, status_kind: i32, fighter_kind: i32, facing: f32, stick_x: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        if MotionModule::frame(boma) > 3.0 && MotionModule::frame(boma) < 41.0 {
            // Throw item
            if ItemModule::is_have_item(boma, 0) {
                if hdr::compare_cat(cat3, *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_AIR_ALL) {
                    if facing * stick_x < 0.0 {
                        PostureModule::reverse_lr(boma);
                    }
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, false);
                }
            } else { // Zair if no item toss
                if [*FIGHTER_KIND_LUCAS,
                    *FIGHTER_KIND_YOUNGLINK,
                    *FIGHTER_KIND_TOONLINK,
                    *FIGHTER_KIND_SAMUS,
                    *FIGHTER_KIND_SAMUSD,
                    *FIGHTER_KIND_SZEROSUIT,
                    *FIGHTER_KIND_LUIGI].contains(&fighter_kind) {
                    if !ItemModule::is_have_item(boma, 0) {
                       if hdr::compare_cat(cat2, *FIGHTER_PAD_CMD_CAT2_FLAG_AIR_LASSO) {
                           StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_AIR_LASSO, true);
                       }
                    }
                }
            }
        }
    }
}

//=================================================================
//== DITCIT
//=================================================================
unsafe fn ditcit(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, facing: f32) {
    let player_number = hdr::get_player_number(boma);
    let mut motion_value: f32 = 0.0;
    let mut motion_vec = Vector3f {x: 0.0, y: 0.0, z: 0.0};

    if status_kind != *FIGHTER_STATUS_KIND_ITEM_THROW {
        ditcit_sliding[player_number] = false;
    }

    if status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW_DASH {
        if MotionModule::frame(boma) > 2.0 && MotionModule::frame(boma) < 6.0
            && ((hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4))
             || (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4))
             || (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4))
             || (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3))
             || (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3))
             || (hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3))) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, false);
            ditcit_sliding[player_number] = true;
        }
    } else {
        if ditcit_sliding[player_number] {  // status_kind == ITEM_THROWN, coming from THROW_DASH
            motion_value = 2.8 * (MotionModule::end_frame(boma) - MotionModule::frame(boma)) / MotionModule::end_frame(boma);
            motion_vec.x = motion_value * facing;
            motion_vec.y = 0.0;
            motion_vec.z = 0.0;
            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }
}

//=================================================================
//== DACUS
//=================================================================
unsafe fn dacus(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        if MotionModule::frame(boma) < 10.0 {
            let is_catch = hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_CATCH);

            // Normal smash input or Z with left stick
            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) || (stick_y >= 0.7 && is_catch) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }

            if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) || (stick_y <= -0.7 && is_catch) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }

            // Adjust input window of tilts to prevent accidental smashes
            if MotionModule::frame(boma) > 2.0 {
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
                }
                if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
                }
            }
        }
    }
}

//=================================================================
//== JUMP CANCEL AIRDODGE
//=================================================================
unsafe fn jump_cancel_airdodge(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, fighter_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        if hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) && !hdr::compare_cat(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        }
    }
}

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

    let player_number = hdr::get_player_number(boma);

    // Prevent airdodging after a footstool until after F20
    if (status_kind == *FIGHTER_STATUS_KIND_JUMP && prev_status_0 == *FIGHTER_STATUS_KIND_TREAD_JUMP)
        || (status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL && prev_status_0 == *FIGHTER_STATUS_KIND_JUMP && prev_status_1 == *FIGHTER_STATUS_KIND_TREAD_JUMP)
        && MotionModule::frame(boma) < 20.0 {
        footstool_airdodge_lockout[player_number] = true;
    } else if footstool_airdodge_lockout[player_number] {
        footstool_airdodge_lockout[player_number] = false;
    }
}



pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    //jump_cancel_airdodge(boma, cat[0], status_kind, fighter_kind); // experimental, must be called before jcgrab
    // jump_cancel_grab(boma, cat[0], status_kind, fighter_kind);
    // airdodge_cancels(boma, cat[1], cat[2], status_kind, fighter_kind, facing, stick_x);
    ditcit(boma, cat[0], status_kind, facing); // original = ditcit(boma, cat1, status_kind, motion_value, motion_vec, facing);
    //dacus(boma, cat[0], status_kind, stick_y);
    footstool_defense(boma, status_kind, situation_kind);
}
