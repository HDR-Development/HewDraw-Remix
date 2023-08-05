// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn final_cutter_landing_bugfix(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2)
    && MotionModule::frame(fighter.module_accessor) <= 2.0 {
        fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
    }
}

unsafe fn horizontal_cutter(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        if (((fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.status_frame() == 15)
            || (fighter.is_situation(*SITUATION_KIND_AIR) && fighter.status_frame() == 17))
            && ControlModule::get_stick_x(fighter.module_accessor).abs() >= 0.85)
            && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < 0.0 {
                REVERSE_LR(fighter);
            }
            let hcutter_status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::kirby::SPECIAL_HI_H);
            StatusModule::change_status_request_from_script(fighter.module_accessor, hcutter_status, false);
        }
    }
}

// unsafe fn disable_dash_attack_slideoff(fighter: &mut L2CFighterCommon) {
//     if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_DASH) && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
//         VarModule::off_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_FALL);
//         VarModule::off_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_CONTINUE);
//     }
// }

// unsafe fn stone_control(fighter: &mut L2CFighterCommon) {
//     if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.status_frame() <= 30 {
//         fighter.clear_lua_stack();
//         lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.0, 0.0);
//         app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
//         fighter.clear_lua_stack();
//     }
// }

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
pub fn hammer_fastfall_landcancel(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.is_prev_situation(*SITUATION_KIND_AIR) {
                AttackModule::clear_all(fighter.module_accessor);
                MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), 33.0, 1.0, 1.0);
                MotionModule::set_rate(fighter.module_accessor, (55.0 - 33.0)/25.0);    // equates to 17F landing lag
            }
            if fighter.is_situation(*SITUATION_KIND_AIR) {
                if fighter.is_cat_flag(Cat2::FallJump) && fighter.stick_y() < -0.66 && KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                    WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
    }
}

// Magic Series
unsafe fn magic_series(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    let cat1 = cat[0];
    let cat4 = cat[3];
    if(    (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_RYU)
        || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_KEN)
        || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCARIO)
        || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_DOLLY)){
        // Level 1: Jab and Dash Attack Cancels
        if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind) {
            if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
                || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {
                // Check for tilt attack inputs
                if boma.is_cat_flag(Cat1::AttackS3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3,false);
                }
                if boma.is_cat_flag(Cat1::AttackHi3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3,false);
                }
                if boma.is_cat_flag(Cat1::AttackLw3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3,false);
                }

                // Check for smash attack inputs
                if boma.is_cat_flag(Cat1::AttackS4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
                }
                if boma.is_cat_flag(Cat1::AttackHi4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
                }
                if boma.is_cat_flag(Cat1::AttackLw4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
                }

                // Check for special attack inputs
                if boma.is_cat_flag(Cat1::SpecialN) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                }
                if boma.is_cat_flag(Cat1::SpecialS) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                }
                if boma.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                }
                if boma.is_cat_flag(Cat1::SpecialLw) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                }

                // Check for jump inputs during dash attack (on hit)
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
                && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                && !boma.is_in_hitlag()) {
                    boma.check_jump_cancel(false);
                }

                //----------------------------------------------------------------------------------------------------------------------
                // Check for command inputs
                // Ryu
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
                    if boma.is_cat_flag( Cat4::SpecialNCommand) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
                    }
                    if boma.is_cat_flag( Cat4::SpecialN2Command) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
                    }
                }
                // Ken
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    if boma.is_cat_flag( Cat4::AttackCommand1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND, false);
                    }
                }
                //----------------------------------------------------------------------------------------------------------------------

            }
        }

        // Level 2: Tilt Cancels
        if [*FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
            if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
                || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {
                // Check for smash attack inputs
                if boma.is_cat_flag(Cat1::AttackS4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
                }
                if boma.is_cat_flag(Cat1::AttackHi4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
                }
                if boma.is_cat_flag(Cat1::AttackLw4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
                }

                // Check for special attack inputs
                if boma.is_cat_flag(Cat1::SpecialN) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                }
                if boma.is_cat_flag(Cat1::SpecialS) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                }
                if boma.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                }
                if boma.is_cat_flag(Cat1::SpecialLw) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                }

                // Check for jump inputs during utilt
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
                && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                && !boma.is_in_hitlag()) {
                    boma.check_jump_cancel(false);
                }

                //----------------------------------------------------------------------------------------------------------------------
                // Check for command inputs
                // Ryu
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
                    if boma.is_cat_flag( Cat4::SpecialNCommand) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
                    }
                    if boma.is_cat_flag( Cat4::SpecialN2Command) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
                    }
                }
                // Ken
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    if boma.is_cat_flag( Cat4::AttackCommand1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND, false);
                    }
                }
                //----------------------------------------------------------------------------------------------------------------------

            }
        }

        // Smash Cancels
        if [*FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
            if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
                || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {

                // Check for special attack inputs
                if boma.is_cat_flag(Cat1::SpecialN) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                }
                if boma.is_cat_flag(Cat1::SpecialS) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                }
                if boma.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                }
                if boma.is_cat_flag(Cat1::SpecialLw) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                }

                // Check for jump inputs
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
                && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                && !boma.is_in_hitlag()) {
                    boma.check_jump_cancel(false);
                }

                //----------------------------------------------------------------------------------------------------------------------
                // Check for command inputs
                // Ryu
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
                    if boma.is_cat_flag( Cat4::SpecialNCommand) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
                    }
                    if boma.is_cat_flag( Cat4::SpecialN2Command) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
                    }
                }
                // Ken
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    if boma.is_cat_flag( Cat4::AttackCommand1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND, false);
                    }
                }
                //----------------------------------------------------------------------------------------------------------------------
            }
        }

        // Aerial Cancels
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
                || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {
                // Check for jump inputs
                if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) {
                    boma.check_jump_cancel(false);
                }
                // Check for special attack inputs
                if boma.is_cat_flag(Cat1::SpecialN) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                }
                if boma.is_cat_flag(Cat1::SpecialS) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                }
                if boma.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                }
                if boma.is_cat_flag(Cat1::SpecialLw) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                }

                //----------------------------------------------------------------------------------------------------------------------
                // Check for command inputs
                // Ryu
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
                    if boma.is_cat_flag( Cat4::SpecialNCommand) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
                    }
                    if boma.is_cat_flag( Cat4::SpecialN2Command) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
                    }
                }
                // Ken
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    if boma.is_cat_flag( Cat4::AttackCommand1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND, false);
                    }
                }
                //----------------------------------------------------------------------------------------------------------------------

            }
        }

    }
}

