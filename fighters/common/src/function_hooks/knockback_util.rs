use super::*;
use utils::ext::*;
use std::arch::asm;

extern "C" {
    #[link_name = "_ZN3app6camera13get_dead_areaEv"]
    fn get_dead_area() -> Rect;
}

#[repr(simd)]
#[derive(Debug)]
struct Rect {
    // left: f32,
    // right: f32,
    // top: f32,
    // bottom: f32,
    vec: [f32; 4]
}

impl Rect {
    fn contains(&self, x: f32, y: f32) -> bool {
        (self.vec[0] <= x && x <= self.vec[1]) && (self.vec[3] <= y && y <= self.vec[2])
    }
    fn grow(&mut self, x: f32, y: f32) {
        self.vec[0] -= x;
        self.vec[1] += x;
        self.vec[2] += y;
        self.vec[3] -= y;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct KnockbackCalcContext {
    pub defender_boma: *mut BattleObjectModuleAccessor,
    pub fly_top_angle_lw: f32,
    pub fly_top_angle_hi: f32,
    pub ecb_bottom: Vector4f,
    pub ecb_left: Vector4f,
    pub ecb_right: Vector4f,

    pub knockback: f32,
    pub hitstun: f32,
    pub damage: f32,
    pub sdi_mul: f32,
    pub base_asdi: f32,
    pub sdi_distance: f32,
    pub launch_radians: f32,
    pub launch_speed: Vector2f,
    pub y_chara_speed: f32,
    pub is_tumble: bool,
    pub gravity: f32,
    pub fall_speed: f32,
    pub pos: Vector2f,
    pub pos_prev: Vector2f,
    pub damage_air_brake: f32,
    pub speed_up_mul: f32,

    pub is_tech_possible: bool,
}

// number of DI angles checked
const NUM_ANGLES_CHECKED: i32 = 12;
const NUM_ANGLES_CHECKED_FINAL: i32 = 12;
// maximum number of survivable DI angles in a finishing hit
const SURVIVABLE_ANGLES_ALLOWED: i32 = 0;
const SURVIVABLE_ANGLES_ALLOWED_FINAL: i32 = 1;
// how many units into the blastzone a fighter will be declared dead
const DEAD_AREA_LENIENCY: f32 = 7.5;
const DEAD_AREA_LENIENCY_FINAL: f32 = 2.5;

impl KnockbackCalcContext {
    pub unsafe fn new(
        defender_boma: *mut BattleObjectModuleAccessor,
        knockback: f32,
        hitstun: f32,
        damage: f32,
        sdi_mul: f32,
        launch_radians: f32,
        launch_speed: Vector2f,
        is_tumble: bool,
    ) -> Self {
        let fly_top_angle_lw= WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("fly_top_angle_lw"));
        let fly_top_angle_hi= WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("fly_top_angle_hi"));
        let ecb_bottom = *GroundModule::get_rhombus(defender_boma, true).add(1);
        let ecb_left =   *GroundModule::get_rhombus(defender_boma, true).add(2);
        let ecb_right =  *GroundModule::get_rhombus(defender_boma, true).add(3);
        let y_chara_speed = 0.0;
        let air_accel_y = WorkModule::get_param_float(defender_boma, hash40("air_accel_y"), hash40(""));
        let hitstun_gravity_min = ParamModule::get_float((*defender_boma).object(), ParamType::Common, "hitstun_gravity_min");
        let hitstun_gravity_max = ParamModule::get_float((*defender_boma).object(), ParamType::Common, "hitstun_gravity_max");
        let gravity = air_accel_y.clamp(hitstun_gravity_min, hitstun_gravity_max);
        let fall_speed = WorkModule::get_param_float(defender_boma, hash40("air_speed_y_stable"), hash40(""));
        let pos = Vector2f::new(ecb_bottom.x, ecb_bottom.y);
        let pos_prev = Vector2f::new(pos.x, pos.y);
        let damage_air_brake = WorkModule::get_param_float(defender_boma, hash40("common"), hash40("damage_air_brake"));
        let speed_up_mul = if WorkModule::is_flag(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP) {
            WorkModule::get_float(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_SPEED_UP_MAX_MAG)
        } else {
            1.0
        };
        let sdi_frame =     WorkModule::get_param_int(defender_boma, hash40("common"), hash40("hit_stop_delay_flick_frame"));
        let sdi_max_count = WorkModule::get_param_int(defender_boma, hash40("common"), hash40("hit_stop_delay_flick_max_count"));
        let base_sdi =      WorkModule::get_param_float(defender_boma, hash40("common"), hash40("hit_stop_delay_flick_mul"));
        let base_asdi =     WorkModule::get_param_float(defender_boma, hash40("common"), hash40("hit_stop_delay_auto_mul"));
        let hitlag_max =    WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("hitstop_frame_max"));
        let hitlag_add =    WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("hitstop_frame_add"));
        let hitlag_mul =    WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("hitstop_frame_mul"));
        let hitlag = (2.0 * (damage * hitlag_mul + hitlag_add)).clamp(0.0, hitlag_max).floor();
        let sdi_count = ((hitlag - 1.0) / (sdi_frame as f32)).clamp(0.0, sdi_max_count as f32).floor();
        let sdi_distance = (sdi_count * base_sdi + base_asdi) * sdi_mul;
        let is_tech_possible = false;
        let mut context = Self {
            defender_boma,
            fly_top_angle_lw,
            fly_top_angle_hi,
            ecb_bottom,
            ecb_left,
            ecb_right,
            knockback,
            hitstun,
            damage,
            sdi_mul,
            base_asdi,
            sdi_distance,
            launch_radians,
            launch_speed,
            y_chara_speed,
            is_tumble,
            gravity,
            fall_speed,
            pos,
            pos_prev,
            damage_air_brake,
            speed_up_mul,
            is_tech_possible,
        };
        return context;
    }

    pub unsafe fn reset_angle(&mut self, launch_radians: f32) {
        // calculate values that depend on the new angle
        let mag = (self.launch_speed.y.powi(2) + self.launch_speed.x.powi(2)).sqrt();
        let launch_speed = Vector2f::new(
            launch_radians.cos() * mag,
            launch_radians.sin() * mag,
        );
        // update the context
        self.launch_radians = launch_radians;
        self.launch_speed = launch_speed;
    }

    pub unsafe fn initial_launch_collision_check(&mut self) {
        let defender_boma = self.defender_boma;

        // check left wall tech
        let ecb_offset = self.ecb_left.x - self.ecb_bottom.x;
        if GroundModule::ray_check(
            defender_boma, 
            &self.pos, 
            &Vector2f{ x: -1.0 * self.sdi_distance + ecb_offset, y: 0.0},
            true
        ) == 1 {
            self.is_tech_possible = true;
            return;
        }

        // check right wall tech
        let ecb_offset = self.ecb_right.x - self.ecb_bottom.x;
        if GroundModule::ray_check(
            defender_boma, 
            &self.pos, 
            &Vector2f{ x: self.sdi_distance + ecb_offset, y: 0.0},
            true
        ) == 1 {
            self.is_tech_possible = true;
            return;
        }

        // check floor tech
        if self.pos.y - self.pos_prev.y < self.base_asdi * self.sdi_mul
        && GroundModule::ray_check(
            defender_boma, 
            &self.pos, 
            &Vector2f{ x: 0.0, y: self.sdi_distance},
            true
        ) == 1 {
            self.is_tech_possible = true;
            return;
        }
    }

    pub unsafe fn collision_check(&mut self) {
        let defender_boma = self.defender_boma;
        let diff = Vector2f::new(self.pos.x - self.pos_prev.x, self.pos.y - self.pos_prev.y);
        if GroundModule::ray_check(
            defender_boma, 
            &self.pos_prev, 
            &diff, 
            diff.y <= 0.0 // only check for platforms if going downwards
        ) == 1 {
            self.is_tech_possible = true;
            return;
        }
    }

    pub unsafe fn step(&mut self) {
        let kb_angle = self.launch_speed.y.atan2(self.launch_speed.x);
        let decay = Vector2f::new(
            self.damage_air_brake * kb_angle.cos().abs(),
            self.damage_air_brake * kb_angle.sin().abs()
        );

        self.pos_prev.x = self.pos.x;
        self.pos_prev.y = self.pos.y;
        self.pos.x += self.launch_speed.x;
        self.pos.y += self.launch_speed.y + self.y_chara_speed;
        if (self.launch_speed.x != 0.0) {
            let dir = f32::signum(self.launch_speed.x);
            self.launch_speed.x = f32::abs(self.launch_speed.x) - decay.x;
            if (self.launch_speed.x < 0.0) {
                self.launch_speed.x = 0.0;
            } else {
              self.launch_speed.x *= dir;
            }
        }

        if (self.launch_speed.y != 0.0) {
            let dir = f32::signum(self.launch_speed.y);
            self.launch_speed.y = f32::abs(self.launch_speed.y) - decay.y;
            if (self.launch_speed.y < 0.0) {
                self.launch_speed.y = 0.0;
            } else {
                self.launch_speed.y *= dir;
            }
        }
        self.y_chara_speed = f32::max(self.y_chara_speed - self.gravity, -self.fall_speed);
    }

    pub unsafe fn get_trajectory(&mut self) -> Vec<Vector2f> {
        let mut trajectory = Vec::new();
        for i in 0..self.hitstun.floor() as i32 {
            trajectory.push(Vector2f::new(self.pos.x, self.pos.y));
            self.step();
            if i == 0 {
                self.initial_launch_collision_check();
            }
            self.collision_check();
            if self.is_tech_possible {
                break;
            }
        }
        return trajectory;
    }

    pub unsafe fn is_finishing_hit(&mut self, is_final: bool) -> bool {
        let defender_boma = self.defender_boma;
        let (num_angles_checked, survivable_angles_allowed, dead_area_leniency_x, dead_area_leniency_y) = if is_final {
            (NUM_ANGLES_CHECKED_FINAL, SURVIVABLE_ANGLES_ALLOWED_FINAL, DEAD_AREA_LENIENCY_FINAL, DEAD_AREA_LENIENCY_FINAL)
        }  else {
            let x = DEAD_AREA_LENIENCY.max(self.sdi_distance);
            let y = if StatusModule::situation_kind(defender_boma) != *SITUATION_KIND_GROUND {
                DEAD_AREA_LENIENCY.max(self.sdi_distance)
            } else {
                DEAD_AREA_LENIENCY
            };
            (NUM_ANGLES_CHECKED, SURVIVABLE_ANGLES_ALLOWED, x, y)
        };
        let mut blastzones = get_dead_area();
        blastzones.grow(dead_area_leniency_x, dead_area_leniency_y);
        let kb_angle = self.launch_speed.y.atan2(self.launch_speed.x).to_degrees();
        let di_angle = WorkModule::get_param_float(defender_boma, hash40("common"), hash40("damage_fly_correction_max"));
        let min_di = kb_angle - di_angle;
        let max_di = kb_angle + di_angle;
        let step = (di_angle * 2.0) / (num_angles_checked as f32);
        let mut survivable_angles = 0;
        let original_context = self.clone();
        for idx in 0..num_angles_checked + 1 {
            // calc and update the DI angle
            let new_radians = (min_di + (idx as f32 * step)).to_radians();
            
            // reset everything to scratch
            *self = original_context.clone();
            self.reset_angle(new_radians);

            // check if it kills at this angle
            let trajectory = self.get_trajectory();
            let mut trajectory_kills = false;
            for (frame, pos) in trajectory.iter().enumerate() {
                if !blastzones.contains(pos.x, pos.y) {
                    // break early so we don't waste effort
                    trajectory_kills = true;
                    break;
                }
            }
            if !trajectory_kills {
                survivable_angles += 1;
            }
            if survivable_angles > survivable_angles_allowed {
                // return early so we don't waste effort
                return false;
            }
        }
        return true;
    }
}