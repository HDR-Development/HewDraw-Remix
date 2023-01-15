// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn airdodge_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if frame > 17.0 {
            FighterStatusModuleImpl::set_fighter_status_data(
                boma,
                false,
                *FIGHTER_TREADED_KIND_NO_REAC,
                false,
                false,
                false,
                (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
                0,
                *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
                0
            );
            boma.check_airdodge_cancel();
        }
    }
}

// Wolf Shine Jump Cancels
unsafe fn shine_jump_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    if ((fighter.is_status (*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.motion_frame() > 7.0)  // Allows for jump cancel on frame 5 in game
        || fighter.is_status_one_of(&[
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT,
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP,
            *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END]))
        && !fighter.is_in_hitlag()
        {
            fighter.check_jump_cancel(false);
        }
}

#[derive(Debug)]
struct KnockbackCalcContext {
    pub knockback: f32,
    pub x_launch_speed: f32,
    pub y_launch_speed: f32,
    pub y_chara_speed: f32,
    pub tumble: bool,
    pub is_damage_fly_top: bool,
    pub hitstun: f32,
    pub gravity: f32,
    pub damageflytop_gravity: f32,
    pub fall_speed: f32,
    pub damageflytop_fall_speed: f32,
    pub x_pos: f32,
    pub y_pos: f32,
    pub decay_x: f32,
    pub decay_y: f32,
}

impl KnockbackCalcContext {
    pub fn step(&mut self) {
        self.x_pos += self.x_launch_speed;
        self.y_pos += self.y_launch_speed + self.y_chara_speed;
        if self.x_launch_speed != 0.0 {
            let dir = self.x_launch_speed.signum();
            self.x_launch_speed = self.x_launch_speed.abs() - self.decay_x;
            if self.x_launch_speed < 0.0 {
                self.x_launch_speed = 0.0;
            } else {
                self.x_launch_speed *= dir;
            }
        }
        if self.y_launch_speed != 0.0 {
            let dir = self.y_launch_speed.signum();
            self.y_launch_speed = self.y_launch_speed.abs() - self.decay_y;
            if self.y_launch_speed < 0.0 {
                self.y_launch_speed = 0.0;
            } else {
                self.y_launch_speed *= dir;
            }
        }
        if self.is_damage_fly_top {
            self.y_chara_speed -= self.damageflytop_gravity;
            if self.y_chara_speed < -self.damageflytop_fall_speed {
                self.y_chara_speed = -self.damageflytop_fall_speed;
            }
        } else {
            self.y_chara_speed -= self.gravity;
            if self.y_chara_speed < -self.fall_speed {
                self.y_chara_speed = -self.fall_speed;
            }
        }
    }
}