// Copy Abilities
// Fox Laser Land Cancel
unsafe fn fox_laser_fastfall_landcancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_FOX_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        } else if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Falco Laser Land Cancel
unsafe fn falco_laser_fastfall_landcancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_FALCO_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        } else if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Laser Airdodge Cancel
unsafe fn airdodge_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_WOLF_SPECIAL_N {
        if frame > 17.0 {
            boma.check_airdodge_cancel();
        }
    }
}

// Water Shuriken Max Dash Cancel
unsafe fn max_water_shuriken_dc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_GEKKOUGA_SPECIAL_N_MAX_SHOT {
        if frame > 12.0 {
            boma.check_dash_cancel();
        }
    }
}

// Firaga Airdodge Cancel
unsafe fn magic_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N1_SHOOT {
        if frame > 2.0 {
            boma.check_airdodge_cancel();
        }
    }
    // Thundaga Land Cancel
    if boma.is_status(*FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N3)
    && boma.is_situation(*SITUATION_KIND_GROUND)
    && boma.is_prev_situation(*SITUATION_KIND_AIR)
    {
        let special_n_fire_cancel_frame_ground = 69.0;
        let landing_lag = 12.0;
        if MotionModule::frame(boma) < (special_n_fire_cancel_frame_ground - landing_lag) {
            MotionModule::set_frame_sync_anim_cmd(boma, special_n_fire_cancel_frame_ground - landing_lag, true, true, true);
        }
    }
}


