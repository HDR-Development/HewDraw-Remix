use super::*;

// Magic Series
unsafe fn magic_series(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    let cat1 = cat[0];
    let cat4 = cat[3];
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
                boma.check_jump_cancel(false, false);
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
                boma.check_jump_cancel(false, false);
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
                boma.check_jump_cancel(false, false);
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
                boma.check_jump_cancel(false, false);
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

// Copy Abilities
// Fox Drift and Laser Land Cancel
unsafe fn fox_drift_laser_landcancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_FOX_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Falco Drift and Laser Land Cancel
unsafe fn falco_drift_laser_landcancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_FALCO_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Wolf Drift and Laser Airdodge Cancel
unsafe fn wolf_drift_airdodge_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_WOLF_SPECIAL_N {
        if frame > 17.0 {
            boma.check_airdodge_cancel();
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
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

// Sora Magic Cancels
unsafe fn magic_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    // Firaga Airdodge Cancel
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

// cycles Kirby to firaga after copying Sora
unsafe fn trail_magic_cycle(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) { 
    if fighter.is_motion(Hash40::new("special_n_drink"))
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_TRAIL {
        let magic_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND);
        let kirby = fighter.global_table[0x4].get_ptr() as *mut Fighter;
        if magic_kind == *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_FIRE 
        && frame > 3.0 {
            WorkModule::on_flag(fighter.boma(), *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CHANGE_MAGIC);
            FighterSpecializer_Trail::change_magic(kirby); // cycles to thunder
        } else if magic_kind == *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_THUNDER
        && frame > 4.0 {
            FighterSpecializer_Trail::change_magic(kirby); // cycles to "blizzard", which is now fire
        }
    }
}

// handles kirby's mining behavior when copying steve
unsafe fn pickel_mining(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) { 
    if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_PICKEL {
        if VarModule::get_int(boma.object(), vars::kirby::instance::MATERIAL_INDEX) as i32 > 99 {
            VarModule::set_int(boma.object(), vars::kirby::instance::MATERIAL_INDEX, 0);
        }
        
        // wait 2 frames before letting the material table advance, preventing any jumps in entries
        if !VarModule::is_flag(boma.object(), vars::kirby::instance::SHOULD_CYCLE_MATERIAL) {
            if VarModule::get_int(boma.object(), vars::kirby::status::MINING_TIMER) == 0 {
                VarModule::on_flag(boma.object(), vars::kirby::instance::SHOULD_CYCLE_MATERIAL);
            } else {
                VarModule::dec_int(boma.object(), vars::kirby::status::MINING_TIMER);
            }
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

// Chef Drift and Land Cancel
unsafe fn chef_drift_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_GAMEWATCH_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if StatusModule::is_changing(boma) {
            let nspec_halt = Vector3f{x: 0.9, y: 1.0, z: 1.0};
            KineticModule::mul_speed(boma, &nspec_halt, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Nayru's Love Drift and Land Cancel
unsafe fn nayru_drift_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32, frame: f32) {
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
                if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
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
        let cooleddown = VarModule::countdown_int(boma.object(), vars::koopa::instance::FIREBALL_COOLDOWN_FRAME, 0);
        if frame < 23.0 && !cooleddown {
            if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                MotionModule::set_frame(boma, 22.0, true);
            }
        }
    }
}

unsafe fn koopa_fireball_cooldown(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    /* //Ignore cooldown during respawn,death,entry and nspecial
    if (&[
        *FIGHTER_STATUS_KIND_ENTRY,*FIGHTER_STATUS_KIND_DEAD,*FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,*FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_SPECIAL_N
    ]).contains(&status_kind) {
        return;
    } */

    if (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_KOOPA) {
        let cooleddown = VarModule::countdown_int(boma.object(), vars::koopa::instance::FIREBALL_COOLDOWN_FRAME, 0);
        let charged_effect =  VarModule::get_int(boma.object(), vars::koopa::instance::FIREBALL_EFFECT_ID);
        //If cooling down, remove ready effect
        if !cooleddown {
            if charged_effect > 0 {
                VarModule::set_int(boma.object(), vars::koopa::instance::FIREBALL_EFFECT_ID,0);
                if EffectModule::is_exist_effect(boma, charged_effect as u32) {
                    EffectModule::kill(boma, charged_effect as u32, false,false);
                }
            }
            return;
        }
        //Otherwise, spawn effect if effect does not exist
        else if (charged_effect <= 0
        || !EffectModule::is_exist_effect(boma, charged_effect as u32))
        {
            if (charged_effect <= 0){
                gimmick_flash(boma);
            }
            let pos = &Vector3f{x: 0.0, y: 5.0, z: 0.0};
            let rot = &Vector3f{x: 180.0, y: 0.0, z: 50.0};
            let handle = EffectModule::req_follow(boma, Hash40::new("koopa_breath_m_fire"), Hash40::new("body"), pos, rot, 1.0, true, 0, 0, 0, 0, 0, false, false) as u32;
            VarModule::set_int(boma.object(), vars::koopa::instance::FIREBALL_EFFECT_ID,handle as i32);
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

// Link's Bow Drift
unsafe fn bow_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_LINK_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
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

// Colorless Attack Dash Cancel on Hit
// This is unique to Kirby due to only having access to colorless attack.
unsafe fn colorless_attack_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_PALUTENA_SPECIAL_N {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) && frame > 19.0 {
            boma.check_dash_cancel();
        }
    }
}

// Dark Pit's Bow Land Cancel
unsafe fn pitb_bow_lc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
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

// Simon's Axe Drift
unsafe fn axe_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SIMON_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Richter's Knife Drift
unsafe fn knife_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_RICHTER_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Richter's Knife land cancel
unsafe fn knife_lc(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_KIRBY_STATUS_KIND_RICHTER_SPECIAL_N)
    && VarModule::is_flag(boma.object(), vars::richter::instance::SPECIAL_N_LAND_CANCEL)
    && boma.is_situation(*SITUATION_KIND_GROUND) {
        // remove the unthrown knife from richter's hand
        if (2.0..13.0).contains(&boma.motion_frame())
        && ArticleModule::is_exist(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE){
            ArticleModule::remove_exist(boma, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }

        let landing_lag = 10.0; // amount of frames until richter can act when landing
        let rate = 27.0 / landing_lag;
        MotionModule::change_motion(boma, Hash40::new("landing_heavy"), 0.0, rate, false, 0.0, false, false);
        VarModule::off_flag(boma.object(), vars::richter::instance::SPECIAL_N_LAND_CANCEL);
        EffectModule::kill_kind(boma, Hash40::new("sys_sp_flash"), true, true);
    }
}

// Toon Link's Bow Drift
unsafe fn heros_bow_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_TOONLINK_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Young Link's Bow Drift
unsafe fn fire_arrow_drift(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_YOUNGLINK_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
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
unsafe fn diddy_nspecial_cancels(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_DIDDY_SPECIAL_N_CHARGE {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if fighter.is_cat_flag(Cat2::StickEscape) {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_ESCAPE);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL.into(), true.into());
            }
            else if fighter.is_cat_flag(Cat2::StickEscapeF) {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_ESCAPE_F);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL.into(), true.into());
            }
            else if fighter.is_cat_flag(Cat2::StickEscapeB) {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_ESCAPE_B);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL.into(), true.into());
            }
            else if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump) && fighter.sub_check_button_frick().get_bool())) {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_GROUND_JUMP);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL.into(), true.into());
            }
            if fighter.sub_check_command_guard().get_bool() {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_GUARD);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL.into(), true.into());
            }
        }
        else {
            if fighter.is_cat_flag(Cat1::AirEscape)  {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_ESCAPE_AIR);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL.into(), true.into());
            }
            else if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump)))
            && fighter.get_num_used_jumps() < fighter.get_jump_count_max()
            {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_JUMP_AERIAL);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL_JUMP.into(), true.into());
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