#[repr(simd)]
#[derive(Debug)]
struct Rect {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl Rect {
    fn contains(&self, x: f32, y: f32) -> bool {
        (self.left <= x && x <= self.right) && (self.bottom <= y && y <= self.top)
    }
}

extern "C" {
    #[link_name = "_ZN3app6camera13get_dead_areaEv"]
    fn get_dead_area() -> Rect;

    #[link_name = "_ZN3app10sv_animcmd25EFFECT_GLOBAL_BACK_GROUNDEP9lua_State"]
    fn EFFECT_GLOBAL_BACK_GROUND(lua_state: u64);
}

/// Knockback log
/// 0x8a -> the opponent was grounded (bool)
/// 0x90 -> backslash (bool)
/// 0x60 -> stop delay (f32) 
/// 0x50 -> collision attr (Hash40)
/// 0x40 -> launch angle in rad (f32)
/// 0x4 -> level (?)
/// 0x4c -> hitstop frame

const HANDLE: i32 = 0x01FF;
const COUNTER: i32 = 0x01FE;

#[no_mangle]
pub unsafe extern "C" fn calculate_finishing_hit(defender: u32, attacker: u32, knockback_info: *const f32) {
    *(knockback_info.add(0x4c / 4) as *mut u32) = 60;
    let defender_boma = &mut *(*utils::util::get_battle_object_from_id(defender)).module_accessor;
    let attacker_boma = &mut *(*utils::util::get_battle_object_from_id(attacker)).module_accessor;
    if !defender_boma.is_fighter() && !defender_boma.is_weapon() { return; }
    if !attacker_boma.is_fighter() && !attacker_boma.is_weapon() { return; }

    let knockback = *knockback_info;
    let initial_speed_x = *knockback_info.add(4);
    let initial_speed_y = *knockback_info.add(5);
    let initial_pos_x = *knockback_info.add(8);
    let initial_pos_y = *knockback_info.add(9);
    let reaction = *knockback_info.add(0x48 / 4);
    let mut y_speed = initial_speed_y;
    let mut y_pos = PostureModule::pos_y(defender_boma);
    let angle = *knockback_info.add(0x10);
    let top_lw = defender_boma.get_param_float("battle_object", "fly_top_angle_lw");
    let top_hi = defender_boma.get_param_float("battle_object", "fly_top_angle_hi");

    let mut context = KnockbackCalcContext {
        knockback,
        x_launch_speed: initial_speed_x,
        y_launch_speed: initial_speed_y,
        y_chara_speed: 0.0,
        tumble: *(knockback_info.add(1) as *const u32) >= 3,
        is_damage_fly_top: top_lw <= angle && angle <= top_hi,
        hitstun: reaction,
        gravity: defender_boma.get_param_float("air_accel_y", ""),
        damageflytop_gravity: defender_boma.get_param_float("damage_fly_top_air_accel_y", ""),
        fall_speed: defender_boma.get_param_float("air_speed_y_stable", ""),
        damageflytop_fall_speed: defender_boma.get_param_float("damage_fly_top_speed_y_stable", ""),
        x_pos: PostureModule::pos_x(defender_boma),
        y_pos: PostureModule::pos_y(defender_boma),
        decay_x: defender_boma.get_param_float("common", "damage_air_brake") * angle.cos().abs(),
        decay_y: defender_boma.get_param_float("common", "damage_air_brake") * angle.sin().abs(),
    };
    println!("{:#x}: {:?}", defender, context);

    let blastzones = get_dead_area();
    let mut is_kill = false;

    let mut x = 0;
    while context.hitstun > x as f32  {
        context.step();
        if !blastzones.contains(context.x_pos, context.y_pos) {
            is_kill = true;
            break;
        }
        x += 1;
    }

    
    // let handle = EffectModule::req(attacker_boma, Hash40::new("sys_ripple"), &Vector3f::new(-50.0, 20.0, 0.0), &Vector3f::new(-90.0f32.to_radians(), 0.0, 0.0), 1.0, 0, 0, false, 0);
    // EffectModule::set_disable_render_offset_last(attacker_boma);
    // EffectModule::set_scale_last(attacker_boma, &Vector3f::new(4.0, 0.1, 4.0));
    // //EffectModule::set_frame_last(boma, handle as u32, 8.0);
    // EffectModule::set_billboard(attacker_boma, handle as u32, true);
    // //LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    // EffectModule::set_rate_last(attacker_boma, 1.0);

    
    // let handle = EffectModule::req(attacker_boma, Hash40::new("mys_ripple"), &Vector3f::new(50.0, 20.0, 0.0), &Vector3f::new(-90.0f32.to_radians(), 0.0, 0.0), 1.0, 0, 0, false, 0);
    // EffectModule::set_disable_render_offset_last(attacker_boma);
    // EffectModule::set_scale_last(attacker_boma, &Vector3f::new(0.0, 1.0, 0.0));
    // //EffectModule::set_frame_last(boma, handle as u32, 8.0);
    // EffectModule::set_billboard(attacker_boma, handle as u32, true);
    // //LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    // EffectModule::set_rate_last(attacker_boma, 1.0);

    if is_kill {
        let mut last_pos = VarModule::get_vec3(attacker_boma.object(), vars::common::instance::LAST_ATTACK_HIT_LOCATION);
        last_pos.x += 10.0;
        let handle = EffectModule::req(attacker_boma, Hash40::new("mys_ripple"), &last_pos, &Vector3f::new(-90.0f32.to_radians(), 0.0, 0.0), 1.0, 0, 0, false, 0);
        EffectModule::set_disable_render_offset_last(attacker_boma);
        EffectModule::set_scale_last(attacker_boma, &Vector3f::new(0.0, 1.0, 0.0));
        //EffectModule::set_frame_last(boma, handle as u32, 8.0);
        EffectModule::set_billboard(attacker_boma, handle as u32, true);
        //LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EffectModule::set_rate_last(attacker_boma, 1.0);
        VarModule::set_int(attacker_boma.object(), HANDLE, handle as i32);
        VarModule::set_int(attacker_boma.object(), COUNTER, 30);
        let common = utils::util::get_fighter_common_from_accessor(attacker_boma);
        PLAY_SE_REMAIN(common, Hash40::new_raw(0x152af449c2));
        let black_handle = EffectModule::req(attacker_boma, Hash40::new("sys_genesis_bg_black"), &Vector3f::zero(), &Vector3f::zero(), 1.0, 0, 0, false, 0);
        EffectModule::set_alpha_last(attacker_boma, 0.90);
        // let handle = EffectModule::req(attacker_boma, Hash40::new("sys_dragoon_bg_lightning"), &Vector3f::zero(), &Vector3f::zero(), 1.0, 0, 0, false, 0);
        // EffectModule::set_rgb(attacker_boma, handle as u32, 0.2, 100.0, 0.2);
        // EffectModule::set_frame(attacker_boma, handle as u32, 10.0);
        // EffectModule::set_frame(attacker_boma, black_handle as u32, 5.0);
        // MotionAnimcmdModule::call_script_single(attacker_boma, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_dragoon"), -1);
        SoundModule::play_se(attacker_boma, Hash40::new("se_dragoon_attack"), false, false, false, false, enSEType(0));
        // EffectModule::req(attacker_boma, Hash40::new("sys_bg_lightning"), &Vector3f::zero(), &Vector3f::zero(), 1.0, 0, 0, false, 0);
        // smash_script::lua_args!(common, Hash40::new("sys_bg_black"), 0, 0, 0,0, 0, 0, 1);
        // EFFECT_GLOBAL_BACK_GROUND(common.lua_state_agent);
    }


    // for x in 0..0x16 {
    //     let range_end = if x == 0x15 { 2 } else { 4 };
    //     print!("{:#05x}: ", x * 0x10);
    //     for y in 0..range_end {
    //         print!("{:#010x} ", *(knockback_info as *const u32).add(x * 4 + y));
    //     }
    //     print!("| ");
    //     for y in 0..range_end {
    //         print!("{:3.2} ", *knockback_info.add(x*4 + y));
    //     }
    //     println!();
    // }

    // println!("{}", *(knockback_info as *const u8).add(0x8a));

    // for x in 0..(reaction as u64) {
    //     y_speed -= defender_boma.get_param_float("damage_fly_top_air_accel_y", "");
    //     if y_speed.abs() >= defender_boma.get_param_float("damage_fly_top_speed_y_stable", "") {
    //         y_speed = defender_boma.get_param_float("damage_fly_top_speed_y_stable", "") * y_speed.signum();
    //     }
    //     y_pos += y_speed;
    //     println!("{}", y_pos);
    // }
    // println!("<{}, {}>: <{}, {}> @ {} | {}", initial_pos_x, initial_pos_y, initial_speed_x, initial_speed_y, knockback, reaction);
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    airdodge_cancel(boma, status_kind, situation_kind, cat[0], frame);
    shine_jump_cancel(fighter);

    // if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
    //     let last_pos = VarModule::get_vec3(fighter.object(), vars::common::instance::LAST_ATTACK_HIT_LOCATION);
    //     let handle = EffectModule::req(fighter.module_accessor, Hash40::new("sys_ripple"), &last_pos, &Vector3f::new(-90.0f32.to_radians(), 0.0, 0.0), 1.0, 0, 0, false, 0);
    //     EffectModule::set_disable_render_offset_last(fighter.boma());
    //     EffectModule::set_scale_last(boma, &Vector3f::new(4.0, 0.1, 4.0));
    //     //EffectModule::set_frame_last(boma, handle as u32, 8.0);
    //     EffectModule::set_billboard(boma, handle as u32, true);
    //     //LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    //     LAST_EFFECT_SET_RATE(fighter, 1.0);
    // }

    // Frame Data
    //frame_data(boma, status_kind, motion_kind, frame);
}

#[smashline::fighter_frame_callback]
fn all_fighters(fighter: &mut L2CFighterCommon) {
    unsafe {

        if VarModule::get_int(fighter.object(), COUNTER) > 0 {
            let scale = (30 - VarModule::get_int(fighter.object(), COUNTER)) as f32 / 30.0 * 5.0;
            EffectModule::set_scale(fighter.module_accessor, VarModule::get_int(fighter.object(), HANDLE) as u32, &Vector3f::new(scale, 1.0, scale));
            VarModule::dec_int(fighter.object(), COUNTER);
        } else {
            VarModule::set_int(fighter.object(), HANDLE, 0);
        }
        

        let frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        let last = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
        if frame - 1.0 < 0.0  && frame != 0.0 {
            println!("<{:.2}, {:.2}>", PostureModule::pos_x(fighter.module_accessor), PostureModule::pos_y(fighter.module_accessor));
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_WOLF )]
pub fn wolf_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		wolf_frame(fighter)
    }
}

pub unsafe fn wolf_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}