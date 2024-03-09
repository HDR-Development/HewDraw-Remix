
// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
// Disable QA jump cancels if not directly QA into the ground
unsafe fn disable_qa_jc(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status(*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP) {
        // only allow QAC from QA1
        if WorkModule::get_int(boma, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_COUNT) > 1 {
            VarModule::on_flag(boma.object(), vars::pikachu::instance::DISABLE_QA_JC);
        }
    }
    if boma.is_status(*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END) {
        // only allow QAC from QA into ground
        if boma.is_situation(*SITUATION_KIND_AIR) && boma.status_frame() > 2 {
            VarModule::on_flag(boma.object(), vars::pikachu::instance::DISABLE_QA_JC);
        }
    }
}

// Reset JC disable flag
unsafe fn reset_jc_disable_flag(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::is_flag(boma.object(), vars::pikachu::instance::DISABLE_QA_JC)
    && boma.is_situation(*SITUATION_KIND_GROUND)
    && ![*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&boma.status()) {
        VarModule::off_flag(boma.object(), vars::pikachu::instance::DISABLE_QA_JC);
        VarModule::off_flag(boma.object(), vars::common::instance::PERFECT_WAVEDASH);
    }
}

// This is held together with the finest Elmer's, I apologize
unsafe fn quick_attack_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL)
    && !VarModule::is_flag(boma.object(), vars::pikachu::instance::DISABLE_QA_JC) {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        PostureModule::add_pos(boma, &Vector3f::new(0.0, 2.5, 0.0));
        VarModule::on_flag(fighter.object(), vars::pikachu::instance::QUICK_ATTACK_CANCEL);
    }
    if VarModule::is_flag(boma.object(), vars::pikachu::instance::QUICK_ATTACK_CANCEL) {
        if fighter.is_cat_flag(Cat1::AirEscape) {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), false.into());
            VarModule::on_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
            PostureModule::add_pos(boma, &Vector3f::new(0.0, -2.5, 0.0));
        }
        if !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_LIGHT]) {
            VarModule::off_flag(fighter.object(), vars::pikachu::instance::QUICK_ATTACK_CANCEL);
            VarModule::off_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
        }
    }
}

pub unsafe fn electric_rats_moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    disable_qa_jc(boma);
    reset_jc_disable_flag(boma);
    fastfall_specials(fighter);
    skull_bash_edge_cancel(fighter);
}

#[no_mangle]
pub unsafe extern "Rust" fn electric_rats_common(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        electric_rats_moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT
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

unsafe fn skull_bash_edge_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END) {
        if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
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
        let handle = EffectModule::req(attacker_boma, Hash40::new("sys_bg_black"), &last_pos, &Vector3f::new(-90.0f32.to_radians(), 0.0, 0.0), 1.0, 0, 0, false, 0);
        EffectModule::set_disable_render_offset_last(attacker_boma);
        EffectModule::set_scale_last(attacker_boma, &Vector3f::new(0.0, 1.0, 0.0));
        //EffectModule::set_frame_last(boma, handle as u32, 8.0);
        EffectModule::set_billboard(attacker_boma, handle as u32, true);
        //LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EffectModule::set_rate_last(attacker_boma, 1.0);
        VarModule::set_int(attacker_boma.object(), HANDLE, handle as i32);
        VarModule::set_int(attacker_boma.object(), COUNTER, 30);
        let black_handle = EffectModule::req(attacker_boma, Hash40::new("sys_bg_finishhit"), &Vector3f::zero(), &Vector3f::zero(), 1.0, 0, 0, false, 0);
        //EffectModule::set_alpha_last(attacker_boma, 0.90);
        SlowModule::set_whole(attacker_boma, 4, 35);
        let common = utils::util::get_fighter_common_from_accessor(defender_boma);
        PLAY_SE_REMAIN(common, Hash40::new_raw(0x152af449c2));
        CAM_ZOOM_IN_arg5(common, 3.0, 0.0, 2.5, 0.0, 0.0);
        // let handle = EffectModule::req(attacker_boma, Hash40::new("sys_dragoon_bg_lightning"), &Vector3f::zero(), &Vector3f::zero(), 1.0, 0, 0, false, 0);
        // EffectModule::set_rgb(attacker_boma, handle as u32, 0.2, 100.0, 0.2);
        EffectModule::set_frame(attacker_boma, handle as u32, 10.0);
        EffectModule::set_frame(attacker_boma, black_handle as u32, 5.0);
        // MotionAnimcmdModule::call_script_single(attacker_boma, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_dragoon"), -1);
        SoundModule::play_se(attacker_boma, Hash40::new("se_common_finishhit"), false, false, false, false, enSEType(0));
        // EffectModule::req(attacker_boma, Hash40::new("sys_bg_lightning"), &Vector3f::zero(), &Vector3f::zero(), 1.0, 0, 0, false, 0);
        // smash_script::lua_args!(common, Hash40::new("sys_bg_black"), 0, 0, 0,0, 0, 0, 1);
        EFFECT_GLOBAL_BACK_GROUND(common.lua_state_agent);
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

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    quick_attack_cancel(fighter, boma);
}

unsafe extern "C" fn all_fighters(fighter: &mut L2CFighterCommon) {
    if VarModule::get_int(fighter.object(), COUNTER) > 0 {
        let scale = (30 - VarModule::get_int(fighter.object(), COUNTER)) as f32 / 30.0 * 5.0;
        EffectModule::set_scale(fighter.module_accessor, VarModule::get_int(fighter.object(), HANDLE) as u32, &Vector3f::new(scale, 1.0, scale));
        if VarModule::get_int(fighter.object(), COUNTER) == 20 {
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_finishhit"), false, false);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_black"), false, false);
            CAM_ZOOM_OUT(fighter);
        }
        if VarModule::get_int(fighter.object(), COUNTER) == 1 {
            SlowModule::clear_whole(fighter.boma());
        }
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

pub extern "C" fn pikachu_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pikachu_frame(fighter);
        electric_rats_common(fighter);
    }
}

pub unsafe fn pikachu_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}
pub fn install() {
    smashline::Agent::new("pikachu")
        .on_line(Main, pikachu_frame_wrapper)
        .on_line(Main, all_fighters)
        .install();
}
