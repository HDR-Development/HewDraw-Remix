use super::*;
use utils::ext::*;
use std::arch::asm;

pub fn install() {
    skyline::install_hooks!(
        process_knockback,
        calculate_knockback
    );
}

extern "C" {
    #[link_name = "_ZN3app6camera13get_dead_areaEv"]
    fn get_dead_area() -> Rect;

    #[link_name = "_ZN3app10sv_animcmd25EFFECT_GLOBAL_BACK_GROUNDEP9lua_State"]
    fn EFFECT_GLOBAL_BACK_GROUND(lua_state: u64);
}

#[derive(Debug, Copy, Clone)]
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
    pub x_pos_prev: f32,
    pub y_pos_prev: f32,
    pub decay_x: f32,
    pub decay_y: f32,
    pub speed_up_mul: f32,
}

impl KnockbackCalcContext {
    pub fn step(&mut self) {
        self.x_pos_prev = self.x_pos;
        self.y_pos_prev = self.y_pos;
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

/// Knockback log
/// 0x8a -> the opponent was grounded (bool)
/// 0x90 -> backslash (bool)
/// 0x60 -> stop delay (f32) 
/// 0x50 -> collision attr (Hash40)
/// 0x40 -> launch angle in rad (f32)
/// 0x4 -> level (?)
/// 0x4c -> hitstop frame

static mut IS_CALCULATING: Option<(u32, u32)> = None;

#[skyline::hook(offset = 0x402f00, inline)]
unsafe fn calculate_knockback(ctx: &skyline::hooks::InlineCtx) {
    let damage_module = *ctx.registers[19].x.as_ref();
    let our_boma = *((damage_module + 0x8) as *mut *mut smash::app::BattleObjectModuleAccessor);
    let ptr = *ctx.registers[20].x.as_ref() as *mut u8;
    let id = *(ptr.add(0x24) as *const u32);
    IS_CALCULATING = Some(((*our_boma).battle_object_id, id));
}

#[skyline::hook(offset = 0x403950, inline)]
unsafe fn process_knockback(ctx: &skyline::hooks::InlineCtx) {
    if let Some((defender, attacker)) = IS_CALCULATING {
        let boma = *ctx.registers[20].x.as_ref() as *mut smash::app::BattleObjectModuleAccessor;
        if (*boma).battle_object_id == defender {
            process_daisydaikon_knockback(defender, attacker);
            calculate_finishing_hit(defender, attacker, *ctx.registers[19].x.as_ref() as *const f32);
        }
    }
}

pub unsafe extern "C" fn process_daisydaikon_knockback(defender: u32, attacker: u32) {
    let defender_boma = &mut *(*util::get_battle_object_from_id(defender)).module_accessor;
    let attacker_boma = &mut *(*util::get_battle_object_from_id(attacker)).module_accessor;
    if defender_boma.is_item() {
        if (defender_boma.kind() == *ITEM_KIND_DAISYDAIKON) {
            if attacker_boma.is_fighter() {
                let attacker_team_no = (TeamModule::hit_team_no(attacker_boma) as i32);
                TeamModule::set_team(defender_boma, attacker_team_no, false);
            } else {
                HitModule::set_xlu_frame_global(defender_boma, 15, 0);
            }
            StatusModule::change_status_force(defender_boma, *ITEM_STATUS_KIND_THROW, true);
        }
    }
}

pub unsafe extern "C" fn calculate_finishing_hit(defender: u32, attacker: u32, knockback_info: *const f32) {
    *(knockback_info.add(0x4c / 4) as *mut u32) = 60;
    let defender_boma = &mut *(*util::get_battle_object_from_id(defender)).module_accessor;
    let attacker_boma = &mut *(*util::get_battle_object_from_id(attacker)).module_accessor;
    // let before = std::time::Instant::now();
    // println!("");
    if !is_potential_finishing_hit(defender_boma, attacker_boma) { 
        // let elapsed = std::time::Instant::now().duration_since(before);
        // println!("is_potential_finishing_hit calculation time: {:?}", elapsed);
        return; 
    }
    // let elapsed = std::time::Instant::now().duration_since(before);
    // println!("is_potential_finishing_hit calculation time: {:?}", elapsed);
    // let before = std::time::Instant::now();
    if !is_valid_finishing_hit(knockback_info, defender_boma) { 
        // let elapsed = std::time::Instant::now().duration_since(before);
        // println!("is_valid_finishing_hit calculation time: {:?}", elapsed);
        return; 
    }
    // let elapsed = std::time::Instant::now().duration_since(before);
    // println!("is_valid_finishing_hit calculation time: {:?}", elapsed);
    call_finishing_hit_effects(defender_boma);
}

unsafe extern "C" fn is_potential_finishing_hit(defender_boma: &mut BattleObjectModuleAccessor, attacker_boma: &mut BattleObjectModuleAccessor) -> bool {
    if !defender_boma.is_fighter() { 
        // println!("kill screen defender is not fighter"); 
        return false; 
    }
    if !attacker_boma.is_fighter() && !attacker_boma.is_weapon() { 
        // println!("kill screen attacker is not fighter or weapon"); 
        return false; 
    }

    if VarModule::get_int(defender_boma.object(), COUNTER) > 0 {
        // println!("kill screen is on cooldown"); 
        return false; 
    }

    if attacker_boma.is_fighter() && util::is_no_finishing_hit(attacker_boma) { 
        // println!("kill screen incoming attack is_no_finishing_hit"); 
        return false; 
    }

    // special case for training mode
    if util::is_training_mode() {
        if VarModule::is_flag(defender_boma.object(), vars::common::instance::ENABLE_FRAME_DATA_DEBUG) {
            return true;
        }

        let mut is_training_toggle = false;
        if attacker_boma.is_weapon() {
            let owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let owner = util::get_battle_object_from_id(owner_id);
            let owner_boma = &mut *(*owner).module_accessor;
            if VarModule::is_flag(owner_boma.object(), vars::common::instance::ENABLE_FRAME_DATA_DEBUG) {
                return true;
            }
        } else if VarModule::is_flag(attacker_boma.object(), vars::common::instance::ENABLE_FRAME_DATA_DEBUG) {
            return true;
        }
        // println!("kill screen training mode is not enabled"); 
        return false;
    }

    if is_teammate_alive(defender_boma) { 
        // println!("kill screen teammate stock exists"); 
        return false; 
    }

    // ensure kill calculations only occur when the defender is on their last stock
    let defender_entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_info = app::lua_bind::FighterManager::get_fighter_information(
        crate::singletons::FighterManager(), 
        app::FighterEntryID(defender_entry_id)
    );
    if app::lua_bind::FighterInformation::stock_count(fighter_info) != 1 { 
        // println!("kill screen defender has more stocks"); 
        return false; 
    }

    return true;
}

pub unsafe extern "C" fn is_teammate_alive(defender_boma: &mut BattleObjectModuleAccessor) -> bool {
    for object_id in util::get_all_active_battle_object_ids() {
        let object = util::get_battle_object_from_id(object_id);
        if object.is_null() { continue; } // skip null

        let other_boma = &mut *(*object).module_accessor;
        if other_boma.kind() == *FIGHTER_KIND_NANA { continue; } // skip nana

        // skip this boma if it belongs to the same player
        let defender_entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let other_entry_id = WorkModule::get_int(other_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        if defender_entry_id == other_entry_id { continue; }

        // check if another player on the same team has at least one stock
        let fighter_info = app::lua_bind::FighterManager::get_fighter_information(
            crate::singletons::FighterManager(), 
            app::FighterEntryID(other_entry_id)
        );
        if FighterUtil::get_team_color(defender_boma) == FighterUtil::get_team_color(other_boma)
        && app::lua_bind::FighterInformation::stock_count(fighter_info) > 0 { 
            return true;
        }
    }

    return false;
}

const NUM_ANGLE_CHECK: i32 = 10;
const NUM_FALSE_ANGLES_ALLOWED: i32 = 1;

unsafe extern "C" fn is_valid_finishing_hit(knockback_info: *const f32, defender_boma: &mut BattleObjectModuleAccessor) -> bool {
    let knockback = *knockback_info;
    let initial_speed_x = *knockback_info.add(4);
    let initial_speed_y = *knockback_info.add(5);
    // let initial_pos_x = *knockback_info.add(8);
    // let initial_pos_y = *knockback_info.add(9);
    let reaction = *knockback_info.add(0x48 / 4);
    let angle = *knockback_info.add(0x10);
    let top_lw = defender_boma.get_param_float("battle_object", "fly_top_angle_lw");
    let top_hi = defender_boma.get_param_float("battle_object", "fly_top_angle_hi");

    // let ecb_top = *GroundModule::get_rhombus(defender_boma, true).add(0);
    let ecb_bottom = *GroundModule::get_rhombus(defender_boma, true).add(1);
    let ecb_left = *GroundModule::get_rhombus(defender_boma, true).add(2);
    let ecb_right = *GroundModule::get_rhombus(defender_boma, true).add(3);

    let base_sdi = WorkModule::get_param_float(defender_boma, smash::hash40("common"), smash::hash40("hit_stop_delay_flick_mul"));
    let sdi_frame = WorkModule::get_param_int(defender_boma, smash::hash40("common"), smash::hash40("hit_stop_delay_flick_frame"));
    let sdi_max_count = WorkModule::get_param_int(defender_boma, smash::hash40("common"), smash::hash40("hit_stop_delay_flick_max_count"));
    let base_asdi = WorkModule::get_param_float(defender_boma, smash::hash40("common"), smash::hash40("hit_stop_delay_auto_mul"));
    let sdi_mul = *knockback_info.add(24);

    let hitlag_max = WorkModule::get_param_float(defender_boma, smash::hash40("battle_object"), smash::hash40("hitstop_frame_max"));
    let hitlag_add = WorkModule::get_param_float(defender_boma, smash::hash40("battle_object"), smash::hash40("hitstop_frame_add"));
    let hitlag_mul = WorkModule::get_param_float(defender_boma, smash::hash40("battle_object"), smash::hash40("hitstop_frame_mul"));
    let damage = *knockback_info.add(22);
    let hitlag = (2.0 * (damage * hitlag_mul + hitlag_add)).clamp(0.0, hitlag_max).floor();
    let sdi_count = ((hitlag - 1.0) / (sdi_frame as f32)).clamp(0.0, sdi_max_count as f32).floor();
    let sdi_distance = (sdi_count * base_sdi + base_asdi) * sdi_mul;

    // println!("base_sdi: {}", base_sdi);
    // println!("sdi_frame: {}", sdi_frame);
    // println!("sdi_max_count: {}", sdi_max_count);
    // println!("base_asdi: {}", base_asdi);
    // println!("sdi_mul: {}", sdi_mul);

    // println!("hitlag_max: {}", hitlag_max);
    // println!("hitlag_add: {}", hitlag_add);
    // println!("hitlag_mul: {}", hitlag_mul);
    // println!("damage: {}", damage);
    // println!("hitlag: {}", hitlag);
    // println!("sdi_distance: {}", sdi_distance);
    // for off in 0..128 {
    //     println!("{}: {}", off, *knockback_info.add(off));
    // }

    let speed_up_mul = if defender_boma.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP) {
        defender_boma.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG)
    } else {
        1.0
    };

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
        x_pos: ecb_bottom.x,
        y_pos: ecb_bottom.y,
        x_pos_prev: ecb_bottom.x,
        y_pos_prev: ecb_bottom.y,
        decay_x: defender_boma.get_param_float("common", "damage_air_brake") * angle.cos().abs(),
        decay_y: defender_boma.get_param_float("common", "damage_air_brake") * angle.sin().abs(),
        speed_up_mul: speed_up_mul,
    };
    //println!("{:#x}: {:?}", defender, context);

