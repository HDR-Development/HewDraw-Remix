#![allow(non_snake_case)]
use smash::lib::L2CValue;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Hash40;

use smash::app::{sv_animcmd, lua_bind};
use smash::lib::lua_const::*;

pub trait ToF32 {
    fn to_f32(self) -> f32;
}

impl ToF32 for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for f32 {
    fn to_f32(self) -> f32 { self }
}

impl ToF32 for f64 {
    fn to_f32(self) -> f32 { self as f32 }
}

#[macro_export]
macro_rules! lua_args {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $(
            $agent.push_lua_stack(&mut $arg.into());
        )*
    };
}

#[inline]
pub unsafe fn ATTACK<A: ToF32, B: ToF32, C: ToF32, D: ToF32>(agent: &mut L2CAgentBase, id: u64, part: u64, bone: Hash40, damage: f32, angle: u64, kbg: D, fkb: i32, bkb: i32, size: C, x: f32, y: f32, z: f32,
                    x2: Option<f32>, y2: Option<f32>, z2: Option<f32>, hitlag: f32, sdi: f32, clang: i32, facing: i32, set_weight: bool, shield_damage: A, trip: f32, rehit: B, reflectable: bool,
                    absorbable: bool, flinchless: bool, disable_hitlag: bool, direct: bool, ground_air: i32, hitbits: i32, collision_part: i32, friendly_fire: bool, effect: Hash40, sfx_level: i32, collision_sound: i32, _type: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, id, part, bone, damage, angle, kbg.to_f32(), fkb, bkb, size.to_f32(), x, y, z);
    if let Some(x2) = x2 { lua_args!(agent, x2); } else { agent.push_lua_stack(&mut L2CValue::new()); }
    if let Some(y2) = y2 { lua_args!(agent, y2); } else { agent.push_lua_stack(&mut L2CValue::new()); }
    if let Some(z2) = z2 { lua_args!(agent, z2); } else { agent.push_lua_stack(&mut L2CValue::new()); }
    lua_args!(agent, hitlag, sdi, clang, facing, set_weight);
    let dmg = shield_damage.to_f32();
    if dmg.is_nan() { agent.push_lua_stack(&mut Hash40::new("no").into()); } else { agent.push_lua_stack(&mut dmg.into()); }
    lua_args!(agent, trip, rehit.to_f32(), reflectable, absorbable, flinchless, disable_hitlag, direct, ground_air, hitbits, collision_part, friendly_fire, effect, sfx_level, collision_sound, _type);
    sv_animcmd::ATTACK(agent.lua_state_agent);
}

#[inline]
pub unsafe fn ATTACK_IGNORE_THROW<A: ToF32, B: ToF32, C: ToF32>(agent: &mut L2CAgentBase, id: u64, part: u64, bone: Hash40, damage: f32, angle: u64, kbg: i32, fkb: i32, bkb: i32, size: C, x: f32, y: f32, z: f32,
                    x2: Option<f32>, y2: Option<f32>, z2: Option<f32>, hitlag: f32, sdi: f32, clang: i32, facing: i32, set_weight: bool, shield_damage: A, trip: f32, rehit: B, reflectable: bool,
                    absorbable: bool, flinchless: bool, disable_hitlag: bool, direct: bool, ground_air: i32, hitbits: i32, collision_part: i32, friendly_fire: bool, effect: Hash40, sfx_level: i32, collision_sound: i32, _type: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, id, part, bone, damage, angle, kbg, fkb, bkb, size.to_f32(), x, y, z);
    if let Some(x2) = x2 { lua_args!(agent, x2); } else { agent.push_lua_stack(&mut L2CValue::new()); }
    if let Some(y2) = y2 { lua_args!(agent, y2); } else { agent.push_lua_stack(&mut L2CValue::new()); }
    if let Some(z2) = z2 { lua_args!(agent, z2); } else { agent.push_lua_stack(&mut L2CValue::new()); }
    lua_args!(agent, hitlag, sdi, clang, facing, set_weight);
    let dmg = shield_damage.to_f32();
    if dmg.is_nan() { agent.push_lua_stack(&mut Hash40::new("no").into()); } else { agent.push_lua_stack(&mut dmg.into()); }
    lua_args!(agent, trip, rehit.to_f32(), reflectable, absorbable, flinchless, disable_hitlag, direct, ground_air, hitbits, collision_part, friendly_fire, effect, sfx_level, collision_sound, _type);
    sv_animcmd::ATTACK_IGNORE_THROW(agent.lua_state_agent);
}