// Bite Early Throw and Turnaround
unsafe fn bite_early_throw_turnaround(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_WARIO_SPECIAL_N_BITE {
        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
            boma.change_status_req(*FIGHTER_KIRBY_STATUS_KIND_WARIO_SPECIAL_N_BITE_END, false);
        }
    }
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_WARIO_SPECIAL_N_BITE_END {
        if frame < 7.0 {
            if facing * stick_x < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

// Chef Land Cancel
unsafe fn ff_chef_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_GAMEWATCH_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
        if MotionModule::frame(boma) <= 1.0 {
            let nspec_halt = Vector3f{x: 0.9, y: 1.0, z: 1.0};
            KineticModule::mul_speed(boma, &nspec_halt, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_GAMEWATCH_SPECIAL_N {
        if frame <= 19.0 {
            MotionModule::set_rate(boma, 2.0);
        }
        if frame > 19.0 {
            MotionModule::set_rate(boma, 1.0);
        }
    }
}

// Nayru's Love Land Cancel and Fast Fall
unsafe fn nayru_fastfall_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32, frame: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_ZELDA_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && frame < 55.0 {
                //StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
                WorkModule::on_flag(boma, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END);
                MotionModule::set_frame_sync_anim_cmd(boma, 56.0, true, true, false);
            }
        }
        else if situation_kind == *SITUATION_KIND_AIR {
            if frame >= 31.0 {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                    WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
    }
}

// Hero Dash Cancel Frizz
unsafe fn dash_cancel_frizz(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_BRAVE_SPECIAL_N_SHOOT)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.is_motion(Hash40::new("brave_special_n1"))
    && fighter.motion_frame() > 20.0 && fighter.motion_frame() < 44.0 // after F20 and before the FAF
    && (WorkModule::get_float(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP) > 12.0)
    {
        if fighter.check_dash_cancel() {
            let mut brave_fighter = app::Fighter{battle_object: *(fighter.battle_object)};
            FighterSpecializer_Brave::add_sp(&mut brave_fighter, -10.0);
            EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, 15, -2, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

// Bullet Climax Mechanics
unsafe fn bayo_nspecial_mechanics(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_BAYONETTA_SPECIAL_N_CHARGE) { //PM-like neutral-b canceling
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.is_cat_flag(Cat1::AirEscape)  {
                ControlModule::reset_trigger(boma);
                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL, true);
                ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }//drift
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.04);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.005);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
        } else { //platdrop
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            if fighter.global_table[STICK_Y].get_f32() <= WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"))
            && fighter.global_table[FLICK_Y].get_i32() < WorkModule::get_param_int(boma, hash40("common"), hash40("pass_flick_y"))
            && GroundModule::is_passable_ground(boma) {
                GroundModule::pass_floor(fighter.module_accessor);
                ControlModule::clear_command;
            }
        }
    }
}

// Falcon Punch Turnarounds
unsafe fn repeated_falcon_punch_turnaround(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    let frame = fighter.motion_frame();
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN)
    && 22.0 < frame && frame < 41.0
    && fighter.is_stick_backward()
    && fighter.stick_x().abs() > 0.1
    {
        fighter.change_status_req(*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN, true);
    }
}

// Warlock Punch Turnarounds
unsafe fn repeated_warlock_punch_turnaround(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    let frame = fighter.motion_frame();
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_GANON_SPECIAL_N_TURN)
    && 30.0 < frame && frame < 45.0
    && fighter.is_stick_backward()
    && fighter.stick_x().abs() > 0.1
    {
        fighter.change_status_req(*FIGHTER_KIRBY_STATUS_KIND_GANON_SPECIAL_N_TURN, true);
    }
}


// Blue Eggs Land Cancel
unsafe fn blue_eggs_land_cancels(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.is_prev_situation(*SITUATION_KIND_AIR)
    {
        // Current FAF in motion list is 50, frame is 0 indexed so subtract a frame
        let special_n_fire_cancel_frame_ground = 49.0;
        // 11F of landing lag plus one extra frame to subtract from the FAF to actually get that amount of lag
        let landing_lag = 12.0;
        if MotionModule::frame(fighter.module_accessor) < (special_n_fire_cancel_frame_ground - landing_lag) {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, special_n_fire_cancel_frame_ground - landing_lag, true, true, false);
        }
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        //fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING, false);
    }
}

// Peanut Popgun Airdodge Cancel
unsafe fn peanut_popgun_ac(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_DIDDY_SPECIAL_N_SHOOT && frame > 5.0 {
        boma.check_airdodge_cancel();
    }
}

//Darkest Lariat Ledge Slipoff
unsafe fn lariat_ledge_slipoff(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_GAOGAEN_SPECIAL_N) {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_KEEP));
        fighter.sub_transition_group_check_air_cliff();
    }
}

//Bowser Flame Land Cancel
unsafe fn koopa_flame_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N {
        if frame < 23.0 {
            if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                MotionModule::set_frame(boma, 22.0, true);
            }
        }
    }
}

// Clown Cannon Shield Cancel
unsafe fn clown_cannon_shield_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_KOOPAJR_SPECIAL_N_HOLD {
        if frame > 16.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                }
            }
        }
    }
}