    let blastzones = get_dead_area();
    let mag = (context.y_launch_speed.powi(2) + context.x_launch_speed.powi(2)).sqrt();
    let kb_angle = context.y_launch_speed.atan2(context.x_launch_speed).to_degrees();
    let di_angle = defender_boma.get_param_float("common", "damage_fly_correction_max");
    let min_di = kb_angle - di_angle;
    let max_di = kb_angle + di_angle;
    // println!("base kb angle: {}, di angle: {}, min_di: {}, max_di: {}", kb_angle, di_angle, min_di, max_di);

    let step = (di_angle * 2.0) / (NUM_ANGLE_CHECK as f32);
    let context_ref = context;
    let mut false_angle_num = 0;
    for idx in -1..NUM_ANGLE_CHECK + 1 {
        let ang = (min_di + (idx as f32 * step)).to_radians();
        context.x_launch_speed = ang.cos() * mag;
        context.y_launch_speed = ang.sin() * mag;

        // special checks for max SDI left/right
        if idx == -1 { // full SDI left (negative X)
            let ang = if ang.sin() > 0.0 {max_di} else {min_di}.to_radians();
            context.x_launch_speed = ang.cos() * mag;
            context.y_launch_speed = ang.sin() * mag;

            // check wall tech
            let ecb_offset = ecb_left.x - ecb_bottom.x;
            if GroundModule::ray_check(
                defender_boma, 
                &Vector2f{ x: context.x_pos, y: context.y_pos}, 
                &Vector2f{ x: -1.0 * sdi_distance + ecb_offset, y: 0.0},
                true
            ) == 1 {
                // println!("idx: {} would be wall techable on the left", idx);
                return false;
            }
            
            context.x_pos -= sdi_distance;
            context.x_pos_prev -= sdi_distance;
        }
        else if idx == NUM_ANGLE_CHECK { // full SDI right (positive X)
            let ang = if ang.sin() > 0.0 {min_di} else {max_di}.to_radians();
            context.x_launch_speed = ang.cos() * mag;
            context.y_launch_speed = ang.sin() * mag;

            // check wall tech
            let ecb_offset = ecb_right.x - ecb_bottom.x;
            if GroundModule::ray_check(
                defender_boma, 
                &Vector2f{ x: context.x_pos, y: context.y_pos}, 
                &Vector2f{ x: sdi_distance + ecb_offset, y: 0.0},
                true
            ) == 1 {
                // println!("idx: {} would be wall techable on the right", idx);
                return false;
            }
            
            context.x_pos += sdi_distance;
            context.x_pos_prev += sdi_distance;
        }

        let mut x = 0;
        let mut does_angle_kill = false;


        // check possible amsah techs
        context.step();
        if context.y_pos - context.y_pos_prev < base_asdi * sdi_mul
        && GroundModule::ray_check(
            defender_boma, 
            &Vector2f{ x: context.x_pos, y: context.y_pos}, 
            &Vector2f{ x: 0.0, y: sdi_distance},
            true
        ) == 1 {
            // println!("idx: {} would be amsah techable", idx);
            return false;
        }

        // do first iteration of knockback check
        if GroundModule::ray_check(
            defender_boma, 
            &Vector2f{ x: context.x_pos_prev, y: context.y_pos_prev}, 
            &Vector2f{ x: context.x_pos - context.x_pos_prev, y: context.y_pos - context.y_pos_prev}, 
            context.y_pos <= context.y_pos_prev // only check for platforms if going downwards
        ) == 1 {
            // if it's ever possible to touch stage, this is not a valid finishing hit
            // println!("idx: {} would touch stage", idx);
            return false;
        }
        if !blastzones.contains(context.x_pos, context.y_pos){
            // println!("{} will kill! adding to counter.", ang.to_degrees());
            does_angle_kill = true;
        }
        x += 1;


        while context.hitstun > x as f32  {
            context.step();
            if GroundModule::ray_check(
                defender_boma, 
                &Vector2f{ x: context.x_pos_prev, y: context.y_pos_prev}, 
                &Vector2f{ x: context.x_pos - context.x_pos_prev, y: context.y_pos - context.y_pos_prev}, 
                context.y_pos <= context.y_pos_prev // only check for platforms if going downwards
            ) == 1 {
                // if it's ever possible to touch stage, this is not a valid finishing hit
                // println!("idx: {} would touch stage", idx);
                return false;
            }
            if !blastzones.contains(context.x_pos, context.y_pos){
                // println!("{} will kill! adding to counter.", ang.to_degrees());
                does_angle_kill = true;
                break;
            }
            x += 1;
        }
        context = context_ref;
        if idx != -1 && idx != NUM_ANGLE_CHECK && !does_angle_kill {false_angle_num += 1;}
        if false_angle_num > NUM_FALSE_ANGLES_ALLOWED { 
            // println!("false angles: at least {}", false_angle_num);
            return false; 
        }
    }
    // println!("false angles: {}", false_angle_num);
    return true;
}

const HANDLE: i32 = 0x01FF;
const COUNTER: i32 = 0x01FE;

pub unsafe extern "C" fn call_finishing_hit_effects(defender_boma: &mut BattleObjectModuleAccessor) {
    let handle = EffectModule::req_screen(defender_boma, Hash40::new("bg_finishhit"), false, true, true);
    EffectModule::set_billboard(defender_boma, handle as u32, true);
    EffectModule::set_disable_render_offset_last(defender_boma);
    VarModule::set_int(defender_boma.object(), HANDLE, handle as i32);
    VarModule::set_int(defender_boma.object(), COUNTER, 20);
    SoundModule::play_se(defender_boma, Hash40::new("se_common_boss_down"), false, false, false, false, enSEType(0));
    VarModule::set_int(defender_boma.object(), COUNTER, 20);
    SlowModule::set_whole(defender_boma, 8, 25);
    let common = util::get_fighter_common_from_accessor(defender_boma);
    EFFECT_GLOBAL_BACK_GROUND(common.lua_state_agent);
}