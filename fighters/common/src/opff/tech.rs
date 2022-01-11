use crate::opff_import::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f, Vector4f};
use smash::app::lua_bind::*;
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::hash40;
use smash::phx::Hash40;
use smash::cpp::root::app::SituationKind;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::{self, lua_bind::*, sv_kinetic_energy, sv_animcmd};


//=================================================================
//== TUMBLE EXIT
//=================================================================
unsafe fn tumble_exit(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32) {
    let id = hdr::get_player_number(boma);
    let remaining_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
    let hitstun_passed = total_hitstun - remaining_hitstun;
    /*
     * Pick: damage fall OR (damage_fly variant + hitstun + 5 frame)
     */

     // Prevent tumble escape flag from persisting successive damage_fly status kinds
    if [*FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
        if remaining_hitstun > 0.0 {
            can_escape_tumble[id] = false;
        }
    }

     // Ensure that you aren't in hitlag so you can't cancel out of hitlag
    if !(FighterStopModuleImpl::is_damage_stop(boma)) {
        if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL
            || ([*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind)
            && remaining_hitstun > 0.0 && hitstun_passed > 5.0) {
            if !(WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
                || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)) {
                /*
                // println!("Tumble knockback...");
                if [*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
                    // println!("Total hitstun: {}", total_hitstun);
                    // println!("Hitstun left: {}", remaining_hitstun);
                    // println!("Hitstun passed: {}", hitstun_passed);
                }
                */
                tumble_kb[id] = true;
            }
        }

        if ![*FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
            tumble_kb[id] = false;
            can_escape_tumble[id] = false;
            // println!("No more KB");
            // println!(" === No KB status kind: {}", status_kind);
            // println!(" Damage Fly status kind: {}", *FIGHTER_STATUS_KIND_DAMAGE_FLY);
        }

        if tumble_kb[id] && remaining_hitstun == 0.0 {
            // println!(" === CAN ESCAPE TUMBLE");
            // println!(" ---> Status Kind: {}", status_kind);
            can_escape_tumble[id] = true;
        }

        if can_escape_tumble[id] {
            // println!(" ESCAPE POSSIBLE ");
            if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_DASH
                                    | *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) {
                if situation_kind == *SITUATION_KIND_AIR {
                    tumble_kb[id] = false;
                    can_escape_tumble[id] = false;
                    // println!(" === TUMBLE ESCAPED");
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                }
            }
        }
    }

    /* OLD TUMBLE EXIT CODE */
    /*
    if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL
        && !(WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR) || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)) {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat1::Walk) || boma.is_cat_flag(Cat1::Turn) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
    */
}

//=================================================================
//== NON-TUMBLE KNOCKBACK DI
//=================================================================
unsafe fn non_tumble_di(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    //let prev_status = StatusModule::prev_status_kind(boma, 0);
    let status_kind_prev = StatusModule::prev_status_kind(boma, 0);

    if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR].contains(&status_kind) {

        //let stick_x = ControlModule::get_stick_x(boma);
        //let stick_y = ControlModule::get_stick_y(boma);
        //let di_stick = (stick_y.abs() + 0.00001) / (stick_x.abs() + 0.00001);
        //let di_angle = di_stick.atan();
        if FighterStopModuleImpl::get_damage_stop_frame(boma) as i32 == 1 {

            // println!("last frame of nontumble hitlag");
            /*
            if(stick_x.abs() > 0.1 || stick_y.abs() > 0.1){
                // println!("nontumble di angle input: {}", di_angle);
                // println!("");
                l2c_agent.clear_lua_stack(); //clear the stack from any previous args
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE as u64)); //push the first arg, that being a KINETIC_ENERGY_ID const
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(di_angle)); //push the second arg, that being a float of the new speed we want to set
                sv_kinetic_energy::set_angle(lua_state); //call the desired function with the lua state which will grab the args we previously pushed
            }
            */
            smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);

        }
        if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR].contains(&status_kind)
            && [*FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN, *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN, *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_THROWN].contains(&status_kind_prev) {
            let remaining_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
            let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
            let hitstun_passed = total_hitstun - remaining_hitstun;
            // println!("Nontumble throw");
            // println!("total hitstun: {}", total_hitstun);
            // println!("remaining hitstun: {}", remaining_hitstun);
            // println!("hitstun passed: {}", hitstun_passed);
            if total_hitstun > 0.0 && hitstun_passed == 1.0 {
                // println!(" === Nontumble throw DI frame");
                smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);
            }

        }
    }
    //let remaining_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    //let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
    //let hitstun_passed = total_hitstun - remaining_hitstun;
    // println!("Status Kind: {}", status_kind);
    // println!("total hitstun: {}", total_hitstun);
    // println!("remaining hitstun: {}", remaining_hitstun);
    // println!("hitstun passed: {}", hitstun_passed);
}