// Ken
unsafe fn ken_air_hado_distinguish(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if !boma.is_status_one_of(&[
        *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N,
        *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND,
        *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND,
        *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N,
        *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND,
        *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N2_COMMAND,
    ]) {
        return;
    }

    // set VarModule flag on f12 - this flag changes hado properties
    if frame == 12.0 && fighter.is_motion_one_of(&[
        Hash40::new("ken_special_air_n"),
    ]) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_CURRENT_HADOKEN_AIR);
    }
    // after frame 13, disallow changing from aerial to grounded hadoken
    // instead, we enter a landing animation
    if (frame > 13.0 || fighter.is_motion_one_of(&[
        Hash40::new("ken_special_air_n_empty"),
        Hash40::new("ken_special_n_empty"),
    ]))
    && boma.is_situation(*SITUATION_KIND_GROUND)
    && boma.is_prev_situation(*SITUATION_KIND_AIR) {
        if frame < 70.0 { // the autocancel frame
            WorkModule::set_float(boma, 11.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            boma.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        } else {
            boma.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
        }
    }
}

//Bowser & Lucas
unsafe fn reset_flags(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) != FIGHTER_KIND_KOOPA {
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,KOOPA_MAX_COOLDOWN);
    }
    if ( WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) != FIGHTER_KIND_LUCAS || [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind)  || !sv_information::is_ready_go() ) {
        //let charge_time = ParamModule::get_int(fighter.object(), ParamType::Agent, "attack_up_charge_time");
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, LUCAS_CHARGE_TIME);
        VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
        VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
        VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
    }
}

