// opff import
utils::import_noreturn!(common::opff::{fighter_common_opff});
use super::*;
use globals::*;

 
unsafe fn dtilt_repeat_increment(boma: &mut BattleObjectModuleAccessor, id: usize, motion_kind: u64) {
    if motion_kind == hash40("attack_lw3")
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        &&  !VarModule::is_flag(boma.object(), vars::shotos::status::REPEAT_INCREMENTED) {
        //VarModule::inc_int(boma.object(), vars::common::REPEAT_NUM_LW);
        VarModule::on_flag(boma.object(), vars::shotos::status::REPEAT_INCREMENTED);
    }
}

// Terry Power Wave Dash Cancel and Super Cancels
unsafe fn power_wave_dash_cancel_super_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat: [i32; 4], motion_kind: u64, frame: f32) {
    let mut agent_base = fighter.fighter_base.agent_base;
    let cat1 = cat[0];
    let cat4 = cat[3];
    let prev_situation_kind = StatusModule::prev_situation_kind(boma);

    if StatusModule::is_changing(boma) {
        return;
    }

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        // Super Cancel
        if frame > 22.0 {
            // Check to see if supers are available
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                // Enable transition term
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);

                // Buster Wolf
                if boma.is_cat_flag(Cat4::SuperSpecialCommand){
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, false);
                }

                // Power Geyser
                if boma.is_cat_flag(Cat4::SuperSpecial2Command){
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, false);
                }
            }
        }

        // Triple Geyser
        if boma.is_cat_flag( Cat4::SpecialN2Command) {
            if MeterModule::drain(boma.object(), 6) {
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
            }
        }

        // Dash Cancel
        if frame > 34.0 {
            boma.check_dash_cancel();
        }
    }
}

// Special and Super Cancels into Triple Geyser
unsafe fn special_super_cancels_triple_geyser(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, cat4: i32, motion_kind: u64) {
    let mut agent_base = fighter.fighter_base.agent_base;
    if [*FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK].contains(&status_kind) {
        // Triple Geyser
        if boma.is_cat_flag( Cat4::SpecialN2Command) {
            if MeterModule::drain(boma.object(), 6) {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
            }
        }
    }

    // Super Cancels into Triple Geyser
    if [*FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW].contains(&status_kind)
        && motion_kind == 0x13434c5490 as u64 {
        if boma.is_cat_flag( Cat4::SpecialN2Command) {
            if MeterModule::drain(boma.object(), 4) {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
            }
        }
    }
}

// Terry Burn Knuckle Land Cancel
// Check for aerial startup
unsafe fn burn_knuckle_land_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    if motion_kind == hash40("special_air_f_start") {
        if situation_kind == *SITUATION_KIND_AIR {
            VarModule::on_flag(boma.object(), vars::dolly::status::AIR_SPECIAL_F);
        }
    }
    if VarModule::is_flag(boma.object(), vars::dolly::status::AIR_SPECIAL_F) {
        if [*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END,
            *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
            }
        }
    }
}

// Power Dunk break
unsafe fn power_dunk_break(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK]) {
        if VarModule::is_flag(boma.object(), vars::shotos::instance::IS_TARGET_COMBO_1) {
            //KineticModule::mul_speed(boma, &Vector3f::new(1.0, 0.0, 0.0), *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            //KineticModule::mul_speed(boma, &Vector3f::new(1.0, 0.0, 0.0), *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
}

// Terrry Super Special Meter Activation
// unsafe fn super_special_meter_activation(boma: &mut BattleObjectModuleAccessor) {
//     if MeterModule::level(boma.object()) >= 4 {
//         WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
//     }
//     else {
//         WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
//     }
// }

// Cancel supers early
unsafe fn cancel_supers_early(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if [*FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2].contains(&status_kind) {
        if frame < 26.0 {
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, false);
                }
            }
        }
    }
}