//=================================================================
//== ASDI
//=================================================================
unsafe fn asdi(fighter: &mut L2CFighterCommon, lua_state: u64, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //let prev_status = StatusModule::prev_status_kind(boma, 0);
    // println!(" ======= Current Status Kind: {}", status_kind);
    if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
        let stick_x = ControlModule::get_stick_x(boma);
        let stick_y = ControlModule::get_stick_y(boma);
        //let asdi_pulse_max = (WorkModule::get_param_float(boma, hash40("hit_stop_delay_flick_mul"), hash40("common"))) * (WorkModule::get_param_float(boma, hash40("hit_stop_delay_auto_mul"), hash40("common")));
        let asdi_pulse_max = 3.0;
        //let asdi_mag = ((stick_x ^2) + (stick_y ^2)).sqrt
        // println!("Current frames of hitlag: {}", WorkModule::get_int(boma, *FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME));
        // println!("Current frames of hitlag (tracked via FighterStopModuleImpl): {}", FighterStopModuleImpl::get_damage_stop_frame(boma));
        if FighterStopModuleImpl::get_damage_stop_frame(boma) as i32 == 1 {
            // println!("last frame of hitlag");
            if(stick_x.abs() > 0.1 || stick_y.abs() > 0.1){
                // println!("ASDI x: {}", stick_x);
                // println!("ASDI y: {}", stick_y);
                // println!("ASDI max magnitude: {}", asdi_pulse_max);
                // println!("Pre-ASDI x position: {}", PostureModule::pos_x(boma));
                // println!("Pre-ASDI y position: {}", PostureModule::pos_y(boma));
                // println!("Inputting ASDI pulse");
                if situation_kind == *SITUATION_KIND_GROUND && stick_y > 0.0 {
                    StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), false);
                }
                let asdi_vec = Vector3f {x: asdi_pulse_max * stick_x * 50.0, y: asdi_pulse_max * stick_y * 50.0, z: 0.0};
                PostureModule::add_pos(boma, &asdi_vec);
                // println!("Post-ASDI x position: {}", PostureModule::pos_x(boma));
                // println!("Post-ASDI y position: {}", PostureModule::pos_y(boma));
            }
        }


    }
}

// plat drop if you input down during a waveland (airdodge landing lag)
unsafe fn waveland_plat_drop(boma: &mut BattleObjectModuleAccessor, cat2: i32, status_kind: i32) {
    let plat_drop_statuses = [
        *FIGHTER_STATUS_KIND_ESCAPE_AIR,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
    ];
    let prev_status = StatusModule::prev_status_kind(boma, 0);
    if status_kind == *FIGHTER_STATUS_KIND_LANDING && plat_drop_statuses.contains(&prev_status) {
        
        // if we are allowed to drop yet
        if (VarModule::is_flag(boma.object(), common::ENABLE_WAVELAND_PLATDROP)) {
            if GroundModule::is_passable_ground(boma) && hdr::stick_y_flick_check(boma, -0.66) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
            }
        }
    }
    let reset_statuses = [
        *FIGHTER_STATUS_KIND_ESCAPE_AIR,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
        *FIGHTER_STATUS_KIND_LANDING
    ];
    if reset_statuses.contains(&status_kind) {
        // if stick set to neutral, toggle waveland platdrop flag
        if ControlModule::get_stick_y(boma) > -0.3 {
            VarModule::on_flag(boma.object(), common::ENABLE_WAVELAND_PLATDROP);
        }
    }
}


//=================================================================
//== DASH DROP
//=================================================================
unsafe fn dash_drop(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE,
        * FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
        if GroundModule::is_passable_ground(boma) && hdr::stick_y_flick_check(boma, -0.66) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, false);
        }
    }
}

