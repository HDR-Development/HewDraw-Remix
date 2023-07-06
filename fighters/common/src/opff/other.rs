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
use crate::util::get_fighter_common_from_accessor;

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

pub unsafe fn airdodge_refresh_on_hit_disable(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    
    if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) && VarModule::is_flag(boma.object(), vars::common::instance::PREV_FLAG_DISABLE_ESCAPE_AIR) {
        //println!("dont refresh!");
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    }
    VarModule::set_flag(boma.object(), vars::common::instance::PREV_FLAG_DISABLE_ESCAPE_AIR, WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR));
}

pub unsafe fn suicide_throw_mashout(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_THROWN) {
        // add suicide throws here
        if !((boma.get_grabber_boma().kind() == *FIGHTER_KIND_KIRBY
            && [hash40("throw_f"), hash40("throw_b")].contains(&LinkModule::get_parent_motion_kind(boma, *LINK_NO_CAPTURE)))
        || (boma.get_grabber_boma().kind() == *FIGHTER_KIND_ROBOT
            && LinkModule::get_parent_motion_kind(boma, *LINK_NO_CAPTURE) == hash40("throw_hi")))
        {
            return;
        }
    
        if !VarModule::is_flag(boma.object(), vars::common::status::SUICIDE_THROW_CAN_CLATTER) {
            // allow mashing to begin
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
            VarModule::on_flag(boma.object(), vars::common::status::SUICIDE_THROW_CAN_CLATTER);
        }
        else {
            let ecb_bottom = *GroundModule::get_rhombus(boma.get_grabber_boma(), true).add(1);
            let line_bottom = Vector2f::new(ecb_bottom.x, ecb_bottom.y - 999.0);
            let mut stage_pos = Vector2f::zero();
            GroundModule::line_segment_check(boma.get_grabber_boma(), &Vector2f::new(ecb_bottom.x, ecb_bottom.y), &line_bottom, &Vector2f::zero(), &mut stage_pos, false);

            if GroundModule::get_correct(boma.get_grabber_boma()) == *GROUND_CORRECT_KIND_AIR
            && stage_pos == Vector2f::zero() {
                // can only mash out if offstage
                if ControlModule::get_clatter_time(boma, 0) <= 0.0 {
                    fighter.change_status(FIGHTER_STATUS_KIND_CAPTURE_JUMP.into(), false.into());
                }
            }
        }
    }
}

pub unsafe fn cliff_xlu_frame_counter(fighter: &mut L2CFighterCommon) {
    let cliff_xlu = VarModule::get_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME);
    // If you have ledge intan frames left
    if cliff_xlu > 0 {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            // Remove ledge intan on landing
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
            VarModule::set_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME, 0);
        }
        else{
            VarModule::dec_int(fighter.battle_object, vars::common::instance::CLIFF_XLU_FRAME);
        }
    }
}

pub unsafe fn ecb_shift_disabled_motions(fighter: &mut L2CFighterCommon) {
    if ( (fighter.kind() == *FIGHTER_KIND_KIRBY
            && fighter.is_motion(Hash40::new("throw_f")))
        || (fighter.kind() == *FIGHTER_KIND_GANON
            && fighter.is_motion(Hash40::new("attack_air_lw")))
        || (fighter.kind() == *FIGHTER_KIND_ROSETTA
            && fighter.is_motion(Hash40::new("attack_air_lw"))) )
    && !VarModule::is_flag(fighter.battle_object, vars::common::status::DISABLE_ECB_SHIFT)
    {
        VarModule::on_flag(fighter.battle_object, vars::common::status::DISABLE_ECB_SHIFT);
    }
}

pub unsafe fn taunt_parry_forgiveness(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_SPECIAL_N])
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    && fighter.global_table[CURRENT_FRAME].get_i32() <= 1
    && fighter.is_parry_input()
    {
        EffectModule::kill_all(fighter.module_accessor, *EFFECT_SUB_ATTRIBUTE_NONE as u32, true, false);
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
    }
}

#[smashline::fighter_frame_callback()]
pub fn decrease_knockdown_bounce_heights(fighter: &mut L2CFighterCommon) {
    unsafe {
        if smash::app::utility::get_category(&mut *fighter.module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            if fighter.is_status(*FIGHTER_STATUS_KIND_DOWN) {
                let mut hip_offset = Vector3f::zero();
                ModelModule::joint_global_offset_from_top(fighter.module_accessor, Hash40::new("hip"), &mut hip_offset);
                if fighter.motion_frame() <= 1.0 {
                    VarModule::set_float(fighter.battle_object, vars::common::status::RESTING_HIP_OFFSET_Y, hip_offset.y);
                }

                // Checks if our hip bone position is above our "resting" position (hip position when laying on the floor)
                // which determines whether we are bouncing or not
                let lower_limit = VarModule::get_float(fighter.battle_object, vars::common::status::RESTING_HIP_OFFSET_Y);
                if hip_offset.y > lower_limit {
                    // Halves hip bone's vertical movement and applies offset to rot bone
                    // Cannot apply offset to hip as it is already offset from rot, while rot is directly offset from top bone
                    let mut rot_translate = Vector3f::zero();
                    MotionModule::joint_local_tra(fighter.module_accessor, Hash40::new("rot"), false, &mut rot_translate);
                    let bounce_height_mul = 0.5;
                    let bounce_height = hip_offset.y - lower_limit;
                    
                    rot_translate.y += bounce_height * -bounce_height_mul;
                    ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("rot"), &Vector3f{ x: 0.0, y: rot_translate.y, z: 0.0 }, false, false);
                }
            }
        }
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    
    airdodge_refresh_on_hit_disable(boma, status_kind);
    suicide_throw_mashout(fighter, boma);
    cliff_xlu_frame_counter(fighter);
    ecb_shift_disabled_motions(fighter);
    taunt_parry_forgiveness(fighter);
}