// Super Cancels
unsafe fn super_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, cat4: i32, motion_kind: u64) {
    let mut agent_base = fighter.fighter_base.agent_base;
    // Power Geyser
    if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL {
            
        // Power Geyser -> Buster Wolf
        if boma.is_cat_flag(Cat4::SuperSpecial2Command) {
            if !StopModule::is_stop(boma){
                if MeterModule::drain(boma.object(), 2) {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
                    VarModule::on_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, false);
                }
            }
        }
    }
    // Buster Wolf
    else if [*FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW].contains(&status_kind)
        || motion_kind == 0x13434c5490 as u64 {
        // Buster Wolf -> Power Geyser
        if boma.is_cat_flag(Cat4::SuperSpecialCommand){
            if !StopModule::is_stop(boma){
                if MeterModule::drain(boma.object(), 2) {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
                    VarModule::on_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, false);
                }
            }
        }
    }
    else{
        VarModule::off_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL);
    }
}

// TRAINING MODE
// Full Meter Gain via shield during taunt
unsafe fn training_mode_meter(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let mut agent_base = fighter.fighter_base.agent_base;
    if is_training_mode() {
        MeterModule::set_meter_cap(boma.object(), 10);
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                let meter_max = (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object()));
                MeterModule::add(boma.object(), meter_max);
            }
        }
    }
}