// Link's Bow Fastfall
unsafe fn bow_fastfall(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_LINK_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Bonus Fruit Airdodge Cancel
unsafe fn bonus_fruit_toss_ac(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_PACMAN_SPECIAL_N_SHOOT {
        if frame > 11.0 {
            boma.check_airdodge_cancel();
        }
    }
}

// Dark Pit's Bow Fastfall Land Cancel
unsafe fn bow_ff_lc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if(WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_PITB){
        if [*FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_SHOOT,
            *FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_CHARGE,
            *FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_DIR,
            *FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_TURN].contains(&status_kind) {
            if status_kind == *FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_SHOOT {
                if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
                }
            }
            if situation_kind == *SITUATION_KIND_AIR {
                if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66
                    && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                    WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
    }
}

// Flamethrower Land Cancel
unsafe fn plizardon_flame_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    let prev_situation = StatusModule::prev_situation_kind(boma);
    if status_kind != *FIGHTER_KIRBY_STATUS_KIND_PLIZARDON_SPECIAL_N || situation_kind != *SITUATION_KIND_GROUND || prev_situation != *SITUATION_KIND_AIR {
        return;
    }
    if StatusModule::is_changing(boma) {
        return;
    }
    if frame < 19.0 {
        MotionModule::set_frame(boma, 18.0, true);
    }
}

// Metal Blade Airdodge Cancel
unsafe fn blade_toss_ac(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_ROCKMAN_SPECIAL_N {
        if boma.status_frame() > 16 {
            boma.check_airdodge_cancel();
        }
    }
}

// Simon's Axe Fastfall
unsafe fn axe_ff(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SIMON_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            if boma.is_cat_flag(Cat2::FallJump)
                && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Toon Link's Bow Fastfall
unsafe fn heros_bow_ff(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_TOONLINK_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 {
                if KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                    WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
    }
}

// Young Link's Bow Fastfall
unsafe fn fire_arrow_ff(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_YOUNGLINK_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump)
                && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}


unsafe fn copy_ability_other_aerial_drift(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_KIRBY_STATUS_KIND_FOX_SPECIAL_N,
        *FIGHTER_KIRBY_STATUS_KIND_FALCO_SPECIAL_N,
        *FIGHTER_KIRBY_STATUS_KIND_WOLF_SPECIAL_N,
        *FIGHTER_KIRBY_STATUS_KIND_GAMEWATCH_SPECIAL_N]) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// PM-like neutral-b canceling
// Donkey Kong
unsafe fn donkey_nspecial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_DONKEY_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
}

// Samus & Dark Samus
unsafe fn samus_nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SAMUS_SPECIAL_N_C {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
}

// Robin
unsafe fn reflet_nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_REFLET_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

// Sheik
unsafe fn sheik_nspecial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SHEIK_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FT_SHEIK_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FT_SHEIK_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
            }
        }
    }
}

// Mewtwo
unsafe fn mewtwo_nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_MEWTWO_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            if MotionModule::is_end(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, false);
            }
        }
    }
}

// Squirtle
unsafe fn pzenigame_nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_PZENIGAME_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
        }
    }
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_PZENIGAME_SPECIAL_N_CHARGE {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat1::AirEscape) {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

// Diddy Kong
unsafe fn diddy_nspecial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_DIDDY_SPECIAL_N_CHARGE) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.is_cat_flag(Cat1::AirEscape)  {
                ControlModule::reset_trigger(boma);
                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL, true);
                ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

// Lucario
unsafe fn lucario_nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32) {
    /***if status_kind == *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD {
        if boma.is_cat_flag(Cat2::CommonGuard) {
            if situation_kind == *SITUATION_KIND_AIR {
                if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                    WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL, true);
                }
            }
            else {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL, true);
            }
        }
    }***/
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
        }
    }
}

// WiiFit Trainer
unsafe fn wiifit_nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_WIIFIT_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                //ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

// Little Mac
unsafe fn littlemac_nspecial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_START) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.is_cat_flag(Cat1::AirEscape)  {
                ControlModule::reset_trigger(boma);
                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL, true);
                ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

// Pac-Man
unsafe fn pacman_nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_PACMAN_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_NEXT_STATUS);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

// Hero
unsafe fn brave_nspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_BRAVE_SPECIAL_N_CANCEL)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    {
        WorkModule::set_int(fighter.module_accessor, *STATUS_KIND_NONE, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
}

// Sephiroth
unsafe fn edge_nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_EDGE_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

// Mii Gunner
unsafe fn miigunner_nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, cat2: i32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_MIIGUNNER_SPECIAL_N1_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            }
        }
    }
}

