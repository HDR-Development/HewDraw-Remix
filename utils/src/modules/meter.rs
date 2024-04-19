use skyline::hooks::InlineCtx;
use smash::app;
use smash::app::{BattleObject, BattleObjectModuleAccessor};
use smash::app::lua_bind::*;
use smash::app::enSEType;
use smash::app::smashball::*;
use smash::lib::lua_const::*;
use smash::phx::*;
use crate::ext::*;
use crate::offsets;
use crate::consts::*;
use super::*;
use smash_script::macros;

macro_rules! get_meter_module {
    ($object:ident) => {{
        unsafe {
            let vtable = *($object as *mut *mut *mut u64);
            &mut *super::get_entry::<MeterModule>(vtable, METER_MODULE_OFFSET).expect("Did not find MeterModule!")
        }
    }}
}

macro_rules! has_meter_module {
    ($object:ident) => {{
        unsafe {
            if $object.is_null() {
                false
            } else {
                let vtable = *($object as *mut *mut *mut u64);
                super::is_hdr_object(vtable as _) && !super::get_entry::<MeterModule>(vtable, METER_MODULE_OFFSET).is_none()
            }
        }
    }}
}

macro_rules! require_meter_module {
    ($object:ident) => {{
        if !has_meter_module!($object) {
            panic!("BattleObject does not contain reference to MeterModule!");
        }
        get_meter_module!($object)
    }}
}

#[repr(C)]
pub struct MeterModule {
    owner: *mut BattleObject,
    current_meter: f32,
    meter_cap: i32,
    meter_per_level: f32,
    remaining_show_frames: i32,
    damage_gain_mul: f32,
    watch: bool,
    watching_motion: Hash40,
    watching_frame: f32,
    has_hit: bool,
    last_levels_consumed: i32,
    last_levels_added: i32
}

impl MeterModule {
    pub(crate) fn new(owner: *mut BattleObject) -> Self {
        Self {
            owner,
            current_meter: 0.0,
            meter_cap: 6,
            meter_per_level: 50.0,
            remaining_show_frames: -1,
            damage_gain_mul: 1.0,
            watch: false,
            watching_motion: Hash40::new("invalid"),
            watching_frame: 0.0,
            has_hit: false,
            last_levels_consumed: 0,
            last_levels_added: 0
        }
    }

    fn flash(object: *mut BattleObject) {
        let module = require_meter_module!(object);

        unsafe {
            let module_accessor = (*object).module_accessor;
            EffectModule::kill_kind(module_accessor, Hash40::new("sys_damage_curse"), false, true);
            EffectModule::req_on_joint(
                module_accessor,
                Hash40::new("sys_damage_curse"),
                Hash40::new("top"),
                &Vector3f::new(0.0, 0.0, 0.0),
                &Vector3f::zero(),
                0.5,
                &Vector3f::new(0.1, 0.1, 0.5),
                &Vector3f::new(0.1, 0.1, 0.5),
                false,
                0,
                0,
                0
            );
        }

        module.remaining_show_frames = ParamModule::get_int(object, ParamType::Common, "meter_flash_frame_count");
    }

    #[export_name = "MeterModule__show"]
    pub extern "Rust" fn show(object: *mut BattleObject) {
        let module = require_meter_module!(object);
        if module.remaining_show_frames == -1 {
            module.remaining_show_frames = -2;
        }

        if module.remaining_show_frames < 0 {
            Self::display(module.owner, 0);
        }
    }

    #[export_name = "MeterModule__stop_show"]
    pub extern "Rust" fn stop_show(object: *mut BattleObject) {
        let module = require_meter_module!(object);
        if module.remaining_show_frames != -1 && module.remaining_show_frames < 0 {
            module.remaining_show_frames = 0;
        }
    }