#[inline]
pub unsafe fn ATK_POWER<F: ToF32>(agent: &mut L2CAgentBase, id: u64, power: F) {
    agent.clear_lua_stack();
    lua_args!(agent, id, power.to_f32());
    sv_animcmd::ATK_POWER(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn ATTACK_FP<A: ToF32, B: ToF32, C: ToF32, D: ToF32>(agent: &mut L2CAgentBase, id: u64, part: u64, bone: Hash40, damage: f32, angle: u64, kbg: D, fkb: i32, bkb: i32, size: C, x: f32, y: f32, z: f32,
                    effect: Hash40, trip: f32, hitlag: f32, sdi: f32, clang: bool, rebound: bool, shield_damage: A, sfx_level: i32, collision_sound: i32, ground_air: i32, direct: bool, _type: i32, hitbits: i32, unk1: bool,
                    collision_part: i32, unk2: bool, blockable: bool, reflectable: bool, absorbable: bool, rehit: B, ignore_invuln: bool, unk3: bool, facing: i32, unk4: bool, friendly_fire: bool, disable_hitlag: bool,
                    no_gfx: bool, flinchless: bool, collision_shape: i32) {
    agent.clear_lua_stack();
    lua_args!(
        agent, id, part, bone, damage, angle, kbg.to_f32(), fkb, bkb, size.to_f32(), x, y, z, effect, trip, hitlag, sdi, clang, rebound, shield_damage.to_f32(), sfx_level, collision_sound, ground_air,
        direct, _type, hitbits, unk1, collision_part, unk2, blockable, reflectable, absorbable, rehit.to_f32(), ignore_invuln, unk3, facing, unk4, friendly_fire, disable_hitlag, no_gfx, flinchless, collision_shape
    );
    sv_animcmd::ATTACK_FP(agent.lua_state_agent);
}

#[inline]
pub unsafe fn ATTACK_ABS(agent: &mut L2CAgentBase, kind: i32, id: u64, damage: f32, angle: u64, kbg: i32, fkb: i32, bkb: i32, hitlag: f32,
                        unk: f32, facing: i32, unk2: f32, unk3: bool, effect: Hash40, sfx_level: i32, sfx_type: i32, _type: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, kind, id, damage, angle, kbg, fkb, bkb, hitlag, unk, facing, unk2, unk3, effect, sfx_level, sfx_type, _type);
    sv_animcmd::ATTACK_ABS(agent.lua_state_agent);
}

#[inline]
pub unsafe fn ATK_HIT_ABS(agent: &mut L2CAgentBase, kind: i32, unk: Hash40, target: u64, target_group: u64, target_no: u64) {
    agent.clear_lua_stack();
    lua_args!(agent, kind, unk, target, target_group, target_no);
    sv_animcmd::ATK_HIT_ABS(agent.lua_state_agent);
}

#[inline]
pub unsafe fn is_excute(agent: &mut L2CAgentBase) -> bool {
    agent.clear_lua_stack();
    sv_animcmd::is_excute(agent.lua_state_agent);
    let ret = agent.pop_lua_stack(1).get_bool();
    ret
}

#[inline]
pub unsafe fn IS_EXIST_ARTICLE(agent: &mut L2CAgentBase, article: i32) -> bool {
    agent.clear_lua_stack();
    lua_args!(agent, article);
    sv_animcmd::IS_EXIST_ARTICLE(agent.lua_state_agent)
}

#[inline]
pub unsafe fn CATCH(agent: &mut L2CAgentBase, id: i32, bone: Hash40, size: f32, x: f32, y: f32, z: f32, x2: Option<f32>, y2: Option<f32>, z2: Option<f32>, status: i32, situation: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, id, bone, size, x, y, z);
    if let Some(x2) = x2 { lua_args!(agent, x2); } else { agent.push_lua_stack(&mut L2CValue::new()); }
    if let Some(y2) = y2 { lua_args!(agent, y2); } else { agent.push_lua_stack(&mut L2CValue::new()); }
    if let Some(z2) = z2 { lua_args!(agent, z2); } else { agent.push_lua_stack(&mut L2CValue::new()); }
    lua_args!(agent, status, situation);
    sv_animcmd::CATCH(agent.lua_state_agent);
}

