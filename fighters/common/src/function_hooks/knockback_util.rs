use super::*;
use utils::ext::*;

extern "C" {
    #[link_name = "_ZN3app6camera13get_dead_areaEv"]
    fn get_dead_area() -> Rect;
}

#[repr(simd)]
#[derive(Clone, Copy)]
struct Rect {
    // left: f32,
    // right: f32,
    // top: f32,
    // bottom: f32,
    vec: [f32; 4]
}

impl Rect {
    pub fn x(self) -> f32 {
        unsafe { core::intrinsics::simd::simd_extract(self, 0) }
    }
    pub fn set_x(&mut self, value: f32) {
        unsafe {
            *self = core::intrinsics::simd::simd_insert(*self, 0, value);
        }
    }
    pub fn y(self) -> f32 {
        unsafe { core::intrinsics::simd::simd_extract(self, 1) }
    }
    pub fn set_y(&mut self, value: f32) {
        unsafe {
            *self = core::intrinsics::simd::simd_insert(*self, 1, value);
        }
    }
    pub fn z(self) -> f32 {
        unsafe { core::intrinsics::simd::simd_extract(self, 2) }
    }
    pub fn set_z(&mut self, value: f32) {
        unsafe {
            *self = core::intrinsics::simd::simd_insert(*self, 2, value);
        }
    }
    pub fn w(self) -> f32 {
        unsafe { core::intrinsics::simd::simd_extract(self, 3) }
    }
    pub fn set_w(&mut self, value: f32) {
        unsafe {
            *self = core::intrinsics::simd::simd_insert(*self, 3, value);
        }
    }
    fn contains(&self, x: f32, y: f32) -> bool {
        (self.x() <= x && x <= self.y()) && (self.w() <= y && y <= self.z())
    }
    fn grow(&mut self, x: f32, y: f32) {
        self.set_x(self.x() - x);
        self.set_y(self.y() + x);
        self.set_z(self.z() + y);
        self.set_w(self.w() - y);
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

pub unsafe fn rotate_vector2f(
    vec: Vector2f,
    degrees: f32
) -> Vector2f {
    let mag = (vec.x.powi(2) + vec.y.powi(2)).sqrt();
    let curr_radians = vec.y.atan2(vec.x);
    let new_radians = curr_radians + degrees.to_radians();
    return Vector2f::new(
        new_radians.cos() * mag,
        new_radians.sin() * mag
    );
}

// the blastzones are effectively expanded by this amount when performing kill calculations
const DEAD_AREA_LENIENCY: f32 = 7.5;
const DEAD_AREA_LENIENCY_FINAL: f32 = -2.5;

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
        let fly_top_angle_lw = WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("fly_top_angle_lw"));
        let fly_top_angle_hi = WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("fly_top_angle_hi"));
        let ecb_bottom = *GroundModule::get_rhombus(defender_boma, true).add(1);
        let ecb_left = *GroundModule::get_rhombus(defender_boma, true).add(2);
        let ecb_right = *GroundModule::get_rhombus(defender_boma, true).add(3);
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
        let sdi_frame = WorkModule::get_param_int(defender_boma, hash40("common"), hash40("hit_stop_delay_flick_frame"));
        let sdi_max_count = WorkModule::get_param_int(defender_boma, hash40("common"), hash40("hit_stop_delay_flick_max_count"));
        let base_sdi = WorkModule::get_param_float(defender_boma, hash40("common"), hash40("hit_stop_delay_flick_mul"));
        let base_asdi = WorkModule::get_param_float(defender_boma, hash40("common"), hash40("hit_stop_delay_auto_mul"));
        let hitlag_max = WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("hitstop_frame_max"));
        let hitlag_add = WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("hitstop_frame_add"));
        let hitlag_mul = WorkModule::get_param_float(defender_boma, hash40("battle_object"), hash40("hitstop_frame_mul"));
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
        let launch_speed = Vector2f::new(launch_radians.cos() * mag, launch_radians.sin() * mag);
        // update the context
        self.launch_radians = launch_radians;
        self.launch_speed = launch_speed;
    }

    pub unsafe fn initial_launch_collision_check(&mut self) {
        let defender_boma = self.defender_boma;

        // check left wall tech
        if -1.0 * self.launch_speed.x <= self.base_asdi * self.sdi_mul
        && GroundModule::ray_check(
            defender_boma,
            &Vector2f::new(self.ecb_left.x, self.ecb_left.y),
            &Vector2f::new(-1.0 * self.sdi_distance, 0.0),
            true,
        ) == 1
        {
            self.is_tech_possible = true;
            return;
        }

        // check right wall tech
        if self.launch_speed.x <= self.base_asdi * self.sdi_mul
        && GroundModule::ray_check(
            defender_boma,
            &Vector2f::new(self.ecb_right.x, self.ecb_right.y),
            &Vector2f::new(self.sdi_distance, 0.0),
            true,
        ) == 1
        {
            self.is_tech_possible = true;
            return;
        }

        // check floor tech
        if self.launch_speed.y <= self.base_asdi * self.sdi_mul
        && GroundModule::ray_check(
            defender_boma,
            &Vector2f::new(self.ecb_bottom.x, self.ecb_bottom.y),
            &Vector2f::new(0.0, -1.0 * self.sdi_distance),
            true,
        ) == 1
        {
            self.is_tech_possible = true;
            return;
        }

    }

    pub unsafe fn collision_check(&mut self) {
        // calculate the offsets including ecb size and possible SDI distance
        let ecb_width = f32::abs(self.ecb_right.x - self.ecb_left.x);
        let ecb_height = f32::abs(f32::midpoint(self.ecb_left.y, self.ecb_right.y) - self.ecb_bottom.y);
        let x_offset = self.sdi_distance + (0.5 * ecb_width);
        let y_offset = self.sdi_distance;

        // calculate necessary ray checks
        let max_left_prev = Vector2f::new(self.pos_prev.x - x_offset, self.pos_prev.y + ecb_height);
        let max_left_curr = Vector2f::new(self.pos.x - x_offset, self.pos.y + ecb_height);
        let max_right_prev = Vector2f::new(self.pos_prev.x + x_offset, self.pos_prev.y + ecb_height);
        let max_right_curr = Vector2f::new(self.pos.x + x_offset, self.pos.y + ecb_height);
        let max_down_prev = Vector2f::new(self.pos_prev.x, self.pos_prev.y - y_offset);
        let max_down_curr = Vector2f::new(self.pos.x, self.pos.y - y_offset);
        let ray_checks = [
            (self.pos_prev, self.pos),
            (max_left_prev, max_left_curr),
            (max_right_prev, max_right_curr),
            (max_down_prev, max_down_curr),
        ];

        // perform a ray check between each point to draw a clear bounding box
        let defender_boma = self.defender_boma;
        for ray_check in ray_checks {
            let prev = ray_check.0;
            let curr = ray_check.1;
            let diff = Vector2f::new(curr.x - prev.x, curr.y - prev.y);
            if GroundModule::ray_check(
                defender_boma,
                &prev,
                &diff,
                diff.y <= 0.0, // only check for platforms if going downwards
            ) == 1
            {
                self.is_tech_possible = true;
                return;
            }
        }
    }

    pub unsafe fn step(&mut self) {
        let kb_angle = self.launch_speed.y.atan2(self.launch_speed.x);
        let decay = Vector2f::new(self.damage_air_brake * kb_angle.cos().abs(), self.damage_air_brake * kb_angle.sin().abs());

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

        // get blastzones, and grow them to build in a minumum kill threshold
        let sdi_distance = self.sdi_distance;
        let dead_area_leniency = if is_final {
            DEAD_AREA_LENIENCY_FINAL + self.sdi_distance
        } else {
            DEAD_AREA_LENIENCY + self.sdi_distance
        };
        let mut blastzones = get_dead_area();
        blastzones.grow(dead_area_leniency, dead_area_leniency);

        // calculate the different angles we will be checking
        let base_radians = self.launch_speed.y.atan2(self.launch_speed.x);
        let di_radians = WorkModule::get_param_float(defender_boma, hash40("common"), hash40("damage_fly_correction_max")).to_radians();
        let angles = [
            // check max DI angles first, since theyre more likely to be surviveable
            // and if any angle is surviveable, we skip the others angle checks
            base_radians + (di_radians * 1.00), // max left
            base_radians - (di_radians * 1.00), // max right
            base_radians,
            // check precise angles later
            base_radians + (di_radians * 0.66),
            base_radians + (di_radians * 0.33),
            base_radians - (di_radians * 0.33),
            base_radians - (di_radians * 0.66),
        ];

        // perform the trajectory check over each angle
        let original_context = self.clone();
        for angle in angles.iter() {
            // get the trajectory for this angle
            *self = original_context.clone();
            self.reset_angle(*angle);
            let trajectory = self.get_trajectory();

            // check if this trajectory kills
            let mut trajectory_kills = false;
            for (frame, pos) in trajectory.iter().enumerate() {
                if !blastzones.contains(pos.x, pos.y) {
                    trajectory_kills = true;
                    break;
                }
            }

            // if any single trajectory is survivable, no kill screen
            if !trajectory_kills {
                return false;
            }
        }

        // on this line, all trajectories killed
        return true;
    }
}
