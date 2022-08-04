use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Vector2f, Vector3f, Vector4f};
use smash::app::{self, lua_bind::*, sv_kinetic_energy, sv_animcmd};
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::phx::*;
use smash::hash40;
use smash::app::sv_animcmd::*;
use smash_script::*;
use crate::misc::*;
use globals::*;

unsafe fn hitstun_overlay_orange(boma: &mut BattleObjectModuleAccessor, id: usize) {
    let cmb_vec1 = Vector4f{x: 0.949, y: 0.5137, z: 0.08643, w: 0.69};
    let cmb_vec2 = Vector4f{x: 0.949, y: 0.5137, z: 0.08643, w: 0.0};
    if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0 {
        if  !VarModule::is_flag(boma.object(), vars::common::instance::IS_IN_HITSTUN) {
            VarModule::on_flag(boma.object(), vars::common::instance::HITSTUN_START);
        }
    } else {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_IN_HITSTUN) {
            ColorBlendModule::cancel_main_color(boma, 0);
        }
        VarModule::off_flag(boma.object(), vars::common::instance::IS_IN_HITSTUN);
    }
    if VarModule::is_flag(boma.object(), vars::common::instance::HITSTUN_START) {
        VarModule::on_flag(boma.object(), vars::common::instance::IS_IN_HITSTUN);
        ColorBlendModule::set_main_color(boma, &cmb_vec1, &cmb_vec2, 1.0, 0.5, 10, true);
        VarModule::off_flag(boma.object(), vars::common::instance::HITSTUN_START);
    }
}

//ecb visualizer :)
pub unsafe fn ecb_visualizer(boma: &mut BattleObjectModuleAccessor) {
    let center_pos = GroundModule::get_center_pos(boma);
    let offset_x = GroundModule::get_offset_x(boma);
    let offset_y = GroundModule::get_offset_y(boma);

    let pos_bottom = Vector3f {x: center_pos + offset_x, y: PostureModule::pos_y(boma) + offset_y, z: 15.0}; //need a positive Z value so the effect is in front of everything
    EffectModule::kill_kind(boma, Hash40::new("sys_ripstick_bullet"), true, true);
    EffectModule::req_2d(boma, Hash40::new("sys_ripstick_bullet"), &pos_bottom, &Vector3f::zero(), 0.25, 0);
}

pub unsafe fn buffer_clearing(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    // disables buffered wiggle inputs during high knockback
    if [*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_DASH);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_TURN_DASH);
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

pub unsafe fn airdodge_refresh_on_hit_disable(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    
    if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) && VarModule::is_flag(boma.object(), vars::common::instance::PREV_FLAG_DISABLE_ESCAPE_AIR) {
        //println!("dont refresh!");
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    }
    VarModule::set_flag(boma.object(), vars::common::instance::PREV_FLAG_DISABLE_ESCAPE_AIR, WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR));
}

pub unsafe fn suicide_throw_mashout(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_THROWN) && LinkModule::get_parent_status_kind(boma, *LINK_NO_CAPTURE) == *FIGHTER_STATUS_KIND_THROW_KIRBY as u64 {
        if fighter.global_table[CURRENT_FRAME].get_i32() <= 0 {
            let throw_frame = ParamModule::get_float(fighter.object(), ParamType::Common, "suicide_throw.throw_frame");
            let damage_frame_mul = ParamModule::get_float(fighter.object(), ParamType::Common, "suicide_throw.damage_frame_mul");
            let recovery_frame = ParamModule::get_float(fighter.object(), ParamType::Common, "suicide_throw.recovery_frame");
            let clatter_frame_base = ParamModule::get_float(fighter.object(), ParamType::Common, "suicide_throw.clatter_frame_base");
            let clatter_frame_max = ParamModule::get_float(fighter.object(), ParamType::Common, "suicide_throw.clatter_frame_max");
            let clatter_frame_min = ParamModule::get_float(fighter.object(), ParamType::Common, "suicide_throw.clatter_frame_min");
            let thrown_damage = DamageModule::damage(boma, 0);
            let thrower_damage = DamageModule::damage(boma.get_grabber_boma(), 0);
            let damage_difference = thrower_damage - thrown_damage;
            let clatter_frame_add = damage_difference * damage_frame_mul;
            let mut clatter_frame = clatter_frame_base + clatter_frame_add;

            if clatter_frame < clatter_frame_min {
                clatter_frame = clatter_frame_min;
            }
            if clatter_frame > clatter_frame_max {
                clatter_frame = clatter_frame_max;
            }
            
            ControlModule::start_clatter(boma, throw_frame, recovery_frame, clatter_frame, 127, 0, false, false);
        }
        else {
            if ControlModule::get_clatter_time(boma, 0) <= 0.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_CAPTURE_JUMP.into(), false.into());
            }
        }
    }
}
pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    
    
    buffer_clearing(boma, status_kind);
    //sliding_smash_disable(fighter, boma, status_kind, fighter_kind);
    airdodge_refresh_on_hit_disable(boma, status_kind);
    suicide_throw_mashout(fighter, boma);
}