//=================================================================
//== CROUCH DURING RUN
//=================================================================
unsafe fn run_squat(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_y: f32) {
    //let crouch_thresh: f32 = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"));
    if status_kind == *FIGHTER_STATUS_KIND_RUN || status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE {
        if stick_y < -0.66 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
    }
}

//=================================================================
//== GLIDE TOSS
//=================================================================
unsafe fn glide_toss(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, facing: f32) {
    let id = hdr::get_player_number(boma);
    let prev_status = StatusModule::prev_status_kind(boma, 0);

    if [*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&status_kind) {
        if MotionModule::frame(boma) <= 6.0 {
            can_glide_toss[id] = true;
            VarModule::set_float(fighter.battle_object, vars::common::ROLL_DIR, facing);
        }
        else {
            can_glide_toss[id] = false;
        }
    }

    if status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW {
        if (prev_status == *FIGHTER_STATUS_KIND_ESCAPE_F) && can_glide_toss[id] {
            let motion_value: f32 = 2.8 * (MotionModule::end_frame(boma) - MotionModule::frame(boma)) / MotionModule::end_frame(boma);
            let motion_vec = Vector3f {x:  motion_value * VarModule::get_float(fighter.battle_object, vars::common::ROLL_DIR), y: 0.0, z: 0.0};
            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
        if (prev_status == *FIGHTER_STATUS_KIND_ESCAPE_B) && can_glide_toss[id] {
            let motion_value: f32 = 2.8 * (MotionModule::end_frame(boma) - MotionModule::frame(boma)) / MotionModule::end_frame(boma);
            let motion_vec = Vector3f {x:  motion_value * VarModule::get_float(fighter.battle_object, vars::common::ROLL_DIR) * -1.0, y: 0.0, z: 0.0};
            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }
}

//=================================================================
//== PIVOTS
//=================================================================
const PIVOT_STICK_SNAPBACK_WINDOW: f32 = 2.0;
unsafe fn pivots(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_value_x: f32, curr_frame: f32){

    // Get the pivot boost amount for the current character
    let dash_speed: f32 = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let mut pivot_boost: smash::phx::Vector3f = smash::phx::Vector3f {x: dash_speed * 0.75, y: 0.0, z: 0.0};
    if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
        && curr_frame <= PIVOT_STICK_SNAPBACK_WINDOW && stick_value_x == 0.0
        && [*FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH].contains(&StatusModule::prev_status_kind(boma, 0))
        && ![*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_TURN].contains(&StatusModule::prev_status_kind(boma, 1))
    {
        /*
        if curr_frame == 3.0 {
            pivot_boost.x = dash_speed * 0.35; // Reduce pivot speed boost if on the last frame of pivot leniency
        }
        */
        PostureModule::reverse_lr(boma);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN,true);
        KineticModule::clear_speed_all(boma);
        KineticModule::add_speed(boma, &pivot_boost);
    }
}


//=================================================================
//== MOONWALKS
//== Note: There may be some unforeseen kinks to work out
//=================================================================
#[derive(Copy, Clone)]
pub enum MwState {
    BEGIN,
    LOOP,
    SLIDE,
    WAIT,
}
unsafe fn moonwalk(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    let id = hdr::get_player_number(boma);

    match state[id] {
        MwState::BEGIN => {
            if status_kind == *FIGHTER_STATUS_KIND_DASH || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH {
                state[id] = MwState::LOOP;
            }
        }
        MwState::LOOP => {
            if stick_x == 0.0 && stick_y == 0.0 {
                state[id] = MwState::WAIT;
            } else if PostureModule::lr(boma) * stick_x < 0.0 {
                let x_vel = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                // Reverse momentum
                //KineticModule::add_speed(boma, &Vector3f {x: x_vel.abs() * -2.0 * stick_x.abs(), y: 0.0, z: 0.0});
                state[id] = MwState::SLIDE;
            } else if status_kind != *FIGHTER_STATUS_KIND_DASH || status_kind != *FIGHTER_STATUS_KIND_TURN_DASH {
                state[id] = MwState::BEGIN;
            } else {
                // Waiting for input
            }
        }
        MwState::SLIDE => {
            let x_vel = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
            let run_accel_add = WorkModule::get_param_float(boma, hash40("run_accel_add"), 0);
            let run_accel_mul = WorkModule::get_param_float(boma, hash40("run_accel_mul"), 0);
            let vel_step = Vector3f {x: (run_accel_add + run_accel_mul * stick_x.abs()) * -1.0, y: 0.0, z: 0.0};

            //WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
            if vel_step.x.abs() + x_vel.abs() <= run_speed_max {
                KineticModule::add_speed(boma, &vel_step);
            }
            if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                PostureModule::set_lr(boma, x_vel.signum() * -1.0);
                PostureModule::update_rot_y_lr(boma);
            }
            if status_kind != *FIGHTER_STATUS_KIND_DASH && status_kind != *FIGHTER_STATUS_KIND_TURN_DASH {
                state[id] = MwState::BEGIN;
            }
        }
        MwState::WAIT => {
            if status_kind != *FIGHTER_STATUS_KIND_DASH && status_kind != *FIGHTER_STATUS_KIND_TURN_DASH {
                state[id] = MwState::BEGIN;
            }
        }
    };
}