unsafe fn meter_cap_control(boma: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let info = app::lua_bind::FighterManager::get_fighter_information(crate::singletons::FighterManager(), app::FighterEntryID(entry_id));
    if lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager()) {
        MeterModule::reset(boma.object());
        // println!("is result mode");
    }
    
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,]) {
        MeterModule::reset(boma.object());
        // println!("is victory");
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ENTRY,]) || !sv_information::is_ready_go() {
        VarModule::on_flag(boma.object(), vars::dolly::instance::IS_INIT_METER);
        // println!("is entry, add 1 bar");
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) && !is_training_mode() {
        VarModule::on_flag(boma.object(), vars::dolly::instance::INCREASE_METER_STOCKS);
        // println!("is death, add 1 bar and extend cap");
    }

    if VarModule::is_flag(boma.object(), vars::dolly::instance::IS_INIT_METER) {
        MeterModule::reset(boma.object());
        VarModule::set_int(boma.object(), vars::dolly::instance::METER_STOCKS, 0);
        VarModule::set_int(boma.object(), vars::dolly::instance::CURRENT_STOCKS, app::lua_bind::FighterInformation::stock_count(info) as i32);
        MeterModule::set_meter_cap(boma.object(), 4);
        MeterModule::add(boma.object(), MeterModule::meter_per_level(boma.object()) * 2.0);
        VarModule::off_flag(boma.object(), vars::dolly::instance::IS_INIT_METER);
    }

    if VarModule::is_flag(boma.object(), vars::dolly::instance::INCREASE_METER_STOCKS) {
        if VarModule::get_int(boma.object(), vars::dolly::instance::CURRENT_STOCKS) > app::lua_bind::FighterInformation::stock_count(info) as i32 {
            VarModule::set_int(boma.object(), vars::dolly::instance::CURRENT_STOCKS, app::lua_bind::FighterInformation::stock_count(info) as i32);
            if VarModule::get_int(boma.object(), vars::dolly::instance::METER_STOCKS) < 3 {
                VarModule::set_int(boma.object(), vars::dolly::instance::METER_STOCKS, VarModule::get_int(boma.object(), vars::dolly::instance::METER_STOCKS) + 1);
            }
            MeterModule::set_meter_cap(boma.object(), 4 + (VarModule::get_int(boma.object(), vars::dolly::instance::METER_STOCKS) * 2));
            MeterModule::add(boma.object(), MeterModule::meter_per_level(boma.object()) * 2.0);
        }
        VarModule::off_flag(boma.object(), vars::dolly::instance::INCREASE_METER_STOCKS);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_FALL,
        ]) 
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
    //dtilt_repeat_increment(boma, id, motion_kind); // UNUSED
    power_wave_dash_cancel_super_cancels(fighter, boma, id, status_kind, situation_kind, cat, motion_kind, frame);
    special_super_cancels_triple_geyser(fighter, boma, id, status_kind, cat[3], motion_kind);
    //burn_knuckle_land_cancel(boma, id, status_kind, situation_kind, motion_kind); // UNUSED
    // super_special_meter_activation(boma);
    cancel_supers_early(boma, status_kind, situation_kind, frame);
    super_cancels(fighter, boma, id, status_kind, cat[3], motion_kind);
    training_mode_meter(fighter, boma, status_kind);
    power_dunk_break(boma);
    special_cancels(boma);
    ex_special_scripting(boma);
    meter_cap_control(boma);
    fastfall_specials(fighter);

    // Magic Series
    magic_series(fighter, boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

unsafe fn ex_special_scripting(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::is_flag(boma.object(), vars::shotos::instance::IS_USE_EX_SPECIAL){
        if boma.is_motion(Hash40::new("special_b_attack_w")){
            MotionModule::change_motion(boma, Hash40::new("special_b_attack"), -1.0, 1.0, false, 0.0, false, false);
        }
    }
    // Fix geting stuck in fsmash after shatter strike due to not setting the smash charge flag
    if VarModule::is_flag(boma.object(), vars::dolly::status::IS_SHATTER_STRIKE){
        if boma.is_motion(Hash40::new("attack_s4_s")) && (MotionModule::frame(boma) >= (MotionModule::end_frame(boma) - 1.0)){
            boma.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
        }
    }
}

unsafe fn special_cancels(boma: &mut BattleObjectModuleAccessor) {
    let mut new_status = 0;
    let mut is_input_cancel = false;
    let mut is_input_special_special_cancel = false;
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S,
                               *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,
                               *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK,
                               *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,
                               *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,
                               *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK,
                               *FIGHTER_STATUS_KIND_SPECIAL_HI,
                               *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,
                               *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP,
                               *FIGHTER_STATUS_KIND_SPECIAL_LW,
                               *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,
                               *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK]){
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
            VarModule::on_flag(boma.object(), vars::shotos::instance::IS_ENABLE_FADC);
        }

        // If we detected that you've connected with a hitbox in any of the above statuses
        if VarModule::is_flag(boma.object(), vars::shotos::instance::IS_ENABLE_FADC){
            // Super cancels
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
                if boma.is_cat_flag(Cat4::SuperSpecialCommand) {
                    is_input_cancel = true;
                    new_status = *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL;
                }
                else if boma.is_cat_flag(Cat4::SuperSpecial2Command){
                    is_input_cancel = true;
                    new_status = *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2;
                }
            }

            // Special -> Special Cancels
            // Burn Knuckle cancels
            if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S,
                                       *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,
                                       *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK]){

                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);

                // Each cancel costs 2 meter
                // Crack Shoot
                if boma.is_cat_flag(Cat1::SpecialS) && boma.is_stick_backward() {
                    is_input_special_special_cancel = true;
                    new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B;
                }
                if boma.is_cat_flag( Cat4::SpecialSCommand) {
                    is_input_special_special_cancel = true;
                    new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND;
                }
                // Rising Tackle
                if boma.is_cat_flag(Cat1::SpecialHi) {
                    is_input_special_special_cancel = true;
                    new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
                }
                if boma.is_cat_flag( Cat4::SpecialHi2Command) {
                    is_input_special_special_cancel = true;
                    new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND;
                }
                // Power Dunk
                if boma.is_cat_flag(Cat1::SpecialLw) {
                    is_input_special_special_cancel = true;
                    new_status = *FIGHTER_STATUS_KIND_SPECIAL_LW;
                }
                if boma.is_cat_flag( Cat4::SpecialHiCommand) {
                    is_input_special_special_cancel = true;
                    new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND;
                }
            }

            // Rising Tackle cancels
            if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI,
                                       *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,
                                       *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP]){

                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);

                // Each cancel costs 2 meter
                // Power Wave
                if boma.is_cat_flag(Cat1::SpecialN) {
                    is_input_special_special_cancel = true;
                    new_status = *FIGHTER_STATUS_KIND_SPECIAL_N;
                }
                // Burn Knuckle
                if boma.is_cat_flag(Cat1::SpecialS) && boma.is_stick_forward() {
                    is_input_special_special_cancel = true;
                    new_status = *FIGHTER_STATUS_KIND_SPECIAL_S;
                }
                if boma.is_cat_flag( Cat4::SpecialNCommand) {
                    is_input_special_special_cancel = true;
                    new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND;
                }
                // Crack Shoot
                if boma.is_cat_flag(Cat1::SpecialS) && boma.is_stick_backward() {
                    is_input_special_special_cancel = true;
                    new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B;
                }
                if boma.is_cat_flag( Cat4::SpecialSCommand) {
                    is_input_special_special_cancel = true;
                    new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND;
                }
                // Power Dunk
                if boma.is_cat_flag(Cat1::SpecialLw) {
                    is_input_special_special_cancel = true;
                    new_status = *FIGHTER_STATUS_KIND_SPECIAL_LW;
                }
                if boma.is_cat_flag( Cat4::SpecialHiCommand) {
                    is_input_cancel = true;
                    new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND;
                }
            }
        }

    }
    else{
        VarModule::off_flag(boma.object(), vars::shotos::instance::IS_ENABLE_FADC);
        return;
    }
    if is_input_cancel{
        if !StopModule::is_stop(boma){
            boma.change_status_req(new_status, false);
        }
    }
    if is_input_special_special_cancel{
        if !StopModule::is_stop(boma){
            if MeterModule::drain(boma.object(), 1) {
                boma.change_status_req(new_status, false);
            }
        }
    }
    
}

