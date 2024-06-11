// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// Mii Swordfighter Airborne Assault Aerial FAF Frame 75
unsafe fn airborne_assault_lag(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    if fighter.is_status(*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_END) {
        if  fighter.is_situation(*SITUATION_KIND_AIR) && fighter.motion_frame() > 76.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        } 
    }
}

// unsafe fn gale_stab_jc_attack(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
//     // Rush
//     if [*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH].contains(&status_kind) {
//         // Jump and Attack cancels
//         let pad_flag = ControlModule::get_pad_flag(boma);
//         if boma.check_jump_cancel(true, false) {
//             return;
//         }
//         if compare_mask(pad_flag, *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) || compare_mask(pad_flag, *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) {
//             StatusModule::change_status_request_from_script(boma, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK, true);
//         }
//         // Wall Jump
//         if situation_kind == *SITUATION_KIND_AIR {
//             if !VarModule::is_flag(fighter.battle_object, vars::common::instance::SPECIAL_WALL_JUMP) {
//                 let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
//                 let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
//                 if touch_left || touch_right {
//                     if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH | *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) {
//                         VarModule::on_flag(fighter.battle_object, vars::common::instance::SPECIAL_WALL_JUMP);
//                         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
//                     }
//                 }
//             }
//         }
//     }
//     // Attack
//     if [*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK].contains(&status_kind) {
//         // Jump cancels
//         let pad_flag = ControlModule::get_pad_flag(boma);
//         if boma.status_frame() > 6 && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag() {
//             boma.check_jump_cancel(true, false);
//         }
//     }
// }

// Mii Swordfighter Aerial Power Thrust Jump Reset
// unsafe fn aerial_power_thrust_jump_reset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64) {
//     if motion_kind == hash40("special_lw3") || status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END {
//         if boma.get_num_used_jumps() == boma.get_jump_count_max() {
//             WorkModule::set_int(boma, boma.get_jump_count_max() - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
//         }
//     }
// }

// Mii Swordfighter Hero's Spin Movement
// unsafe fn heros_spin_movement(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, frame: f32) {
//     let valueMii = 0.7;
//     let valueWalk = 0.28;
//     let motion_value = valueMii;
//     if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_HOLD {
//         if situation_kind == *SITUATION_KIND_GROUND {
//             if stick_x != 0.0 {
//                 let motion_vec = x_motion_vec(valueWalk, stick_x);
//                 KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
//             }
//         }
//     }
//     /*if motion_kind == hash40("special_hi3") || status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_END {
//         if situation_kind == *SITUATION_KIND_GROUND {
//             if frame < 46.0 {
//                 if stick_x != 0.0 {
//                     let motion_vec = x_motion_vec(motion_value, stick_x);
//                     KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
//                 }
//             }
//         }
//     }*/
// }

// Land cancel stuff
// unsafe fn land_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, motion_kind: u64) {
//     // Activate Land cancel flag
//     if motion_kind == hash40("special_hi3") {
//         VarModule::on_flag(fighter.battle_object, vars::common::instance::SPIN_ATTACK_LAND_CANCEL);
//     }
//     // Reset Land cancel flag
//     if !(motion_kind == hash40("special_hi3") || motion_kind == hash40("special_air_hi3")) {
//         VarModule::off_flag(fighter.battle_object, vars::common::instance::SPIN_ATTACK_LAND_CANCEL);
//     }
//     // Land cancel
//     if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL && VarModule::is_flag(fighter.battle_object, vars::common::instance::SPIN_ATTACK_LAND_CANCEL) {
//         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
//     }
// }

// Attacks out of Aerial Assault
// unsafe fn aerial_acrobatics(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, motion_kind: u64, frame: f32) {
//     if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK {
//         if motion_kind == hash40("special_s1") || (motion_kind == hash40("special_air_s1") && frame >= 16.0) {
//             VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
//             if boma.is_cat_flag(Cat1::AttackN) {
//                 StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
//             }
//             if (boma.is_cat_flag(Cat1::AttackS3) && boma.is_stick_forward())
//                 || (boma.is_cat_flag(Cat1::AttackS4) && boma.is_stick_forward()) {
//                 StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
//             }
//             if (boma.is_cat_flag(Cat1::AttackS3) && boma.is_stick_backward())
//                 || (boma.is_cat_flag(Cat1::AttackS4) && boma.is_stick_backward()) {
//                 StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
//             }
//             if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
//                                     | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
//                 StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
//             }
//             if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
//                                     | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
//                 StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
//             }
//         }
//     }
// }

// Skyward Slash Dash actionability
unsafe fn skyward_slash_dash_act(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
	if boma.is_status(*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(boma.object(), vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT);
        }
    }
    if boma.is_status(*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH_END) {
        if VarModule::is_flag(boma.object(), vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT)
        && !VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK)
        && boma.is_situation(*SITUATION_KIND_AIR) {
            if boma.status_frame() >= 31 {
                VarModule::off_flag(boma.object(), vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT);
                VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

// Kinesis Blade OPFF stuff
// unsafe fn kinesis_blade(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64) {
// 	if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && (motion_kind == hash40("special_lw1") || motion_kind == hash40("special_air_lw1")){
//         if VarModule::get_int(boma.object(), vars::miiswordsman::instance::SPECIAL_LW1_CHARGE_LEVEL) > 0 {
//             //println!("Kinesis ready");
//             if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
//                 //println!("Kinesis activation");
//                 VarModule::on_flag(boma.object(), vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER);
//                 StatusModule::change_status_request(boma, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT, false);
//             }
//         }
//         /*
//         if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_GUARD) {
//             println!("Kinesis anim test");
//             MotionModule::change_motion(boma, Hash40::new("special_lw1_hit_lv2"), 0.0, 1.0, false, 0.0, false, false);
//         }
//         */
//     }
// }

// Transition into hitgrab on hit
// unsafe fn hitgrab_transition(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64) {
//     if StatusModule::is_changing(boma) {
//         return;
//     }
// 	if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && ((motion_kind == hash40("special_lw3") && MotionModule::frame(boma) > 17.0) || (motion_kind == hash40("special_air_lw3") && MotionModule::frame(boma) > 11.0)) {
//         if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !fighter.is_in_hitlag() {
//             //println!("Swordfighter gon' give it to ya");
//             StatusModule::change_status_request(boma, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END, false);
//         }
//     }
// }

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && (
        fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
        ])
        || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_1,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_1,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1
            ].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO))
            && fighter.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_SPECIAL_N,
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT
            ])
        )
        || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_2,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_2,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2
            ].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO))
            && fighter.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_SPECIAL_N,
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END,
                *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK,
                *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH_END,
                *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_BOUND
            ])
        )
        || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_3,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_3,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_3,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3
            ].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO))
            && ( fighter.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_SPECIAL_N,
                *FIGHTER_STATUS_KIND_SPECIAL_S,
                *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END,
                *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_LOOP,
                *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END_MAX,
            ])
            || (fighter.is_motion(Hash40::new("special_air_hi3")) && fighter.motion_frame() > 49.0) )
        )
    )
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    skyward_slash_dash_act(boma);
    airborne_assault_lag(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn miiswordsman_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		miiswordsman_frame(fighter)
    }
}

pub unsafe fn miiswordsman_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, miiswordsman_frame_wrapper);
}