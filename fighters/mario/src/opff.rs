// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn dair_mash_rise(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, motion_kind: u64, situation_kind: i32, frame: f32) {
    if motion_kind == hash40("attack_air_lw") {
        //let motion_vec = Vector3f{x:0.0, y: 2.5, z: 0.0};
        let cbm_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2};
        let cbm_vec2 = Vector4f{ /* Red */ x: 0.9907, /* Green */ y: 0.02, /* Blue */ z: 0.0251, /* Alpha */ w: 0.2};
        let rise_amount = 0.275;
        let max_drift_speed = 0.955;
        let max_rise_speed = 0.815;
        let mut motion_vec = Vector3f{x:0.0, y: rise_amount, z: 0.0};
        let x_speed = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let y_speed = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let facing = PostureModule::lr(boma);
        if frame <= 29.0 {
            if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
                // Tell the game that you've started rising
                VarModule::on_flag(boma.object(), vars::mario::status::AERIAL_COMMAND_RISING);
                // Add vertical speed for the dair rise if you've activated the rise and this isn't your second time attempting to initiate the rise during your current airtime
                if VarModule::is_flag(boma.object(), vars::mario::status::AERIAL_COMMAND_RISING) &&  !VarModule::is_flag(boma.object(), vars::mario::status::AERIAL_COMMAND_RISEN) {
                    // Reset momentum on the first special button press press
                    if  !VarModule::is_flag(boma.object(), vars::mario::status::AERIAL_COMMAND_MOMENTUM_RESET){
                        // Slow down the move to better facilitate recovering
                        MotionModule::set_rate(boma, 0.5);
                        // Have mario glow a bit to indicate that he's recovering
                        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_vec1, /* Diffuse */ &cbm_vec2, 0.21, 1.0, 5, /* Display Color */ true);
                        // Reset momentum
                        KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                        KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                        KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                        VarModule::on_flag(boma.object(), vars::mario::status::AERIAL_COMMAND_MOMENTUM_RESET);
                    }
                    //KineticModule::add_speed(boma, &motion_vec);
                    if y_speed + motion_vec.y > max_rise_speed {
                        motion_vec.y = max_rise_speed - y_speed;
                    }
                    KineticModule::add_speed(boma, &motion_vec);
                    motion_vec.y = rise_amount;

                    /*
                    if x_speed.abs() > max_drift_speed {
                        motion_vec.x = (max_drift_speed * facing) - x_speed;
                    }
                    KineticModule::add_speed(boma, &motion_vec);
                    motion_vec.x = 0.0;
                    */
                    //KineticModule::add_speed_outside(boma,*KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION,&motion_vec);
                }
            }
        }
    }

    if VarModule::is_flag(boma.object(), vars::mario::status::AERIAL_COMMAND_RISING) {
        if motion_kind != hash40("attack_air_lw")
            || (motion_kind == hash40("attack_air_lw") && frame > 29.0) {
            ColorBlendModule::cancel_main_color(boma, 0);
            VarModule::on_flag(boma.object(), vars::mario::status::AERIAL_COMMAND_RISEN);
            VarModule::off_flag(boma.object(), vars::mario::status::AERIAL_COMMAND_RISING);
            VarModule::off_flag(boma.object(), vars::mario::status::AERIAL_COMMAND_MOMENTUM_RESET);
        }
    }

    // If grounded, reset aerial rise and momentum reset flags
    if situation_kind == *SITUATION_KIND_GROUND && VarModule::is_flag(boma.object(), vars::mario::status::AERIAL_COMMAND_RISEN) {
        VarModule::off_flag(boma.object(), vars::mario::status::AERIAL_COMMAND_RISEN);
        VarModule::off_flag(boma.object(), vars::mario::status::AERIAL_COMMAND_MOMENTUM_RESET);
    }
}

// Super Jump Punch Wall Jump
unsafe fn up_b_wall_jump(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if situation_kind == *SITUATION_KIND_AIR {
            if frame >= 22.0 && frame <= 35.0 {
                if  !VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_WALL_JUMP) {
                    if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32) {
                        if boma.is_cat_flag(Cat1::TurnDash) {
                            VarModule::on_flag(boma.object(), vars::common::instance::SPECIAL_WALL_JUMP);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                        }
                    }
                    if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32) {
                        if boma.is_cat_flag(Cat1::TurnDash) {
                            VarModule::on_flag(boma.object(), vars::common::instance::SPECIAL_WALL_JUMP);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                        }
                    }
                }
            }
        }
    }
}

unsafe fn dspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    //PM-like down-b canceling
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if situation_kind == *SITUATION_KIND_AIR {
            WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
        }
    }
    if status_kind == *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat1::AirEscape) {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

// Double fireball handling
unsafe fn double_fireball(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) && VarModule::is_flag(boma.object(), vars::mario::instance::CAN_INPUT_SPECIAL_N_DOUBLE_FIREBALL) {
        let restart_frame = 10.0;
        if boma.is_cat_flag(Cat1::SpecialN) || boma.is_cat_flag(Cat1::SpecialS) || boma.is_cat_flag(Cat1::SpecialHi) || boma.is_cat_flag(Cat1::SpecialLw){
            VarModule::off_flag(fighter.battle_object, vars::mario::status::IS_SPECIAL_N_FIREBRAND);
            VarModule::off_flag(boma.object(), vars::mario::instance::CAN_INPUT_SPECIAL_N_DOUBLE_FIREBALL);
            VarModule::on_flag(boma.object(), vars::mario::instance::SPECIAL_N_DOUBLE_FIREBALL_NOTIFY_FLAG);
            //MotionModule::set_frame_sync_anim_cmd(boma, restart_frame, true, true, false);
            boma.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_N, false);
        }
    }
}