unsafe fn jab_cancels(boma: &mut BattleObjectModuleAccessor) {
    let mut new_status = 0;
    let mut is_input_cancel = false;
    // Jab 1 cancels
    if boma.is_motion(Hash40::new("attack_11")) {
        if boma.is_cat_flag(Cat1::AttackHi3) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_ATTACK_HI3;
        } else if boma.is_cat_flag(Cat1::AttackLw3) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_ATTACK_LW3;
        }
        // Tilt cat flags override smash cat flags, need to check smashes separately after tilts so the smash input can be properly detecetd
        if boma.is_cat_flag(Cat1::AttackS4) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_ATTACK_S4_START;
        } else if boma.is_cat_flag(Cat1::AttackHi4) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_ATTACK_HI4_START;
        }
        // Power Charge
        if boma.is_cat_flag(Cat4::SpecialN2Command) {
            is_input_cancel = true;
            ControlModule::clear_command(boma, false);
            WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
            new_status = *FIGHTER_STATUS_KIND_ATTACK_DASH;
        }
    }
    // Jab 2 cancels
    else if boma.is_motion(Hash40::new("attack_12")) {
        if boma.is_cat_flag(Cat1::AttackHi3) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_ATTACK_HI3;
        } else if boma.is_cat_flag(Cat1::AttackS4) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_ATTACK_S4_START;
        } else if boma.is_cat_flag(Cat1::AttackHi4) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_ATTACK_HI4_START;
        }
        // Power Charge
        if boma.is_cat_flag(Cat4::SpecialN2Command) {
            is_input_cancel = true;
            ControlModule::clear_command(boma, false);
            WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
            new_status = *FIGHTER_STATUS_KIND_ATTACK_DASH;
        }
    }
    else{
        return;
    }

    if is_input_cancel{
        if (!StopModule::is_stop(boma) )|| (new_status == *FIGHTER_STATUS_KIND_ATTACK_DASH) {
            VarModule::on_flag(boma.object(), vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
            boma.change_status_req(new_status, false);
        }
    }
}