//=================================================================
//== MOONWALKS
//== Version 8.4.0a Implementation
//=================================================================
unsafe fn moonwalks(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_value_x: f32, stick_value_y: f32, facing: f32) {
    /* Moonwalk melee calculation: (stick_pos.x * run_accel_mul) + (sign(stick_pos.x) * run_accel_add) */
    let run_speed = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
    let run_accel_mul = WorkModule::get_param_float(boma, hash40("run_accel_mul"), 0);
    let run_accel_add = WorkModule::get_param_float(boma, hash40("run_accel_add"), 0);
    let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
    let dash_speed: f32 = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let stick_x = fighter.global_table[hdr_modules::consts::globals::STICK_X].get_f32();
    let is_dash_input: bool = ((ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0) || ((ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0);
    let mut moonwalk_go: f32 = 0.0;
    if dash_speed > run_speed{
        moonwalk_go = dash_speed;
    }
    else{
        moonwalk_go = run_speed;
    }
    if [*FIGHTER_STATUS_KIND_DASH].contains(&status_kind) {
        if stick_value_x*facing < -0.1 && stick_value_y < -0.65 {
            VarModule::on_flag(fighter.battle_object, vars::common::IS_MOONWALK);
            VarModule::on_flag(fighter.battle_object, vars::common::IS_MOONWALK_JUMP);
        }
        if !is_dash_input && MotionModule::frame(boma) > 2.0 && stick_value_x*facing < -0.18 /*Walk stick sensitivity*/ && VarModule::is_flag(fighter.battle_object, vars::common::IS_MOONWALK) {  // If you haven't input a turn dash, your dash frame isn't at the start of a turn dash, your stick is backwards, and the moonwalk input is valid
            let mut prev_speed = 0.0;
            if KineticModule::is_enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                prev_speed = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            }
            else {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
                prev_speed = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
            }
            let added_speed = stick_value_x.signum() * ((run_accel_mul + (run_accel_add * stick_value_x.abs())));
            let moonwalk_speed = prev_speed + added_speed;
            let moonwalk_speed_clamped = clamp(moonwalk_speed, -run_speed, run_speed);

            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            app::sv_kinetic_energy::unable(fighter.lua_state_agent);

            // println!("moonwalkin: {}", moonwalk_speed);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
            app::sv_kinetic_energy::enable(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP, moonwalk_speed_clamped);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        }
        else {
            if MotionModule::frame(boma) > 2.0 && VarModule::is_flag(fighter.battle_object, vars::common::IS_MOONWALK) {
                // println!("moonwalk off dash");
                VarModule::off_flag(fighter.battle_object, vars::common::IS_MOONWALK);
                if !is_dash_input {
                    // println!("no dash");
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
                    let speed_stop = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
                    let added_speed = speed_stop - (facing * -2.0 * ground_brake);
                    let added_speed_clamped = clamp(added_speed, -run_speed, run_speed);

                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                    app::sv_kinetic_energy::enable(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, added_speed_clamped);
                    app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
                    app::sv_kinetic_energy::unable(fighter.lua_state_agent);
                }
            }
        }
    }
}

unsafe fn shield_lock_tech(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    // airdodge with second shield button while holding another shield button
    if situation_kind == *SITUATION_KIND_AIR
    && [*FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_PASS,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3].contains(&status_kind) {
        if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD_HOLD) {
            if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
            }
        }
    }

    // allow button jump during shield lock
    if status_kind == *FIGHTER_STATUS_KIND_GUARD_ON || status_kind == *FIGHTER_STATUS_KIND_GUARD {
        let special_buttons = [
        *CONTROL_PAD_BUTTON_SPECIAL,
        *CONTROL_PAD_BUTTON_SPECIAL_RAW
        ];
        let special_disabled = WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_GUARD_HOLD_SPECIAL_BUTTON);
        let special_hold = special_buttons.iter().any(|x| ControlModule::check_button_on(boma, *x));
        let guard_hold = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD_HOLD);

        if (special_hold && !special_disabled) || guard_hold {
            if boma.is_cat_flag(Cat1::Jump) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
        }
    }
}