// Byleth
unsafe fn master_nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_MASTER_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                //ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    final_cutter_landing_bugfix(fighter);
    horizontal_cutter(fighter);
    //disable_dash_attack_slideoff(fighter);
    //stone_control(fighter);

    // Magic Series
    magic_series(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
    
    // Fox Laser Land Cancel and Fast Fall
    fox_laser_fastfall_landcancel(boma, status_kind, situation_kind, cat[1], stick_y);

    // Falco Laser Land Cancel and Fast Fall
    falco_laser_fastfall_landcancel(boma, status_kind, situation_kind, cat[1], stick_y);

    // Water Shuriken Max Dash Cancel
    max_water_shuriken_dc(boma, status_kind, situation_kind, cat[0], frame);

    // Firaga and Thundaga Cancels
    magic_cancels(boma, status_kind, situation_kind, cat[0], frame);

    // Bite Early Throw and Turnaround
    bite_early_throw_turnaround(boma, status_kind, stick_x, facing, frame);

    // Chef Land Cancel
    ff_chef_land_cancel(boma, status_kind, situation_kind, cat[1], stick_y);
    frame_data(boma, status_kind, motion_kind, frame);

    // Nayru's Love Land Cancel and Fast Fall
    nayru_fastfall_land_cancel(boma, status_kind, situation_kind, cat[2], stick_y, frame);

    // Hero Dash Cancel Frizz
    dash_cancel_frizz(fighter);

    // Laser Airdodge Cancel
    airdodge_cancel(boma, status_kind, situation_kind, cat[0], frame);

    // Bullet Arts Mechanics
    bayo_nspecial_mechanics(fighter, boma);

    // Falcon Punch Turnarounds
    repeated_falcon_punch_turnaround(fighter);

    // Warlock Punch Turnarounds
    repeated_warlock_punch_turnaround(fighter);

    //Blue Eggs Land Cancel
    blue_eggs_land_cancels(fighter);

    // Peanut Popgun Airdodge Cancel
    peanut_popgun_ac(boma, status_kind, situation_kind, cat[1], frame);

    // Darkest Lariat Ledge Slipoff
    lariat_ledge_slipoff(fighter);

    // Bowser Flame Land Cancel
    koopa_flame_cancel(boma, status_kind, situation_kind, frame);

    // Clown Cannon Shield Cancel
    clown_cannon_shield_cancel(boma, status_kind, situation_kind, frame);

    // Link's Bow Fastfall
    bow_fastfall(boma, status_kind, situation_kind, cat[1], stick_y);

    // Bonus Fruit Airdodge Cancel
    bonus_fruit_toss_ac(boma, status_kind, situation_kind, cat[0], frame);

    // Dark Pit's Bow Fastfall Land Cancel
    bow_ff_lc(boma, status_kind, situation_kind, cat[1], stick_y);

    // Flamethower Land Cancel
    plizardon_flame_cancel(boma, status_kind, situation_kind, frame);

    // Metal Blade Airdodge Cancel
    blade_toss_ac(boma, status_kind, situation_kind, cat[0], frame);

    // Simon's Axe Fastfall
    axe_ff(boma, status_kind, situation_kind, cat[1], stick_y);

    // Toon Link's Bow Fastfall
    heros_bow_ff(boma, status_kind, situation_kind, cat[1], stick_y);

    // Young Link's Bow Fastfall
    fire_arrow_ff(fighter, boma, status_kind, situation_kind, cat[1], stick_y);

    // Others Aerial Drift
    copy_ability_other_aerial_drift(fighter);

    // PM-like Neutral B Cancels
    donkey_nspecial_cancels(fighter, boma, status_kind, situation_kind);
    samus_nspecial_cancels(boma, status_kind, situation_kind);
    reflet_nspecial_cancels(boma, status_kind, situation_kind);
    sheik_nspecial_cancels(fighter, boma, status_kind, situation_kind);
    mewtwo_nspecial_cancels(boma, status_kind, situation_kind);
    pzenigame_nspecial_cancels(boma, status_kind, situation_kind, cat[1]);
    diddy_nspecial_cancels(fighter, boma);
    lucario_nspecial_cancels(boma, status_kind, situation_kind, cat[2]);
    wiifit_nspecial_cancels(boma, status_kind, situation_kind);
    littlemac_nspecial_cancels(fighter, boma);
    pacman_nspecial_cancels(boma, status_kind, situation_kind);
    brave_nspecial_cancels(fighter);
    edge_nspecial_cancels(boma, status_kind, situation_kind, cat[2]);
    miigunner_nspecial_cancels(boma, status_kind, situation_kind, cat[1], cat[2]);
    master_nspecial_cancels(boma, status_kind, situation_kind);
}

#[utils::macros::opff(FIGHTER_KIND_KIRBY )]
pub fn kirby_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		kirby_frame(fighter)
    }
}

pub unsafe fn kirby_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}