unsafe fn tilt_cancels(boma: &mut BattleObjectModuleAccessor) {
    let mut new_status = 0;
    let mut is_input_cancel = false;
    let mut is_input_metered_cancel = false;
    if boma.is_motion(Hash40::new("attack_s3_s")){
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD){
            // Dash
            if boma.is_cat_flag(Cat4::Command6N6) {
                is_input_metered_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_DASH;
            }
            // Jump
            if boma.is_input_jump() {
                is_input_metered_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
            }
        }
        // Power Charge
        if boma.is_cat_flag(Cat4::SpecialN2Command) {
            is_input_cancel = true;
            ControlModule::clear_command(boma, false);
            WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
            new_status = *FIGHTER_STATUS_KIND_ATTACK_DASH;
        }
    }
    else if boma.is_motion(Hash40::new("attack_hi3")){
        if boma.is_cat_flag(Cat1::AttackS4) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_ATTACK_S4_START;
        }
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD){
            // Dash
            if boma.is_cat_flag(Cat4::Command6N6) {
                is_input_metered_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_DASH;
            }
            // Jump
            if boma.is_input_jump() {
                is_input_metered_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
            }
        }
        // Power Charge
        if boma.is_cat_flag(Cat4::SpecialN2Command) {
            is_input_cancel = true;
            ControlModule::clear_command(boma, false);
            WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
            new_status = *FIGHTER_STATUS_KIND_ATTACK_DASH;
        }
    }
    else if boma.is_motion(Hash40::new("attack_lw3")) {
        if boma.is_cat_flag(Cat1::AttackHi3) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_ATTACK_HI3;
        }
        else if boma.is_cat_flag(Cat1::AttackS4) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_ATTACK_S4_START;
        }
        else if boma.is_cat_flag(Cat1::AttackHi4) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_ATTACK_HI4_START;
        }
        // Power Charge
        if boma.is_cat_flag(Cat4::SpecialN2Command) {
            is_input_cancel = true;
            ControlModule::clear_command(boma, false);
            WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
            new_status = *FIGHTER_STATUS_KIND_ATTACK_DASH;
        }
    }
    else{
        return;
    }
    if is_input_cancel{
        if (!StopModule::is_stop(boma) )|| (new_status == *FIGHTER_STATUS_KIND_ATTACK_DASH) {
            VarModule::on_flag(boma.object(), vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
            boma.change_status_req(new_status, false);
        }
    }
    if is_input_metered_cancel{
        if !StopModule::is_stop(boma) && !VarModule::is_flag(boma.object(), vars::dolly::status::UNABLE_CANCEL_S3_DASH){
            if MeterModule::drain(boma.object(), 1) {
                if new_status == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                    boma.change_status_req(new_status, true);
                }
                else{
                    boma.change_status_req(new_status, false);
                }
            }
        }
    }
}

unsafe fn dash_attack_cancels(boma: &mut BattleObjectModuleAccessor) {

    let mut new_status = 0;
    let mut is_input_cancel = false;
    let mut is_input_metered_cancel = false;
    if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
        if boma.is_cat_flag(Cat4::SuperSpecialCommand) {
            is_input_cancel = true;
            new_status = *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL;
        }
        else if boma.is_cat_flag(Cat4::SuperSpecial2Command) {
            is_input_cancel = true;
            new_status = *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2;
        }
    }
    if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)
    && !VarModule::is_flag(boma.object(), vars::shotos::instance::IS_USE_EX_SPECIAL)
    && !VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK){
        // Rising Tackle
            if boma.is_cat_flag(Cat1::SpecialHi) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI
        }
        else if boma.is_cat_flag(Cat4::SpecialHi2Command) {
            is_input_cancel = true;
            new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND;
        }
        // Power Dunk
        else if boma.is_cat_flag(Cat4::SpecialHiCommand) {
            is_input_cancel = true;
            new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND;
        }
        else if boma.is_cat_flag(Cat1::SpecialLw) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_LW
        }
    }

    if is_input_cancel{
        if !StopModule::is_stop(boma){
            VarModule::on_flag(boma.object(), vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
            boma.change_status_req(new_status, false);
        }
    }
}

unsafe fn smash_cancels(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);

    let mut new_status = 0; 
    let mut is_input_cancel = false;   

    if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD){
        // Power Wave
        if boma.is_cat_flag(Cat1::SpecialN) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_N;
        }
        // Burn Knuckle
        else if boma.is_cat_flag(Cat4::SpecialNCommand) {
            is_input_cancel = true;
            new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND;
        }
        else if boma.is_cat_flag(Cat1::SpecialS) && boma.is_stick_forward() {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_S;
        }
        // Crack Shoot
        else if boma.is_cat_flag(Cat4::SpecialSCommand) {
            is_input_cancel = true;   
            new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND;
        }
        else if boma.is_cat_flag(Cat1::SpecialS) && boma.is_stick_backward() {
            is_input_cancel = true;
            new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B;
        }
        // Rising Tackle
        else if boma.is_cat_flag(Cat1::SpecialHi) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI
        }
        else if boma.is_cat_flag(Cat4::SpecialHi2Command) {
            is_input_cancel = true;
            new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND;
        }
        // Power Dunk
        else if boma.is_cat_flag(Cat4::SpecialHiCommand) {
            is_input_cancel = true;
            new_status = *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND;
        }
        else if boma.is_cat_flag(Cat1::SpecialLw) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_LW
        }
        // Power Charge
        if boma.is_cat_flag(Cat4::SpecialN2Command) {
            is_input_cancel = true;
            ControlModule::clear_command(boma, false);
            WorkModule::off_flag(boma, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
            new_status = *FIGHTER_STATUS_KIND_ATTACK_DASH;
        }
    }
    else{
        return;
    }
    if is_input_cancel{
        if (!StopModule::is_stop(boma) )|| (new_status == *FIGHTER_STATUS_KIND_ATTACK_DASH) {
            VarModule::on_flag(boma.object(), vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
            boma.change_status_req(new_status, false);
        }
    }
    
}