unsafe fn tap_upB_jump_refresh(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
        // if using tap jump (until I find a better way to check)
        if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) && !ControlModule::check_button_release(boma, *CONTROL_PAD_BUTTON_JUMP) {
            // if first 3 frames of dj
            if MotionModule::frame(boma) <= 2.0 {
                VarModule::on_flag(boma.object(), common::UP_SPECIAL_JUMP_REFRESH_WINDOW);
            }
            else {
                VarModule::off_flag(boma.object(), common::UP_SPECIAL_JUMP_REFRESH_WINDOW);
            }
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && VarModule::is_flag(boma.object(), common::UP_SPECIAL_JUMP_REFRESH_WINDOW) && !VarModule::is_flag(boma.object(), common::DISABLE_UP_SPECIAL_JUMP_REFRESH) {
        // Grants 1 extra jump if all jumps used up
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            WorkModule::set_int(boma, WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        VarModule::on_flag(boma.object(), common::DISABLE_UP_SPECIAL_JUMP_REFRESH);
        VarModule::off_flag(boma.object(), common::UP_SPECIAL_JUMP_REFRESH_WINDOW);
    }
    if situation_kind == *SITUATION_KIND_GROUND && VarModule::is_flag(boma.object(), common::DISABLE_UP_SPECIAL_JUMP_REFRESH) {
        VarModule::off_flag(boma.object(), common::DISABLE_UP_SPECIAL_JUMP_REFRESH);
    }
}

 
unsafe fn drift_di(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if boma.is_situation(*SITUATION_KIND_AIR)
    && !StopModule::is_stop(boma)
    && boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
    ])
    {
        let speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);

        let mut speed_mul = ParamModule::get_float(fighter.battle_object, ParamType::Common, "drift_di.speed_mul_base");
        let speed_mul_add_max = ParamModule::get_float(fighter.battle_object, ParamType::Common, "drift_di.speed_mul_add_max");

        let lerp_min_speed = ParamModule::get_float(fighter.battle_object, ParamType::Common, "drift_di.speed_lerp_min");
        let lerp_max_speed = ParamModule::get_float(fighter.battle_object, ParamType::Common, "drift_di.speed_lerp_max");

        if speed_x.abs() < lerp_min_speed {
            speed_mul += speed_mul_add_max;
        } else if speed_x.abs() < lerp_max_speed {
            let ratio = 1.0 - ((speed_x.abs() - lerp_min_speed) / (lerp_max_speed - lerp_min_speed));
            speed_mul += ratio * speed_mul_add_max;
        }

        let drift_value = fighter.stick_x() * speed_mul;
        fighter.set_speed(Vector2f::new(speed_x + drift_value, speed_y), *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    }
}


extern "C" {
    #[link_name = "\u{1}_ZN3app14sv_information8stage_idEv"]
    pub fn stage_id() -> i32;
}

pub unsafe fn freeze_stages(boma: &mut BattleObjectModuleAccessor) {

    // determine the current stage id
    //println!("stage id: {}", stage_id());

    // warioware
    if (stage_id() == 104) {
        smash::app::FighterUtil::set_stage_pause_for_final(true, boma);
    }
}