// Once down special is called, imediately uses special low shoot and circumvent the charge mechanic of the og down-b
unsafe fn galaxy_spin_poc(fighter: &mut L2CFighterCommon ,boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, true);
    }
}

// Aerial SMG spin rise
unsafe fn galaxy_spin_rise(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, situation_kind: i32, frame: f32) {
    let fighter_gravity = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut FighterKineticEnergyGravity;
    let air_fri = Vector3f{x: 0.89, y: 1.0, z: 0.0}; // air friction
    if motion_kind == hash40("special_air_lw_light") {
        if fighter.is_status(*FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT) {
            if VarModule::is_flag(fighter.battle_object, vars::mario::instance::DISABLE_DSPECIAL_STALL) && fighter.is_situation(*SITUATION_KIND_AIR)  {
                if frame >= 6.0 && frame < 35.0 {  
                    smash::app::lua_bind::FighterKineticEnergyGravity::set_speed(fighter_gravity, 1.12);
                    KineticModule::mul_speed(boma, &air_fri, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(fighter_gravity, -0.06);
                    smash::app::lua_bind::FighterKineticEnergyGravity::set_stable_speed(fighter_gravity, -0.2);
                }
                if frame >= 35.0 && frame < 45.0 {
                    smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(fighter_gravity, -0.07);
                    smash::app::lua_bind::FighterKineticEnergyGravity::set_stable_speed(fighter_gravity, -0.2);
                    KineticModule::mul_speed(boma, &air_fri, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                }
                if frame >= 45.0 && frame < 50.0 {
                    smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(fighter_gravity, -0.075);
                    smash::app::lua_bind::FighterKineticEnergyGravity::set_stable_speed(fighter_gravity, -0.2);
                    KineticModule::mul_speed(boma, &air_fri, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                }
                if frame >= 50.0 {
                    VarModule::off_flag(fighter.battle_object, vars::mario::instance::DISABLE_DSPECIAL_STALL);
                }
            }  
        }
    }
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY]) 
    && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT {
        VarModule::off_flag(fighter.battle_object, vars::mario::instance::DISABLE_DSPECIAL_STALL);
    }

    if fighter.is_situation(*SITUATION_KIND_GROUND) 
    || fighter.is_situation(*SITUATION_KIND_CLIFF) 
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD])
    || fighter.is_situation(*SITUATION_KIND_LADDER) {
        VarModule::on_flag(fighter.battle_object, vars::mario::instance::DISABLE_DSPECIAL_STALL);
    }
}

// Grounded SMG spin movement
unsafe fn galaxy_spin_move(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, situation_kind: i32, frame: f32, stick_x: f32, facing: f32) {
    if motion_kind == hash40("special_lw_light") {
        let current_speed =  KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if fighter.is_status(*FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT) && fighter.is_situation(*SITUATION_KIND_GROUND){
            if frame >= 6.0 && frame < 20.0 { 
                if stick_x != 0.0 {
                    let motion_vec = x_motion_vec(1.2, stick_x);
                    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
                }
            }
            if frame >= 20.0 && frame < 55.0 { 
                if stick_x != 0.0 {
                    let motion_value = 1.2 + (frame - 20.0) * ((0.01 - 1.2)/(55.0 - 20.0));
                    let motion_vec = x_motion_vec(motion_value, stick_x);
                    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
                }
            }
        }
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// NokNok Shell Timer Count
unsafe fn noknok_timer(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    let gimmick_timerr = VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER);
    if gimmick_timerr > 0 && gimmick_timerr < 1801 {
        if gimmick_timerr > 1799 {
            VarModule::off_flag(boma.object(), vars::mario::instance::NOKNOK_SHELL);
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
            gimmick_flash(boma);
        } else {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, gimmick_timerr + 1);
        }
    }
}

// NokNok shell flag reset
unsafe fn noknok_reset(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if VarModule::is_flag(fighter.battle_object, vars::mario::instance::NOKNOK_SHELL) {
        if [*FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_WIN,
            *FIGHTER_STATUS_KIND_LOSE,
            *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
                VarModule::off_flag(fighter.battle_object, vars::mario::instance::NOKNOK_SHELL);
        }
    }
}

// TRAINING MODE
// NokNok shell flag reset via taunt
unsafe fn noknok_training(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            VarModule::off_flag(fighter.battle_object, vars::mario::instance::NOKNOK_SHELL);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
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
    //dair_mash_rise(fighter, boma, id, motion_kind, situation_kind, frame);
    up_b_wall_jump(fighter, boma, id, status_kind, situation_kind, cat[0], frame);
    dspecial_cancels(boma, status_kind, situation_kind, cat[0]);
    //double_fireball(fighter, boma);
    galaxy_spin_poc(fighter, boma, status_kind);
    galaxy_spin_rise(fighter, boma, status_kind, motion_kind, situation_kind, frame);
    galaxy_spin_move(fighter, boma, status_kind, motion_kind, situation_kind, frame, stick_x, facing);
    noknok_timer(fighter, boma, id);
    noknok_reset(fighter, id, status_kind);
    noknok_training(fighter, id, status_kind);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_MARIO )]
pub fn mario_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		mario_frame(fighter)
    }
}

pub unsafe fn mario_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