#[inline]
pub unsafe fn FT_CATCH_STOP<A: ToF32, B: ToF32>(agent: &mut L2CAgentBase, unk1: A, unk2: B) {
    agent.clear_lua_stack();
    lua_args!(agent, unk1.to_f32(), unk2.to_f32());
    sv_animcmd::FT_CATCH_STOP(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_DESIRED_RATE(agent: &mut L2CAgentBase, motion_frames: f32, game_frames: f32) {
    agent.clear_lua_stack();
    lua_args!(agent, (game_frames / motion_frames));
    sv_animcmd::FT_MOTION_RATE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_MOTION_RATE_RANGE(agent: &mut L2CAgentBase, motion_start_frame: f32, motion_end_frame: f32, game_frames: f32) {
    agent.clear_lua_stack();
    lua_args!(agent, (game_frames / (motion_end_frame - motion_start_frame)));
    sv_animcmd::FT_MOTION_RATE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_MOTION_RATE<F: ToF32>(agent: &mut L2CAgentBase, rate: F) {
    agent.clear_lua_stack();
    lua_args!(agent, rate.to_f32());
    sv_animcmd::FT_MOTION_RATE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_SHOOTING_ATTACK_GROUND_CHECK_NEW<A: ToF32, B: ToF32, C: ToF32>(agent: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C) {
    agent.clear_lua_stack();
    lua_args!(agent, unk1.to_f32(), unk2.to_f32(), unk3.to_f32());
    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5<A: ToF32, B: ToF32, C: ToF32>(agent: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C, unk4: Hash40, unk5: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, unk1.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4, unk5);
    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_START_CUTIN(agent: &mut L2CAgentBase) {
    agent.clear_lua_stack();
    sv_animcmd::FT_START_CUTIN(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn START_INFO_FLASH_EYE(agent: &mut L2CAgentBase) {
    agent.clear_lua_stack();
    sv_animcmd::START_INFO_FLASH_EYE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_LEAVE_NEAR_OTTOTTO<A: ToF32, B: ToF32>(agent: &mut L2CAgentBase, unk1: A, unk2: B) {
    agent.clear_lua_stack();
    lua_args!(agent, unk1.to_f32(), unk2.to_f32());
    sv_animcmd::FT_LEAVE_NEAR_OTTOTTO(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_START_ADJUST_MOTION_FRAME_arg1(agent: &mut L2CAgentBase, arg: f32) {
    agent.clear_lua_stack();
    lua_args!(agent, arg);
    sv_animcmd::FT_START_ADJUST_MOTION_FRAME_arg1(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT<F: ToF32>(agent: &mut L2CAgentBase, offset: F) {
    agent.clear_lua_stack();
    lua_args!(agent, offset.to_f32());
    sv_animcmd::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn CHECK_VALID_START_CAMERA<A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32>(agent: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C, unk4: D, unk5: E, unk6: F, unk7: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, unk1.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7);
    sv_animcmd::CHECK_VALID_START_CAMERA(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn CHECK_VALID_FINAL_START_CAMERA<A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32>(agent: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C, unk4: D, unk5: E, unk6: F) {
    agent.clear_lua_stack();
    lua_args!(agent, unk1.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32());
    sv_animcmd::CHECK_VALID_FINAL_START_CAMERA(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn REQ_MOTION_CAMERA(agent: &mut L2CAgentBase, camera: Hash40, unk: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, camera, unk);
    sv_animcmd::REQ_MOTION_CAMERA(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn REQ_FINAL_START_CAMERA(agent: &mut L2CAgentBase, camera: Hash40, unk: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, camera, unk);
    sv_animcmd::REQ_FINAL_START_CAMERA(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn REQ_FINAL_START_CAMERA_arg3(agent: &mut L2CAgentBase, camera: Hash40, unk: bool, unk2: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, camera, unk, unk2);
    sv_animcmd::REQ_FINAL_START_CAMERA_arg3(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn IS_GENERATABLE_ARTICLE(agent: &mut L2CAgentBase, article: i32) -> bool {
    agent.clear_lua_stack();
    lua_args!(agent, article);
    let ret = sv_animcmd::IS_GENERATABLE_ARTICLE(agent.lua_state_agent);
    agent.clear_lua_stack();
    ret
}

#[inline]
pub unsafe fn CAM_ZOOM_IN_arg5(agent: &mut L2CAgentBase, zoom_amount: f32, arg2: f32, arg3: f32, y_rot: f32, x_rot: f32) {
    agent.clear_lua_stack();
    lua_args!(agent, zoom_amount, arg2, arg3, y_rot, x_rot);
    sv_animcmd::CAM_ZOOM_IN_arg5(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn CAM_ZOOM_IN_arg6(agent: &mut L2CAgentBase, arg1: f32, arg2: f32, arg3: f32, arg4: f32, arg5: f32, arg6: f32) {
    agent.clear_lua_stack();
    lua_args!(agent, arg1, arg2, arg3, arg4, arg5, arg6);
    sv_animcmd::CAM_ZOOM_IN_arg6(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn HIT_NO(agent: &mut L2CAgentBase, num: u64, status: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, num, status);
    sv_animcmd::HIT_NO(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn HIT_NODE(agent: &mut L2CAgentBase, bone: Hash40, status: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, bone, status);
    sv_animcmd::HIT_NODE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn HIT_RESET_ALL(agent: &mut L2CAgentBase) {
    agent.clear_lua_stack();
    sv_animcmd::HIT_RESET_ALL(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn ATK_SET_SHIELD_SETOFF_MUL_arg3 <A: ToF32> (agent: &mut L2CAgentBase, unk: u64, unk2: A, unk3: f32) {
    agent.clear_lua_stack();
    lua_args!(agent, unk, unk2.to_f32(), unk3);
    sv_animcmd::ATK_SET_SHIELD_SETOFF_MUL_arg3(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn ATK_LERP_RATIO <A: ToF32> (agent: &mut L2CAgentBase, ratio: A) {
    agent.clear_lua_stack();
    lua_args!(agent, ratio.to_f32());
    sv_animcmd::ATK_LERP_RATIO(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn QUAKE(agent: &mut L2CAgentBase, kind: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, kind);
    sv_animcmd::QUAKE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_ATTACK_ABS_CAMERA_QUAKE(agent: &mut L2CAgentBase, attack_abs_kind: i32, quake_kind: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, attack_abs_kind, quake_kind);
    sv_animcmd::FT_ATTACK_ABS_CAMERA_QUAKE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn SET_SPEED_EX<A: ToF32, B: ToF32>(agent: &mut L2CAgentBase, speed_x: A, speed_y: B, kinetic_kind: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, speed_x.to_f32(), speed_y.to_f32(), kinetic_kind);
    sv_animcmd::SET_SPEED_EX(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn SLOW_OPPONENT(agent: &mut L2CAgentBase, slow_mul: f32, slow_time: f32) {
    agent.clear_lua_stack();
    lua_args!(agent, slow_mul, slow_time);
    sv_animcmd::SLOW_OPPONENT(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_SET_FINAL_FEAR_FACE(agent: &mut L2CAgentBase, unk: u64) {
    agent.clear_lua_stack();
    lua_args!(agent, unk);
    sv_animcmd::FT_SET_FINAL_FEAR_FACE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn CAM_ZOOM_OUT(agent: &mut L2CAgentBase) {
    agent.clear_lua_stack();
    sv_animcmd::CAM_ZOOM_OUT(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn CAM_ZOOM_OUT_FINAL(agent: &mut L2CAgentBase) {
    agent.clear_lua_stack();
    sv_animcmd::CAM_ZOOM_OUT_FINAL(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn CAM_ZOOM_IN_FINAL_arg13(agent: &mut L2CAgentBase, x: f32, y: f32, z: f32, unk1: i32, unk2: i32, unk3: i32, unk4: i32, unk5: i32, unk6: bool, object_id: u32, unk7: i32, unk8: i32, unk9: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, x, y, z, unk1, unk2, unk3, unk4, unk5, unk6, object_id, unk7 ,unk8, unk9);
    sv_animcmd::CAM_ZOOM_OUT_FINAL(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn ATK_SET_SHIELD_SETOFF_MUL<A: ToF32>(agent: &mut L2CAgentBase, id: u64, val: A) {
    agent.clear_lua_stack();
    lua_args!(agent, id, val.to_f32());
    sv_animcmd::ATK_SET_SHIELD_SETOFF_MUL(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn ATK_SET_SHIELD_SETOFF_MUL2<A: ToF32>(agent: &mut L2CAgentBase, id: u64, val: A) {
    agent.clear_lua_stack();
    lua_args!(agent, id, val.to_f32());
    sv_animcmd::ATK_SET_SHIELD_SETOFF_MUL2(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn ATK_SET_SHIELD_SETOFF_MUL_arg5(agent: &mut L2CAgentBase, unk: u64, unk2: u64, unk3: u64, unk4: u64, unk5: f32) {
    agent.clear_lua_stack();
    lua_args!(agent, unk, unk2, unk3, unk4, unk5);
    sv_animcmd::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn ATK_SET_SHIELD_SETOFF_MUL_arg4(agent: &mut L2CAgentBase, unk: u64, unk2: u64, unk3: u64, unk4: f32) {
    agent.clear_lua_stack();
    lua_args!(agent, unk, unk2, unk3, unk4);
    sv_animcmd::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn WHOLE_HIT(agent: &mut L2CAgentBase, hit_status: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, hit_status);
    sv_animcmd::WHOLE_HIT(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FLASH<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32
    >(agent: &mut L2CAgentBase, unk: A, unk2: B, unk3: C, unk4: D) {
    agent.clear_lua_stack();
    lua_args!(agent, unk.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32());
    sv_animcmd::FLASH(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FLASH_FRM<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32
    >(agent: &mut L2CAgentBase, frame: A, unk: B, unk2: C, unk3: D, unk4: E) {
    agent.clear_lua_stack();
    lua_args!(agent, frame.to_f32(), unk.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32());
    sv_animcmd::FLASH_FRM(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32
    >(agent: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8);
    sv_animcmd::EFFECT_FOLLOW(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_WORK<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32
    >(agent: &mut L2CAgentBase, effect_const: i32, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: bool) {
    let effect = lua_bind::WorkModule::get_int64(agent.module_accessor, effect_const);
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8);
    sv_animcmd::EFFECT_FOLLOW_WORK(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_RND_WORK<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32
    >(agent: &mut L2CAgentBase, effect_const: i32, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, x_pos_range: H, y_pos_range: I,
    z_pos_range: J, x_rot_range: K, y_rot_range: L, z_rot_range: M, unk14: bool) {
    let effect = lua_bind::WorkModule::get_int64(agent.module_accessor, effect_const);
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), x_pos_range.to_f32(), y_pos_range.to_f32(), z_pos_range.to_f32(), x_rot_range.to_f32(), y_rot_range.to_f32(), z_rot_range.to_f32(), unk14);
    sv_animcmd::EFFECT_FOLLOW_RND_WORK(agent.lua_state_agent);
    agent.clear_lua_stack();
}

pub unsafe fn EFFECT_FOLLOW_arg11<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32
    >(agent: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: bool, unk9: i32) {
    let color = lua_bind::WorkModule::get_int64(agent.module_accessor, unk9);
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8, color);
    sv_animcmd::EFFECT_FOLLOW_arg11(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FLW_POS<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32
    >(agent: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8);
    sv_animcmd::EFFECT_FLW_POS(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FLW_UNSYNC_VIS<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32
    >(agent: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8);
    sv_animcmd::EFFECT_FLW_UNSYNC_VIS(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn LANDING_EFFECT<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32
    >(agent: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14);
    sv_animcmd::LANDING_EFFECT(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn LANDING_EFFECT_FLIP<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32
    >(agent: &mut L2CAgentBase, left_effect: Hash40, right_effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool, axis: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, left_effect, right_effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14, axis);
    sv_animcmd::LANDING_EFFECT_FLIP(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn LAST_EFFECT_SET_ALPHA<F: ToF32>(agent: &mut L2CAgentBase, alpha: F) {
    agent.clear_lua_stack();
    lua_args!(agent, alpha.to_f32());
    sv_animcmd::LAST_EFFECT_SET_ALPHA(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn LAST_EFFECT_SET_SCALE_W<A: ToF32, B: ToF32, C: ToF32>(agent: &mut L2CAgentBase, x: A, y: B, z: C) {
    agent.clear_lua_stack();
    lua_args!(agent, x.to_f32(), y.to_f32(), z.to_f32());
    sv_animcmd::LAST_EFFECT_SET_SCALE_W(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FOOT_EFFECT<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32
    >(agent: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14);
    sv_animcmd::FOOT_EFFECT(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32
    >(agent: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14);
    sv_animcmd::EFFECT(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_WORK<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32
    >(agent: &mut L2CAgentBase, effect_const: i32, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool) {
    let effect = lua_bind::WorkModule::get_int64(agent.module_accessor, effect_const);
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14);
    sv_animcmd::EFFECT_WORK(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]	
pub unsafe fn EFFECT_FLIP<	
    A: ToF32,	
    B: ToF32,	
    C: ToF32,	
    D: ToF32,	
    E: ToF32,	
    F: ToF32,	
    G: ToF32,	
    H: ToF32,	
    I: ToF32,	
    J: ToF32,	
    K: ToF32,	
    L: ToF32,	
    M: ToF32	
    >(agent: &mut L2CAgentBase, unk1: Hash40, unk2: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk10: H, unk11: I, unk12: J, unk13: K, unk14: L, unk15: M, unk16: bool, axis: i32) {	
    agent.clear_lua_stack();	
    lua_args!(agent, unk1, unk2, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14.to_f32(), unk15.to_f32(), unk16, axis);	
    sv_animcmd::EFFECT_FLIP(agent.lua_state_agent);	
    agent.clear_lua_stack();	
}

#[inline]
pub unsafe fn EFFECT_ALPHA<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32,
    N: ToF32
    >(agent: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool, alpha: N) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14, alpha.to_f32());
    sv_animcmd::EFFECT(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FLIP_ALPHA<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32,
    N: ToF32
    >(agent: &mut L2CAgentBase, left_effect: Hash40, right_effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool, axis: i32, alpha: N) {
    agent.clear_lua_stack();
    lua_args!(agent, left_effect, right_effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14, axis, alpha.to_f32());
    sv_animcmd::EFFECT_FLIP_ALPHA(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_ALPHA<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    >(agent: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: bool, alpha: H) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8, alpha.to_f32());
    sv_animcmd::EFFECT_FOLLOW_ALPHA(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_FLIP<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32
    >(agent: &mut L2CAgentBase, right_effect: Hash40, left_effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk10: bool, axis: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, right_effect, left_effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk10, axis);
    sv_animcmd::EFFECT_FOLLOW_FLIP(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_FLIP_ALPHA<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32
    >(agent: &mut L2CAgentBase, left_effect: Hash40, right_effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk10: bool, 
    axis: i32, alpha: H) {
    agent.clear_lua_stack();
    lua_args!(agent, left_effect, right_effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(),
    unk10, axis, alpha.to_f32());
    sv_animcmd::EFFECT_FOLLOW_FLIP_ALPHA(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn ENABLE_AREA(agent: &mut L2CAgentBase, kind: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, kind);
    sv_animcmd::ENABLE_AREA(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn UNABLE_AREA(agent: &mut L2CAgentBase, kind: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, kind);
    sv_animcmd::UNABLE_AREA(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn SET_SEARCH_SIZE_EXIST<A: ToF32>(agent: &mut L2CAgentBase, id: u64, size: A) {
    agent.clear_lua_stack();
    lua_args!(agent, id, size.to_f32());
    sv_animcmd::SET_SEARCH_SIZE_EXIST(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn LAST_PARTICLE_SET_COLOR<
    A: ToF32,
    B: ToF32,
    C: ToF32
>(agent: &mut L2CAgentBase, unk: A, unk2: B, unk3: C) {
    agent.clear_lua_stack();
    lua_args!(agent, unk.to_f32(), unk2.to_f32(), unk3.to_f32());
    sv_animcmd::LAST_PARTICLE_SET_COLOR(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn LAST_EFFECT_SET_COLOR<
    A: ToF32,
    B: ToF32,
    C: ToF32
>(agent: &mut L2CAgentBase, unk: A, unk2: B, unk3: C) {
    agent.clear_lua_stack();
    lua_args!(agent, unk.to_f32(), unk2.to_f32(), unk3.to_f32());
    sv_animcmd::LAST_EFFECT_SET_COLOR(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn BURN_COLOR<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32
>(agent: &mut L2CAgentBase, unk: A, unk2: B, unk3: C, unk4: D) {
    agent.clear_lua_stack();
    lua_args!(agent, unk.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32());
    sv_animcmd::BURN_COLOR(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn BURN_COLOR_FRAME<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32
>(agent: &mut L2CAgentBase, frame: A, unk: B, unk2: C, unk3: D, unk4: E) {
    agent.clear_lua_stack();
    lua_args!(agent, frame.to_f32(), unk.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32());
    sv_animcmd::BURN_COLOR_FRAME(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn BURN_COLOR_NORMAL(agent: &mut L2CAgentBase) {
    agent.clear_lua_stack();
    sv_animcmd::BURN_COLOR_NORMAL(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn LAST_EFFECT_SET_RATE<F: ToF32>(agent: &mut L2CAgentBase, rate: F) {
    agent.clear_lua_stack();
    lua_args!(agent, rate.to_f32());
    sv_animcmd::LAST_EFFECT_SET_RATE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_OFF_KIND(agent: &mut L2CAgentBase, effect: Hash40, unk: bool, unk2: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, unk, unk2);
    sv_animcmd::EFFECT_OFF_KIND(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_OFF_KIND_WORK(agent: &mut L2CAgentBase, effect_const: i32, unk: bool, unk2: bool) {
    let effect = lua_bind::WorkModule::get_int64(agent.module_accessor, effect_const);
    agent.clear_lua_stack();
    lua_args!(agent, effect, unk, unk2);
    sv_animcmd::EFFECT_OFF_KIND_WORK(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn COL_NORMAL(agent: &mut L2CAgentBase) {
    agent.clear_lua_stack();
    sv_animcmd::COL_NORMAL(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FILL_SCREEN_MODEL_COLOR<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    >(agent: &mut L2CAgentBase, unk1: i32, unk2: A, unk3: B, unk4: C, unk5: D, unk6: E, unk7: F, unk8: G, unk9: H, unk10: I, effect_screen_layer: i32, unk11: J) {
    agent.clear_lua_stack();
    lua_args!(agent, unk1, unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), effect_screen_layer, unk11.to_f32());
    sv_animcmd::FILL_SCREEN_MODEL_COLOR(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn CANCEL_FILL_SCREEN<
    A: ToF32,
>(agent: &mut L2CAgentBase, unk1: i32, unk2: A) {
    agent.clear_lua_stack();
    lua_args!(agent, unk1, unk2.to_f32());
    sv_animcmd::CANCEL_FILL_SCREEN(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn SA_SET(agent: &mut L2CAgentBase, unk: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, unk);
    sv_animcmd::SA_SET(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn CHECK_FINISH_CAMERA<A: ToF32, B: ToF32>(agent: &mut L2CAgentBase, unk: A, unk2: B) {
    agent.clear_lua_stack();
    lua_args!(agent, unk.to_f32(), unk2.to_f32());
    sv_animcmd::CHECK_FINISH_CAMERA(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_SE(agent: &mut L2CAgentBase, se: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, se);
    sv_animcmd::PLAY_SE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_STATUS(agent: &mut L2CAgentBase, se: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, se);
    sv_animcmd::PLAY_STATUS(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_LANDING_SE(agent: &mut L2CAgentBase, se: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, se);
    sv_animcmd::PLAY_LANDING_SE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_SE_NO_3D(agent: &mut L2CAgentBase, se: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, se);
    sv_animcmd::PLAY_SE_NO_3D(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_SE_REMAIN(agent: &mut L2CAgentBase, se: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, se);
    sv_animcmd::PLAY_SE_REMAIN(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_DOWN_SE(agent: &mut L2CAgentBase, se: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, se);
    sv_animcmd::PLAY_DOWN_SE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_FLY_VOICE(agent: &mut L2CAgentBase, se1: Hash40, se2: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, se1, se2);
    sv_animcmd::PLAY_FLY_VOICE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn STOP_SE(agent: &mut L2CAgentBase, se: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, se);
    sv_animcmd::STOP_SE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_STEP(agent: &mut L2CAgentBase, step: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, step);
    sv_animcmd::PLAY_STEP(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_STEP_FLIPPABLE(agent: &mut L2CAgentBase, step: Hash40, step2: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, step, step2);
    sv_animcmd::PLAY_STEP_FLIPPABLE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_SEQUENCE(agent: &mut L2CAgentBase, sequence: Hash40) {
    agent.clear_lua_stack();
    lua_args!(agent, sequence);
    sv_animcmd::PLAY_SEQUENCE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn SET_PLAY_INHIVIT<A: ToF32>(agent: &mut L2CAgentBase, se: Hash40, unk: A) {
    agent.clear_lua_stack();
    lua_args!(agent, se, unk.to_f32());
    sv_animcmd::SET_PLAY_INHIVIT(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn AFTER_IMAGE_OFF<F: ToF32>(agent: &mut L2CAgentBase, unk: F) {
    agent.clear_lua_stack();
    lua_args!(agent, unk.to_f32());
    sv_animcmd::AFTER_IMAGE_OFF(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn AFTER_IMAGE4_ON_arg29<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32
>(agent: &mut L2CAgentBase, trail1: Hash40, trail2: Hash40, trail_length: u64, trail_bone1: Hash40, trail_x1: A, trail_y1: B,
        trail_z1: C, trail_bone2: Hash40, trail_x2: D, trail_y2: E, trail_z2: F, unk10: bool, flare: Hash40, flare_bone: Hash40, flare_x: G, flare_y: H,
        flare_z: I, flare_x_rot: J, flare_y_rot: K, flare_z_rot: L, flare_size: M, unk13: u64, axis: i32, unk15: u64, trail_blend: i32,
        blend: u64, cull: i32, unk16: f32, unk17: f32) {
    agent.clear_lua_stack();
    lua_args!(agent, trail1, trail2, trail_length, trail_bone1, trail_x1.to_f32(), trail_y1.to_f32(), trail_z1.to_f32(), trail_bone2, trail_x2.to_f32(), trail_y2.to_f32(), trail_z2.to_f32());
    lua_args!(agent, unk10, flare, flare_bone, flare_x.to_f32(), flare_y.to_f32(), flare_z.to_f32(), flare_x_rot.to_f32(), flare_y_rot.to_f32(), flare_z_rot.to_f32(), flare_size.to_f32(), unk13, axis, unk15, trail_blend, blend, cull, unk16, unk17);
    sv_animcmd::AFTER_IMAGE4_ON_arg29(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn AFTER_IMAGE4_ON_WORK_arg29(agent: &mut L2CAgentBase, trail1: i32, trail2: i32, trail_length: u64, trail_bone1: Hash40, trail_x1: f32, trail_y1: f32,
        trail_z1: f32, trail_bone2: Hash40, trail_x2: f32, trail_y2: f32, trail_z2: f32, unk10: bool, flare: i32, flare_bone: Hash40, flare_x: f32, flare_y: f32,
        flare_z: f32, flare_x_rot: f32, flare_y_rot: f32, flare_z_rot: f32, flare_size: f32, unk13: u64, axis: i32, unk15: u64, trail_blend: i32,
        blend: u64, cull: i32, unk16: f32, unk17: f32) {
    agent.clear_lua_stack();
    lua_args!(agent, trail1, trail2, trail_length, trail_bone1, trail_x1, trail_y1, trail_z1, trail_bone2, trail_x2, trail_y2, trail_z2, unk10, flare, flare_bone, flare_x, flare_y, flare_z, flare_x_rot);
    lua_args!(agent, flare_y_rot, flare_z_rot, flare_size, unk13, axis, unk15, trail_blend, blend, cull, unk16, unk17);
    sv_animcmd::AFTER_IMAGE4_ON_WORK_arg29(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_NO_STOP<
    A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32 , G: ToF32
    >(agent: &mut L2CAgentBase, effect: Hash40, bone: Hash40, unk: A, unk2: B, unk3: C, unk4: D, unk5: E, unk6: F, unk7: G, unk8: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, unk.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7.to_f32(), unk8);
    sv_animcmd::EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_NO_STOP_FLIP<
    A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32, G: ToF32
    >(agent: &mut L2CAgentBase, effect: Hash40, bone: Hash40, unk: Hash40, unk2: A, unk3: B, unk4: C, unk5: D, unk6: E, unk7: F, unk8: G, unk9: bool, axis: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, unk, unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7.to_f32(), unk8.to_f32(), unk9, axis);
    sv_animcmd::EFFECT_FOLLOW_NO_STOP_FLIP(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FLW_POS_NO_STOP(agent: &mut L2CAgentBase, effect: Hash40, bone: Hash40, unk: u64, unk2: u64, unk3: u64, unk4: u64, unk5: u64, unk6: u64, unk7: u64, unk8: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, bone, unk, unk2, unk3, unk4, unk5, unk6, unk7, unk8);
    sv_animcmd::EFFECT_FLW_POS_NO_STOP(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn COL_PRI(agent: &mut L2CAgentBase, pri: u64) {
    agent.clear_lua_stack();
    lua_args!(agent, pri);
    sv_animcmd::COL_PRI(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn AREA_WIND_2ND_RAD<
    A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32, G: ToF32, H: ToF32
    >(agent: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C, unk4: D, unk5: E, unk6: F, unk7: G, unk8: H) {
    agent.clear_lua_stack();
    lua_args!(agent, unk1.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7.to_f32(), unk8.to_f32());
    sv_animcmd::AREA_WIND_2ND_RAD(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn AREA_WIND_2ND_RAD_arg9<
    A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32, G: ToF32, H: ToF32, I: ToF32
    >(agent: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C, unk4: D, unk5: E, unk6: F, unk7: G, unk8: H, unk9: I) {
    agent.clear_lua_stack();
    lua_args!(agent, unk1.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7.to_f32(), unk8.to_f32(), unk9.to_f32());
    sv_animcmd::AREA_WIND_2ND_RAD_arg9(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn AREA_WIND_2ND_arg10<
    A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32, G: ToF32, H: ToF32, I: ToF32, J: ToF32
    >(agent: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C, unk4: D, unk5: E, unk6: F, unk7: G, unk8: H, unk9: I, unk10: J) {
    agent.clear_lua_stack();
    lua_args!(agent, unk1.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32());
    sv_animcmd::AREA_WIND_2ND_arg10(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_ADD_DAMAGE<F: ToF32>(agent: &mut L2CAgentBase, damage: F) {
    agent.clear_lua_stack();
    lua_args!(agent, damage.to_f32());
    sv_animcmd::FT_ADD_DAMAGE(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn CORRECT(agent: &mut L2CAgentBase, kind: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, kind);
    sv_animcmd::CORRECT(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn RUMBLE_HIT(agent: &mut L2CAgentBase, kind: Hash40, unk: u64) {
    agent.clear_lua_stack();
    lua_args!(agent, kind, unk);
    sv_animcmd::RUMBLE_HIT(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_DETACH_KIND(agent: &mut L2CAgentBase, effect: Hash40, unk: i64) {
    agent.clear_lua_stack();
    lua_args!(agent, effect, unk);
    sv_animcmd::EFFECT_DETACH_KIND(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_DETACH_KIND_WORK(agent: &mut L2CAgentBase, effect_const: i32, unk: i64) {
    let effect = lua_bind::WorkModule::get_int64(agent.module_accessor, effect_const);
    agent.clear_lua_stack();
    lua_args!(agent, effect, unk);
    sv_animcmd::EFFECT_DETACH_KIND_WORK(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn REVERSE_LR(agent: &mut L2CAgentBase) {
    agent.clear_lua_stack();
    sv_animcmd::REVERSE_LR(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn SEARCH(agent: &mut L2CAgentBase, id: u64, part: u64, bone: Hash40, size: f32, x: f32, y: f32, z: f32, x2: Option<f32>, y2: Option<f32>, z2: Option<f32>, 
    collision: i32, hit_status: i32, unk: u64, ground_air: i32, collision_category: i32, collision_parts: i32, unk2: bool) {
    agent.clear_lua_stack();
    lua_args!(agent, id, part, bone, size, x, y, z);
    if let Some(x2) = x2 { lua_args!(agent, x2); } else { agent.push_lua_stack(&mut L2CValue::new()); }
    if let Some(y2) = y2 { lua_args!(agent, y2); } else { agent.push_lua_stack(&mut L2CValue::new()); }
    if let Some(z2) = z2 { lua_args!(agent, z2); } else { agent.push_lua_stack(&mut L2CValue::new()); }
    lua_args!(agent, collision, hit_status, unk, ground_air, collision_category, collision_parts, unk2);
    sv_animcmd::SEARCH(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn ADD_SPEED_NO_LIMIT<X: ToF32, Y: ToF32>(agent: &mut L2CAgentBase, x_speed: X, y_speed: Y) {
    agent.clear_lua_stack();
    lua_args!(agent, x_speed.to_f32(), y_speed.to_f32());
    sv_animcmd::ADD_SPEED_NO_LIMIT(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn THROW_ITEM_arg3(agent: &mut L2CAgentBase, angle: i32, speed: i32, power: i32) {
    agent.clear_lua_stack();
    lua_args!(agent, angle, speed, power);
    sv_animcmd::THROW_ITEM_arg3(agent.lua_state_agent);
    agent.clear_lua_stack();
}

#[inline]
pub unsafe fn game_CaptureCutCommon(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        sv_animcmd::ATTACK_ABS(agent.lua_state_agent);
        agent.clear_lua_stack();
    }
}

#[inline]
pub unsafe fn wait_loop_clear(agent: &mut L2CAgentBase) {
    agent.clear_lua_stack();
    sv_animcmd::wait_loop_clear(agent.lua_state_agent);
    agent.pop_lua_stack(1);
}

#[macro_export]
macro_rules! grab {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_module_access::grab($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! item {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_module_access::item($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! shield {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_module_access::shield($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! search {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_module_access::search($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! slope {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_module_access::slope($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! damage {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_module_access::damage($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! physics {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_module_access::physics($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! camera {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_module_access::camera($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! capture {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_module_access::capture($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! sv_kinetic_energy {
    ($cmd_name:ident, $agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_kinetic_energy::$cmd_name($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! attack {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_module_access::attack($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}


#[macro_export]
macro_rules! effect {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_module_access::effect($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! sound {
    ($agent:ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_module_access::sound($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! notify_event_msc_cmd {
    ($agent: ident, $($arg:expr),* $(,)?) => {
        $agent.clear_lua_stack();
        lua_args!($agent, $($arg),*);
        smash::app::sv_battle_object::notify_event_msc_cmd($agent.lua_state_agent);
        $agent.pop_lua_stack(1)
    }
}