pub unsafe fn hitfall(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, cat: [i32 ; 4]) {
    if boma.kind() == *FIGHTER_KIND_GAOGAEN
    && boma.is_situation(*SITUATION_KIND_AIR)
    && boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR)
    {
        /* this is written this way because stick_y_flick wont update during
            hitlag, which means we need a flag to allow you to hitfall 1 frame
            after the end of hitlag as well, and we need to check previous 
            stick y directly to detect hitfall. That way, with the 5 frame buffer,
            if you input a fastfall during hitlag, it will get registered after
            the hitlag is over. Without the HITFALL_BUFFER flag, you have to
            input the fastfall BEFORE you hit the move, only.
        */
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        || AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        {
            VarModule::set_int(boma.object(), vars::common::HITFALL_BUFFER, 0);
        }

        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            VarModule::inc_int(boma.object(), vars::common::HITFALL_BUFFER);
        }

        let buffer = VarModule::get_int(boma.object(), vars::common::HITFALL_BUFFER);

        if boma.is_cat_flag(Cat2::FallJump)
        && 0 < buffer && buffer <= 5 
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
}

pub unsafe fn respawn_taunt(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if !boma.is_status(*FIGHTER_STATUS_KIND_REBIRTH) {
        return;
    }

    match MotionModule::motion_kind(boma) {
        utils::hash40!("appeal_hi_r") => return,
        utils::hash40!("appeal_hi_l") => return,
        utils::hash40!("appeal_lw_r") => return,
        utils::hash40!("appeal_lw_l") => return,
        utils::hash40!("appeal_s_l") => return,
        utils::hash40!("appeal_s_r") => return,
        _ => {}
    }

    let motion = if boma.is_button_trigger(Buttons::AppealHi) {
        if PostureModule::lr(boma) == 1.0 {
            Hash40::new("appeal_hi_r")
        } else {
            Hash40::new("appeal_hi_l")
        }
    } else if boma.is_button_trigger(Buttons::AppealSL) {
        Hash40::new("appeal_s_l")
    } else if boma.is_button_trigger(Buttons::AppealSR) {
        Hash40::new("appeal_s_r")
    } else if boma.is_button_trigger(Buttons::AppealLw) {
        if PostureModule::lr(boma) == 1.0 {
            Hash40::new("appeal_lw_r")
        } else {
            Hash40::new("appeal_lw_l")
        }
    } else {
        return;
    };

    MotionModule::change_motion(boma, motion, 0.0, 1.0, false, 0.0, false, false);
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32, curr_frame: f32) {
    tumble_exit(boma, cat[0], status_kind, situation_kind);
    non_tumble_di(fighter, lua_state, l2c_agent, boma, status_kind);
    //asdi(fighter, lua_state, boma, status_kind); // Unused
    dash_drop(boma, status_kind);
    run_squat(boma, status_kind, stick_y); // Must be done after dash_drop()
    glide_toss(fighter, boma, status_kind, facing);
    //moonwalk(boma, status_kind, stick_x, stick_y, facing);  // State-based implementation
    //moonwalks(fighter, boma, status_kind, stick_x, stick_y, facing);  // Version 8.4.0a implementation
    //pivots(boma, status_kind, stick_x, curr_frame);
    shield_lock_tech(boma, status_kind, situation_kind, cat[0]);
    //tap_upB_jump_refresh(fighter, boma, status_kind, situation_kind, fighter_kind, cat[0]);
    drift_di(fighter, boma, status_kind, situation_kind);
    waveland_plat_drop(boma, cat[1], status_kind);
    hitfall(boma, status_kind, situation_kind, fighter_kind, cat);
    respawn_taunt(boma, status_kind);
    

    /*if BufferModule::is_persist(boma) && VarModule::is_flag(boma.object(), common::FLOAT_PAUSE_AERIAL) {
        VarModule::off_flag(boma.object(), common::FLOAT_PAUSE_AERIAL);
        let cbm_vec1 = Vector4f{x: 0.95, y: 0.95, z: 0.95, w: 0.2};
        let cbm_vec2 = Vector4f{x: 0.0, y: 0.0, z: 0.3, w: 0.8};
        ColorBlendModule::set_main_color(boma, &cbm_vec1, &cbm_vec2, 0.5, 2.2, 2, true);
    }
    if !BufferModule::is_persist(boma) && !VarModule::is_flag(boma.object(), common::FLOAT_PAUSE_AERIAL) {
        VarModule::on_flag(boma.object(), common::FLOAT_PAUSE_AERIAL);
        ColorBlendModule::cancel_main_color(boma, 0);
    }*/

    freeze_stages(boma);
}
    