unsafe fn aerial_cancels(boma: &mut BattleObjectModuleAccessor) {
    let dir = boma.get_aerial();
    if dir == None {
        return;
    }
    match MotionModule::motion_kind(boma) {
        super::hash40!("attack_air_n") if matches!(dir, Some(AerialKind::Nair) | Some(AerialKind::Bair)) => return,
        super::hash40!("attack_air_f") => return,
        super::hash40!("attack_air_b") => return,
        super::hash40!("attack_air_hi") => return,
        super::hash40!("attack_air_lw") => return,
        _ => {
            VarModule::on_flag(boma.object(), vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
            boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_AIR, false);
        }
    }
}

unsafe fn magic_flag_reset(boma: &mut BattleObjectModuleAccessor) {
    if !(boma.is_motion_one_of(&[Hash40::new("attack_12"),
                                 Hash40::new("attack_13"),
                                 Hash40::new("attack_s3_s"),
                                 Hash40::new("attack_hi3"),
                                 Hash40::new("attack_lw3"),
                                 Hash40::new("attack_s4"),
                                 Hash40::new("attack_hi4"),
                                 Hash40::new("attack_lw4"),
                                 Hash40::new("attack_air_n"),
                                 Hash40::new("attack_air_f"),
                                 Hash40::new("attack_air_b"),
                                 Hash40::new("attack_air_hi"),
                                 Hash40::new("attack_air_lw")])
        || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N,
                                   *FIGHTER_STATUS_KIND_SPECIAL_S,
                                   *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,
                                   *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK,
                                   *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,
                                   *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,
                                   *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK,
                                   *FIGHTER_STATUS_KIND_SPECIAL_HI,
                                   *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,
                                   *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP,
                                   *FIGHTER_STATUS_KIND_SPECIAL_LW,
                                   *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,
                                   *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK])){
            VarModule::off_flag(boma.object(), vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
        }
}

unsafe fn super_fs_cancel(boma: &mut BattleObjectModuleAccessor) -> bool {
    if MeterModule::drain(boma.object(), 6) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
        true
    } else {
        false
    }
}

unsafe fn magic_series(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    
    magic_flag_reset(boma);

    if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) /*&& VarModule::is_flag(boma.object(), vars::shotos::status::IS_ENABLE_MAGIC_SERIES_CANCEL)*/ {
        return;
    }

    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK]) {
        jab_cancels(boma);
        return;
    }
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_DASH]) {
        dash_attack_cancels(boma);
        return;
    }

    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3]) {
        tilt_cancels(boma);
        return;
    }

    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4]) {
        smash_cancels(boma);
        return;
    }

    // Aerial Cancels
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        aerial_cancels(boma);
        return;
    }

}

#[fighter_frame( agent = FIGHTER_KIND_DOLLY )]
pub fn dolly_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        MeterModule::update(fighter.object(), false);
        // if fighter.is_button_on(Buttons::AppealAll) {
        //     MeterModule::show(fighter.object());
        // } else {
        //     MeterModule::stop_show(fighter.object());
        // }
        utils::ui::UiManager::set_ff_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_ff_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            MeterModule::meter(fighter.object()),
            (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object())),
            MeterModule::meter_per_level(fighter.object())
        );
    }
}

#[utils::macros::opff(FIGHTER_KIND_DOLLY )]
pub fn dolly_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		dolly_frame(fighter)
    }
}

pub unsafe fn dolly_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
