use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Vector2f, Vector3f, Vector4f};
use smash::app::{self, lua_bind::*, sv_kinetic_energy, sv_animcmd};
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::phx::*;
use smash::hash40;
use smash::app::sv_animcmd::*;
use smash_script::*;

use hdr_modules::consts::{*, globals::*};
use hdr_modules::VarModule;

use crate::utils::hdr;
use crate::vars::*;
use crate::hooks::general_mechanics::jump::calc_melee_momentum;

unsafe fn hitstun_overlay_orange(boma: &mut BattleObjectModuleAccessor, id: usize) {
    let cmb_vec1 = Vector4f{x: 0.949, y: 0.5137, z: 0.08643, w: 0.69};
    let cmb_vec2 = Vector4f{x: 0.949, y: 0.5137, z: 0.08643, w: 0.0};
    if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0 {
        if !is_in_hitstun[id] {
            hitstun_start[id] = true;
        }
    } else {
        if is_in_hitstun[id] {
            ColorBlendModule::cancel_main_color(boma, 0);
        }
        is_in_hitstun[id] = false;
    }
    if hitstun_start[id] {
        is_in_hitstun[id] = true;
        ColorBlendModule::set_main_color(boma, &cmb_vec1, &cmb_vec2, 1.0, 0.5, 10, true);
        hitstun_start[id] = false;
    }
}

//ecb visualizer :)
pub unsafe fn ecb_visualizer(boma: &mut BattleObjectModuleAccessor) {
    let center_pos = GroundModule::get_center_pos(boma);
    let offset_x = GroundModule::get_offset_x(boma);
    let offset_y = GroundModule::get_offset_y(boma);

    let pos_bottom = Vector3f {x: center_pos + offset_x, y: PostureModule::pos_y(boma) + offset_y, z: 15.0}; //need a positive Z value so the effect is in front of everything
    EffectModule::kill_kind(boma, Hash40::new("sys_ripstick_bullet"), true, true);
    EffectModule::req_2d(boma, Hash40::new("sys_ripstick_bullet"), &pos_bottom, &hdr::DEFAULT_VECTOR3, 0.25, 0);
}

pub unsafe fn damage_slideoff_airdodge_disable(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
	if [*FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_SLIP_WAIT, *FIGHTER_STATUS_KIND_DAMAGE].contains(&status_kind) {
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
}

pub unsafe fn sliding_smash_disable(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_START && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_TURN_DASH && MotionModule::frame(boma) <= 1.0 {
        if fighter_kind != *FIGHTER_KIND_ROCKMAN {
            let mut f_agent = fighter.agent;

            f_agent.clear_lua_stack();
            f_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_MOTION as u64));
            f_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            f_agent.clear_lua_stack();
        }
    }
}

// Basically what we're doing here is:
//  1. If you just did a tap jump buffered cstick aerial, and your stick is neutral/below neutral on frame 2 of jump (the frame cstick stops overriding left stick), we override fullhop and set your jump speed to what it would be on frame 2 of shorthop
//  2. If you just did a buffered cstick aerial, use the stick_x value on the first frame your cstick stops overriding your left stick to calculate your jump momentum
pub unsafe fn buffered_cstick_aerial_fixes(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_JUMP {
        // if frame 2 of buffered aerial
        if fighter.global_table[CURRENT_FRAME].get_i32() == 1 {
            // this is first frame cstick stops overriding left stick if input on second-to-last frame of jumpsquat
            if VarModule::is_flag(fighter.module_accessor, common::CSTICK_OVERRIDE_SECOND) {
                // set proper jump x speed using stick_x value on this frame
                let new_speed = calc_melee_momentum(fighter, false, false, false);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, new_speed);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                VarModule::set_float(fighter.module_accessor, common::CURRENT_MOMENTUM, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND));
                
                if VarModule::is_flag(fighter.module_accessor, common::IS_TAP_JUMP) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI) && fighter.global_table[STICK_Y].get_f32() <= 0.0 {
                    // setting shorthop speed here
                    let mini_jump_y = WorkModule::get_param_float(fighter.module_accessor, hash40("mini_jump_y"), 0);
                    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
                    let mut f_agent = fighter.agent;
        
                    let shorthop_y_speed = (air_accel_y * (mini_jump_y / (0.5 * air_accel_y)).sqrt()) - air_accel_y; // this is your shorthop y speed on frame 2 of jump
                    //println!("shorthop_y_speed: {}", shorthop_y_speed);
                    
                    f_agent.clear_lua_stack();
                    f_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
                    f_agent.push_lua_stack(&mut L2CValue::new_num(shorthop_y_speed));
                    sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                    f_agent.clear_lua_stack();
                }
            }
        }
        // if frame 3 of buffered aerial
        if fighter.global_table[CURRENT_FRAME].get_i32() == 2 {
            // this is first frame cstick stops overriding left stick if input on last frame of jumpsquat
            if VarModule::is_flag(fighter.module_accessor, common::CSTICK_OVERRIDE) && !VarModule::is_flag(fighter.module_accessor, common::CSTICK_OVERRIDE_SECOND) {
                // set proper jump x speed using stick_x value on this frame
                let new_speed = calc_melee_momentum(fighter, false, false, false);
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, new_speed);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
                VarModule::set_float(fighter.module_accessor, common::CURRENT_MOMENTUM, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND));
            }
        }
    }
}

pub unsafe fn airdodge_refresh_on_hit_disable(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    
    if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) && VarModule::is_flag(boma, common::PREV_FLAG_DISABLE_ESCAPE_AIR) {
        //println!("dont refresh!");
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    }
    VarModule::set_flag(boma, common::PREV_FLAG_DISABLE_ESCAPE_AIR, WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR));
}

pub unsafe fn tumble_timer(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL {
        if fighter.global_table[CURRENT_FRAME].get_i32() <= 20 {
            VarModule::on_flag(boma, common::DISABLE_AIRDODGE);
        }
        else {
            VarModule::off_flag(boma, common::DISABLE_AIRDODGE);
        }
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    let id = hdr::get_player_number(boma);
    if DEBUG && hdr::is_training_mode() {
        hitstun_overlay_orange(boma, id);
        if id == 0 {
            ecb_visualizer(boma);
        }
    }
    damage_slideoff_airdodge_disable(boma, status_kind);
    sliding_smash_disable(fighter, boma, status_kind, fighter_kind);
    buffered_cstick_aerial_fixes(fighter, boma, status_kind);
    airdodge_refresh_on_hit_disable(boma, status_kind);
    tumble_timer(fighter, boma, status_kind);
}