    pub extern "Rust" fn display(object: *mut BattleObject, new_levels: i32) {
        unsafe {
            let is_loss = new_levels < 0;
            let total_levels = if is_loss {
                Self::level(object) + new_levels.abs()
            } else {
                Self::level(object)
            };
            let module_accessor = (*object).module_accessor;
            EffectModule::kill_kind(module_accessor, Hash40::new("sys_starrod_bullet"), false, true);
            for x in 0..total_levels {
                let position = Vector3f::new(
                    -15.0 + 5.0 * (x % 5 + 1) as f32,
                    22.0 + 5.0 * (x / 5) as f32,
                    -15.0 + 5.0 * (x % 5 + 1) as f32,
                );
                let handle = EffectModule::req_follow(
                    module_accessor,
                    Hash40::new("sys_starrod_bullet"),
                    Hash40::new("top"),
                    &position,
                    &Vector3f::zero(),
                    0.3,
                    false,
                    0,
                    0,
                    0,
                    0,
                    0,
                    false,
                    false,
                ) as u32;
                if total_levels - new_levels.abs() - 1 >= x {
                    continue;
                }
                if is_loss {
                    EffectModule::set_alpha_last(module_accessor, 0.15);
                    EffectModule::set_scale_last(module_accessor, &Vector3f::new(0.25, 0.25, 0.25));
                } else {
                    EffectModule::set_rgb(module_accessor, handle, 0.1, 0.7, 3.0);
                    EffectModule::set_scale_last(module_accessor, &Vector3f::new(0.4, 0.4, 0.4));
                }
            }
        }
    }

    fn kill_display(object: *mut BattleObject) {
        let module = require_meter_module!(object);
        unsafe {
            let module_accessor = (*module.owner).module_accessor;
            WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
            EffectModule::kill_kind(module_accessor, Hash40::new("sys_damage_curse"), false, true);
            EffectModule::kill_kind(module_accessor, Hash40::new("sys_starrod_bullet"), false, true);
            
            module.remaining_show_frames = -1;
        }
    }

    fn keep_watch(object: *mut BattleObject) {
        let module = require_meter_module!(object);
        if module.watch {
            unsafe {
                let module_accessor = (*module.owner).module_accessor;
                if module.watching_motion != Hash40::new_raw(MotionModule::motion_kind(module_accessor))
                || (module.watching_frame > MotionModule::frame(module_accessor) && !MotionModule::is_looped(module_accessor)) {
                    module.watch = false;
                    module.watching_motion = Hash40::new("invalid");
                    module.watching_frame = 0.0;
                    module.damage_gain_mul = 1.0;
                } else {
                    module.watching_frame = MotionModule::frame(module_accessor);
                }
            }
        }
    }

    pub(super) fn signal_hit(object: *mut BattleObject) {
        let module = require_meter_module!(object);
        module.has_hit = module.watch;
    }

    #[export_name = "MeterModule__set_meter_per_level"]
    pub extern "Rust" fn set_meter_per_level(object: *mut BattleObject, amount: f32) {
        let module = require_meter_module!(object);
        module.meter_per_level = amount;
    }

    #[export_name = "MeterModule__meter_per_level"]
    pub extern "Rust" fn meter_per_level(object: *mut BattleObject) -> f32 {
        require_meter_module!(object).meter_per_level
    }

    #[export_name = "MeterModule__set_meter_cap"]
    pub extern "Rust" fn set_meter_cap(object: *mut BattleObject, amount: i32) {
        let module = require_meter_module!(object);
        module.meter_cap = amount;
    }

    #[export_name = "MeterModule__meter_cap"]
    pub extern "Rust" fn meter_cap(object: *mut BattleObject) -> i32 {
        require_meter_module!(object).meter_cap
    }

    #[export_name = "MeterModule__meter"]
    pub extern "Rust" fn meter(object: *mut BattleObject) -> f32 {
        require_meter_module!(object).current_meter
    }

    #[export_name = "MeterModule__level"]
    pub extern "Rust" fn level(object: *mut BattleObject) -> i32 {
        let current = require_meter_module!(object).current_meter;
        (current / Self::meter_per_level(object)) as i32 // truncate down
    }

    #[export_name = "MeterModule__watch_damage"]
    pub extern "Rust" fn watch_damage(object: *mut BattleObject, watch: bool) {
        let module = require_meter_module!(object);
        module.watch = watch;
        if watch {
            unsafe {
                let module_accessor = (*object).module_accessor;
                let motion_kind = Hash40::new_raw(MotionModule::motion_kind(module_accessor));
                let frame = MotionModule::frame(module_accessor);
                module.watching_motion = motion_kind;
                module.watching_frame = frame;   
            }
        }
    }