unsafe fn lucas_offense_effct_handler(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) && !VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT) 
    && (VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1) == -1 || VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2) == -1) {
        // The case is that Lucas is in Offense Up, has cleared past `pkfr_hold` effects, yet he does not have his hand effects. //
        let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("lucas_pkfr_hold"), Hash40::new("handl"), &Vector3f{x: -2.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1, handle as i32);
        let handle2 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("lucas_pkfr_hold"), Hash40::new("handr"), &Vector3f{x: -2.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2, handle2 as i32);
        let handle3 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_status_defense_up"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3, handle3 as i32);
    }
    else if !VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) 
    && (VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1) != -1 || VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2) != -1) {
        // The case is that Lucas is no longer in Offence Up, and his hand effects NEED TO BE CLEARED. //
        let handle = VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1) as u32;
        EffectModule::kill(fighter.module_accessor, handle, false, false);
        let handle2 = VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2) as u32;
        EffectModule::kill(fighter.module_accessor, handle2, false, false);
        let handle3 = VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3) as u32;
        EffectModule::kill(fighter.module_accessor, handle3, false, false);
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1, -1);
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2, -1);
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3, -1);
    }
}

unsafe fn lucas_offense_charge(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, situation_kind: i32)  {
    if(VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE)) {
        if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, 
            *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_FIRE]
        ) {
            //println!("In swing! Status of release: {} Reflective: {}", VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF));
            if(AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            }
        }
        else if !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4, 
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_START, 
            *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_END]) && VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF
        ) {
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
        }
    } 
}

// Piranha Plant Ptooie Stance
unsafe fn packun_ptooie_stance(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SWALLOW_WAIT) {
        let opponent_boma = fighter.get_grabbed_opponent_boma();
        let grabbed_fighter = smash::app::utility::get_kind(opponent_boma);
        if grabbed_fighter == *FIGHTER_KIND_PACKUN {
            let old_stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
            let new_stance = VarModule::get_int(opponent_boma.object(), vars::packun::instance::CURRENT_STANCE);
            if new_stance != old_stance {
                // println!("Copying Pirahna Plant's Current Stance, which is {}", new_stance);
                VarModule::set_int(boma.object(), vars::packun::instance::CURRENT_STANCE, new_stance);
            }
        }
    }
}

unsafe fn packun_ptooie_scale(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        VarModule::set_float(boma.object(), vars::packun::instance::PTOOIE_SCALE, 1.3);
    }
    else {
        VarModule::set_float(boma.object(), vars::packun::instance::PTOOIE_SCALE, 1.0);
    }
}