    #[export_name = "MeterModule__set_damage_gain_mul"]
    pub extern "Rust" fn set_damage_gain_mul(object: *mut BattleObject, mul: f32) {
        require_meter_module!(object).damage_gain_mul = mul;
    }

    #[export_name = "MeterModule__damage_gain_mul"]
    pub extern "Rust" fn damage_gain_mul(object: *mut BattleObject) -> f32 {
        require_meter_module!(object).damage_gain_mul
    }

    #[export_name = "MeterModule__drain"]
    pub extern "Rust" fn drain(object: *mut BattleObject, count: i32) -> bool {
        let module = require_meter_module!(object);
        if Self::level(module.owner) >= count {
            module.current_meter -= Self::meter_per_level(module.owner) * count as f32;
            module.last_levels_consumed += count;
            true
        } else {
            false
        }
    }

    #[export_name = "MeterModule__drain_direct"]
    pub extern "Rust" fn drain_direct(object: *mut BattleObject, amount: f32) {
        let module = require_meter_module!(object);
        if Self::meter(module.owner) >= amount {
            module.current_meter -= amount;
        }
        else {
            module.current_meter = 0.0;
        }
    }

    #[export_name = "MeterModule__add"]
    pub extern "Rust" fn add(object: *mut BattleObject, amount: f32) {
        let module = require_meter_module!(object);
        let count = Self::level(module.owner);
        module.current_meter += amount;
        module.current_meter = module.current_meter.min(Self::meter_cap(module.owner) as f32 * Self::meter_per_level(module.owner));
        module.last_levels_added += Self::level(module.owner) - count;
    }

    #[export_name = "MeterModule__reset"]
    pub extern "Rust" fn reset(object: *mut BattleObject) {
        let module = require_meter_module!(object);
        module.current_meter = 0.0;
        module.meter_cap = 6;
        module.meter_per_level = 50.0;
        module.damage_gain_mul = 1.0;
        module.has_hit = false;
        module.last_levels_added = 0;
        module.last_levels_consumed = 0;
        module.remaining_show_frames = 0;
        module.watch = false;
        module.watching_frame = 0.0;
        module.watching_motion = Hash40::new("invalid");
    }

    #[export_name = "MeterModule__update"]
    pub extern "Rust" fn update(object: *mut BattleObject, show_flash_on_change: bool) {
        let module = require_meter_module!(object);

        Self::keep_watch(module.owner);

        let new_levels = if module.watch && module.has_hit {
            let difference = VarModule::get_float(module.owner, vars::common::instance::LAST_ATTACK_DAMAGE_DEALT);
            // if lucario causes valid damage, reset regen pause timer
            unsafe { 
                let obj = &mut *module.owner ;
                if difference > 0.0 && obj.is_fighter() && obj.kind() == *FIGHTER_KIND_LUCARIO {
                    VarModule::set_int(module.owner, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, 0);
                }
            }
            let current = Self::level(module.owner);
            module.current_meter += difference * module.damage_gain_mul;
            module.watch = false;
            module.watching_motion = Hash40::new("invalid");
            module.watching_frame = 0.0;
            module.damage_gain_mul = 1.0;
            module.current_meter = module.current_meter.min(Self::meter_cap(module.owner) as f32 * Self::meter_per_level(module.owner));
            Self::level(module.owner) - current
        } else {
            0
        };

        module.has_hit = false;

        let new_levels = new_levels - module.last_levels_consumed + module.last_levels_added;
        module.last_levels_consumed = 0;
        module.last_levels_added = 0;

        unsafe {
            if (*object).agent_kind_hash.hash == Hash40::new("fighter_kind_dolly").hash {
                let boma = (*object).module_accessor;
                let is_superspecial = !WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
                dolly_super_special_check(boma, is_superspecial as u8);
            }
        }

        if new_levels != 0 && show_flash_on_change {
            Self::display(module.owner, new_levels);
            Self::flash(module.owner);
        }

        match module.remaining_show_frames {
            0 => Self::kill_display(module.owner),
            -1 => {},
            _ => module.remaining_show_frames -= 1
        }
    }
}