pub unsafe fn kirby_copy_handler(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    if !WorkModule::is_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY) {
        reset_flags(fighter, boma, status_kind, situation_kind);
        return;
    }

    let copy = WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);

    match copy {
        // Ryu
        0x3C => magic_series(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame),
        // Ken
        0x3D => {
            magic_series(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
            ken_air_hado_distinguish(fighter, boma, frame);
        },
        // Lucario
        0x2C => magic_series(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame),
        // Terry
        0x55 => magic_series(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame),
        // Fox
        0x7 => fox_drift_laser_landcancel(boma, status_kind, situation_kind, cat[1], stick_y),
        // Falco
        0x14 => falco_drift_laser_landcancel(boma, status_kind, situation_kind, cat[1], stick_y),
        // Wolf
        0x2F => wolf_drift_airdodge_cancel(boma, status_kind, situation_kind, cat[1], frame),
        // Greninja
        0x35 => max_water_shuriken_dc(boma, status_kind, situation_kind, cat[0], frame),
        // Sora
        0x5D => {
            magic_cancels(boma, status_kind, situation_kind, cat[0], frame);
            trail_magic_cycle(fighter, boma, frame);
        },
        // Steve
        0x58 => pickel_mining(fighter, boma),
        // Wario
        0x21 => bite_early_throw_turnaround(boma, status_kind, stick_x, facing, frame),
        // Mr. Game & Watch
        0x1C => chef_drift_land_cancel(boma, status_kind, situation_kind, cat[1], stick_y),
        // Zelda
        0x11 => nayru_drift_land_cancel(boma, status_kind, situation_kind, cat[2], stick_y, frame),
        // Hero
        0x53 => {
            dash_cancel_frizz(fighter);
            brave_nspecial_cancels(fighter);
        },
        // Captain Falcon
        0xB => repeated_falcon_punch_turnaround(fighter),
        // Banjo & Kazooie
        0x54 => blue_eggs_land_cancels(fighter),
        // Diddy Kong
        0x27 => {
            peanut_popgun_ac(boma, status_kind, situation_kind, cat[1], frame);
            diddy_nspecial_cancels(fighter, boma, status_kind);
        },
        // Incineroar
        0x47 => lariat_ledge_slipoff(fighter),
        // Bowser
        0xF => {
            koopa_flame_cancel(boma, status_kind, situation_kind, frame);
            koopa_fireball_cooldown(boma, status_kind);
        },
        // Bowser Jr.
        0x3A => clown_cannon_shield_cancel(boma, status_kind, situation_kind, frame),
        // Link
        0x2 => bow_drift(boma, status_kind, situation_kind, cat[1], stick_y),
        // Pac Man
        0x37 => {
            bonus_fruit_toss_ac(boma, status_kind, situation_kind, cat[0], frame);
            pacman_nspecial_cancels(boma, status_kind, situation_kind);
        },
        // Palutena
        0x36 => colorless_attack_dash_cancel(boma, status_kind, situation_kind, cat[0], frame),
        // Dark Pit
        0x1F => pitb_bow_lc(boma, status_kind, situation_kind, cat[1], stick_y),
        // Charizard
        0x26 => plizardon_flame_cancel(boma, status_kind, situation_kind, frame),
        // Mega Man
        0x31 => blade_toss_ac(boma, status_kind, situation_kind, cat[0], frame),
        // Simon
        0x43 => axe_drift(boma, status_kind, situation_kind, cat[1], stick_y),
        // Richter
        0x44 => {
            knife_drift(boma, status_kind, situation_kind, cat[1], stick_y);
            knife_lc(boma);
        },
        // Toon Link
        0x2E => heros_bow_drift(boma, status_kind, situation_kind, cat[1], stick_y),
        // Young Link
        0x17 => fire_arrow_drift(fighter, boma, status_kind, situation_kind, cat[1], stick_y),
        // Lucas
        0x28 => {
            lucas_offense_charge(fighter, boma, situation_kind);
            lucas_offense_effct_handler(fighter);
        },
        // Piranha Plant
        0x51 => {
            packun_ptooie_stance(fighter, boma, status_kind);
            packun_ptooie_scale(boma);
        },
        // Donkey Kong
        0x1 => donkey_nspecial_cancels(fighter, boma, status_kind, situation_kind),
        // Samus
        0x3 => samus_nspecial_cancels(fighter, status_kind, situation_kind),
        // Robin
        0x38 => reflet_nspecial_cancels(fighter, status_kind, situation_kind),
        // Sheik
        0x10 => sheik_nspecial_cancels(fighter, boma, status_kind, situation_kind),
        // Mewtwo
        0x19 => mewtwo_nspecial_cancels(boma, status_kind, situation_kind),
        // Squirtle
        0x24 => pzenigame_nspecial_cancels(boma, status_kind, situation_kind, cat[1]),
        // Lucario
        0x2C => lucario_nspecial_cancels(fighter, status_kind, situation_kind, cat[2]),
        // Wii Fit Trainer
        0x32 => wiifit_nspecial_cancels(fighter, status_kind, situation_kind),
        // Sephiroth
        0x59 => edge_nspecial_cancels(boma, status_kind, situation_kind, cat[2]),
        // Mii Gunner
        0x4A => miigunner_nspecial_cancels(boma, status_kind, situation_kind, cat[1], cat[2]),
        // Byleth
        0x56 => master_nspecial_cancels(boma, status_kind, situation_kind),
        _ => {}
    }
}