#[skyline::hook(offset = 0x46ae84, inline)]
unsafe fn hit_module_handle_attack_event(ctx: &InlineCtx)  {
    let data = *ctx.registers[1].x.as_ref() as *mut u32;
    let attacker_id = *data;

    let collision_id = *data.add(1);
    let battle_object = &mut *utils_dyn::util::get_battle_object_from_id(attacker_id);
    if !battle_object.is_fighter() && !battle_object.is_weapon() {
        return;
    }

    let collision_data = *ctx.registers[27].x.as_ref() as *mut f32;
    let loc_x = *collision_data.add(4);
    let loc_y = *collision_data.add(5);
    let loc_z = *collision_data.add(6);
    VarModule::set_int(battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID, collision_id as i32);
    VarModule::set_vec3(battle_object, vars::common::instance::LAST_ATTACK_HIT_LOCATION, Vector3f { x: loc_x, y: loc_y, z: loc_z });
}

#[skyline::hook(offset = 0x4c7080)]
unsafe fn shield_module_send_shield_attack_collision_event(shield_module: *mut u64, opp_attack_module: *mut u64, collision: *mut u8, group_index: i32, raw_power: f32, real_power: f32, pos_x: f32, lr: f32) {
    call_original!(shield_module, opp_attack_module, collision, group_index, raw_power, real_power, pos_x, lr);
    let attacker_id = *(collision.add(0x24) as *const u32);
    let battle_object = &mut *utils_dyn::util::get_battle_object_from_id(attacker_id);
    if !battle_object.is_fighter() && !battle_object.is_weapon() {
        return;
    }

    let hitbox_id = *(collision.add(0x33) as *const u8);
    let loc_x = *(collision.add(0x10) as *const f32);
    let loc_y = *(collision.add(0x14) as *const f32);
    let loc_z = *(collision.add(0x18) as *const f32);

    VarModule::set_int(battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID, hitbox_id as i32);
    VarModule::set_vec3(battle_object, vars::common::instance::LAST_ATTACK_HIT_LOCATION, Vector3f { x: loc_x, y: loc_y, z: loc_z });
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
const NUM_ANGLE_CHECK: i32 = 10;
const NUM_FALSE_ANGLES_ALLOWED: i32 = 1;

pub unsafe extern "C" fn is_no_finishing_hit(attacker_boma: &mut BattleObjectModuleAccessor) -> bool {
    for is_abs in [false, true] {
        for id in 0..8 {
            let attack_data = AttackModule::attack_data(attacker_boma, id, is_abs);
            let off = if is_abs { 0xd9 } else { 0xc9 };
            if AttackModule::is_attack(attacker_boma, id, is_abs)
            && *attack_data.cast::<bool>().add(off) {
                return true;
            }
        }
    }
    return false;
}

unsafe extern "C" fn is_potential_finishing_hit(defender_boma: &mut BattleObjectModuleAccessor, attacker_boma: &mut BattleObjectModuleAccessor) -> bool {
    if !defender_boma.is_fighter() { return false; }
    if !attacker_boma.is_fighter() && !attacker_boma.is_weapon() { return false; }
    if VarModule::get_int(defender_boma.object(), COUNTER) > 0 {
        // println!("no effects because COUNTER: {}", VarModule::get_int(defender_boma.object(), COUNTER));
        return false; 
    }
    if is_no_finishing_hit(attacker_boma) { return false; }
    if !is_training_mode() {
        // ensure kill calculations only occur when the defender is on their last stock
        let entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        // let manager = utils_dyn::singletons::FighterManager() as *mut u64;
        let fighter_info = app::lua_bind::FighterManager::get_fighter_information(crate::singletons::FighterManager(), app::FighterEntryID(entry_id));
        if FighterInformation::stock_count(fighter_info) != 1 { return false; }
    } else {
        // in training mode, check the flag for if the effects should be played
        let mut is_training_toggle = false;
        if !attacker_boma.is_weapon() {
            if VarModule::is_flag(attacker_boma.object(), vars::common::instance::ENABLE_FRAME_DATA_DEBUG) { is_training_toggle = true; }
        } else {
            let owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let owner = utils_dyn::util::get_battle_object_from_id(owner_id);
            let owner_boma = &mut *(*owner).module_accessor;
            if VarModule::is_flag(owner_boma.object(), vars::common::instance::ENABLE_FRAME_DATA_DEBUG) { is_training_toggle = true; }
        }
        if !is_training_toggle { return false; }
    }
    return true;
}

unsafe extern "C" fn is_valid_finishing_hit(knockback_info: *const f32, defender_boma: &mut BattleObjectModuleAccessor) -> bool {
    let knockback = *knockback_info;
    let initial_speed_x = *knockback_info.add(4);
    let initial_speed_y = *knockback_info.add(5);
    let initial_pos_x = *knockback_info.add(8);
    let initial_pos_y = *knockback_info.add(9);
    let reaction = *knockback_info.add(0x48 / 4);
    let angle = *knockback_info.add(0x10);
    let top_lw = defender_boma.get_param_float("battle_object", "fly_top_angle_lw");
    let top_hi = defender_boma.get_param_float("battle_object", "fly_top_angle_hi");

    let ecb_bottom = *GroundModule::get_rhombus(defender_boma, true).add(1);
    let base_sdi = WorkModule::get_param_float(defender_boma, smash::hash40("common"), smash::hash40("hit_stop_delay_flick_mul"));
    let sdi_frame = WorkModule::get_param_int(defender_boma, smash::hash40("common"), smash::hash40("hit_stop_delay_flick_frame"));
    let sdi_max_count = WorkModule::get_param_int(defender_boma, smash::hash40("common"), smash::hash40("hit_stop_delay_flick_max_count"));
    let base_asdi = WorkModule::get_param_float(defender_boma, smash::hash40("common"), smash::hash40("hit_stop_delay_auto_mul"));
    let sdi_mul = *knockback_info.add(24);
    // println!("base_sdi: {}", base_sdi);
    // println!("sdi_frame: {}", sdi_frame);
    // println!("sdi_max_count: {}", sdi_max_count);
    // println!("base_asdi: {}", base_asdi);
    // println!("sdi_mul: {}", sdi_mul);

    // for off in 0..128 {
    //     println!("{}: {}", off, *knockback_info.add(off));
    // }

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
    for idx in 0..NUM_ANGLE_CHECK {
        let ang = (min_di + (idx as f32 * step)).to_radians();
        context.x_launch_speed = ang.cos() * mag;
        context.y_launch_speed = ang.sin() * mag;
        let mut x = 0;
        let mut does_angle_kill = false;


        // first hitstun iteration, special amsah tech checks
        context.step();
        if context.y_pos - context.y_pos_prev < base_asdi * sdi_mul
        && GroundModule::ray_check(
            defender_boma, 
            &Vector2f{ x: context.x_pos, y: context.y_pos}, 
            &Vector2f{ x: 0.0, y: base_asdi * sdi_mul}, // TODO: extend this by the possible SDI distance
            true
        ) == 1 {
            println!("idx: {} would be amsah techable", idx);
            return false;
        }
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
        if !does_angle_kill {false_angle_num += 1;}
        if false_angle_num > NUM_FALSE_ANGLES_ALLOWED { 
            // println!("false angles: at least {}", false_angle_num);
            return false; 
        }
    }
    // println!("false angles: {}", false_angle_num);
    return true;
}

pub unsafe extern "C" fn call_finishing_hit_effects(defender_boma: &mut BattleObjectModuleAccessor) {
    let handle = EffectModule::req_screen(defender_boma, Hash40::new("bg_finishhit"), false, true, true);
    EffectModule::set_billboard(defender_boma, handle as u32, true);
    EffectModule::set_disable_render_offset_last(defender_boma);
    VarModule::set_int(defender_boma.object(), HANDLE, handle as i32);
    VarModule::set_int(defender_boma.object(), COUNTER, 20);
    SoundModule::play_se(defender_boma, Hash40::new("se_common_boss_down"), false, false, false, false, enSEType(0));
    VarModule::set_int(defender_boma.object(), COUNTER, 20);
    SlowModule::set_whole(defender_boma, 8, 25);
    let common = utils_dyn::util::get_fighter_common_from_accessor(defender_boma);
    EFFECT_GLOBAL_BACK_GROUND(common.lua_state_agent);
}

pub unsafe extern "C" fn calculate_finishing_hit(defender: u32, attacker: u32, knockback_info: *const f32) {
    *(knockback_info.add(0x4c / 4) as *mut u32) = 60;
    let defender_boma = &mut *(*utils_dyn::util::get_battle_object_from_id(defender)).module_accessor;
    let attacker_boma = &mut *(*utils_dyn::util::get_battle_object_from_id(attacker)).module_accessor;
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

static mut IS_CALCULATING: Option<(u32, u32)> = None;

#[skyline::hook(offset = 0x402f00, inline)]
unsafe fn calculate_knockback(ctx: &InlineCtx) {
    let damage_module = *ctx.registers[19].x.as_ref();
    let our_boma = *((damage_module + 0x8) as *mut *mut smash::app::BattleObjectModuleAccessor);
    let ptr = *ctx.registers[20].x.as_ref() as *mut u8;
    let id = *(ptr.add(0x24) as *const u32);
    IS_CALCULATING = Some(((*our_boma).battle_object_id, id));
}

#[skyline::hook(offset = 0x403950, inline)]
unsafe fn process_knockback(ctx: &InlineCtx) {
    if let Some((defender, attacker)) = IS_CALCULATING {
        let boma = *ctx.registers[20].x.as_ref() as *mut smash::app::BattleObjectModuleAccessor;
        if (*boma).battle_object_id == defender {
            calculate_finishing_hit(defender, attacker, *ctx.registers[19].x.as_ref() as *const f32);
        }
    }
}

// #[skyline::hook(offset = 0x401e50)]
// unsafe fn knockback_calculator(arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: f32, arg6: f32, arg7: f32, arg8: f32) -> f32 {
//     let knockback = call_original!(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8);
//     if let Some((defender, attacker, log)) = IS_CALCULATING.take() {
//         calculate_finishing_hit(defender, attacker, log, knockback);
//     }
//     knockback
// }

#[skyline::hook(offset = offsets::fighter_handle_damage())]
unsafe fn fighter_handle_damage_hook(fighter: *mut smash::app::BattleObject, arg: *const u8) {
    let module_accessor = (*fighter).module_accessor;
    let _entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let damage_received = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
    call_original!(fighter, arg);
    let damage_received = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE) - damage_received;
    let attacker_ids = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID);
    for x in 0..8 {
        if attacker_ids & (1 << x) == 0 {
            continue;
        }
        if let Some(object_id) = crate::util::get_active_battle_object_id_from_entry_id(x) {
            let object = crate::util::get_battle_object_from_id(object_id);
            if !object.is_null() && super::is_hdr_object((*object).vtable as _) {
                VarModule::set_float(object, vars::common::instance::LAST_ATTACK_DAMAGE_DEALT, damage_received);
                VarModule::set_int(object, vars::common::instance::LAST_ATTACK_RECEIVER_ENTRY_ID, (*fighter).battle_object_id as i32);
                MeterModule::signal_hit(object);
            }
        }
    }
}

#[skyline::hook(offset = 0x970ff0)]
pub unsafe extern "C" fn dolly_super_special_check(module_accessor: *mut smash::app::BattleObjectModuleAccessor, param_2: u8) {
    original!()(module_accessor, param_2)
}

#[repr(C)]
pub struct TempModule {
  vtable: *const u64,
  owner: *mut BattleObjectModuleAccessor,
  // ...
}

#[skyline::hook(offset = 0x971250)]
pub unsafe extern "C" fn dolly_super_special_check_param(work: &mut TempModule, _damage: &mut TempModule) -> u64 {
    let module_accessor = work.owner;
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    if MeterModule::level(module_accessor.as_mut().unwrap().object()) >= 4 {
        return 1
    }
    0
}

pub fn init() {
    skyline::install_hooks!(
        fighter_handle_damage_hook,
        dolly_super_special_check,
        dolly_super_special_check_param,
        hit_module_handle_attack_event,
        shield_module_send_shield_attack_collision_event,
        process_knockback,
        calculate_knockback
    );
}
