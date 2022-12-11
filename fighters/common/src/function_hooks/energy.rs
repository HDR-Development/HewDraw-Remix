use super::*;
use crate::consts::*;
use crate::consts::globals::*;

#[repr(C)]
pub struct KineticEnergyVTable {
    pub destructor: extern "C" fn(&mut KineticEnergy),
    pub deleter: extern "C" fn(*mut KineticEnergy),
    pub unk: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub update: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub get_speed: extern "C" fn(&mut KineticEnergy) -> *mut PaddedVec2,
    pub initialize: extern "C" fn(&mut KineticEnergy, &mut BattleObjectModuleAccessor),
    pub get_some_flag: extern "C" fn(&mut KineticEnergy) -> bool,
    pub set_some_flag: extern "C" fn(&mut KineticEnergy, bool),
    pub setup_energy: extern "C" fn(&mut KineticEnergy, u32, &Vector3f, u64, &mut BattleObjectModuleAccessor),
    pub clear_energy: extern "C" fn(&mut KineticEnergy),
    pub unk2: extern "C" fn(&mut KineticEnergy),
    pub set_speed: extern "C" fn (&mut KineticEnergy, &Vector2f),
    pub mul_accel: extern "C" fn(&mut KineticEnergy, &Vector2f),
    // ...

}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct PaddedVec2 {
    pub x: f32,
    pub y: f32,
    pub padding: u64
}

impl PaddedVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            padding: 0
        }
    }

    pub fn zeros() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            padding: 0
        }
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[repr(C)]
pub struct KineticEnergy {
    pub vtable: &'static KineticEnergyVTable,
    pub _x8: u64, // probably padding
    pub speed: PaddedVec2,
    pub rot_speed: PaddedVec2,
    pub enable: bool,
    pub unk2: [u8; 0xF], // probably padding 
    pub accel: PaddedVec2,
    pub speed_max: PaddedVec2,
    pub speed_brake: PaddedVec2,
    pub speed_limit: PaddedVec2,
    pub _x80: u8,
    pub consider_ground_friction: bool,
    pub active_flag: bool, // no clue?
    pub _x83: u8,
    pub energy_reset_type: u32,
}


#[repr(simd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

#[repr(simd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[repr(simd)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,

}

impl KineticEnergy {
    pub fn adjust_speed_for_ground_normal(speed: &PaddedVec2, boma: &mut BattleObjectModuleAccessor) -> PaddedVec2 {
        #[skyline::from_offset(0x47b4d0)]        
        extern "C" fn adjust_speed_for_ground_normal_internal(speed: Vec2, boma: &mut BattleObjectModuleAccessor) -> Vec2;

        unsafe {
            let result = adjust_speed_for_ground_normal_internal(Vec2 { x: speed.x, y: speed.y }, boma);
            PaddedVec2::new(result.x, result.y)
        }
    }

    pub fn process(&mut self, boma: &mut BattleObjectModuleAccessor) {
        unsafe {
            #[skyline::from_offset(0x47bf70)]
            extern "C" fn process_energy(energy: &mut KineticEnergy, boma: &mut BattleObjectModuleAccessor);

            process_energy(self, boma)
        }
    }

    pub fn update(&mut self, boma: &mut BattleObjectModuleAccessor) {
        unsafe {
            (self.vtable.update)(self, boma)
        }
    }

    pub fn get_speed<'a>(&'a mut self) -> &'a mut PaddedVec2 {
        unsafe {
            std::mem::transmute((self.vtable.get_speed)(self))
        }
    }

    pub fn initialize(&mut self, boma: &mut BattleObjectModuleAccessor) {
        unsafe {
            (self.vtable.initialize)(self, boma)
        }
    }

    pub fn get_some_flag(&mut self) -> bool {
        unsafe {
            (self.vtable.get_some_flag)(self)
        }
    }

    pub fn set_some_flag(&mut self, flag: bool) {
        unsafe {
            (self.vtable.set_some_flag)(self, flag)
        }
    }

    pub fn setup_energy(&mut self, reset_type: u32, incoming_speed: &Vector3f, some: u64, boma: &mut BattleObjectModuleAccessor) {
        unsafe {
            (self.vtable.setup_energy)(self, reset_type, incoming_speed, some, boma)
        }
    }

    pub fn clear_energy(&mut self) {
        unsafe {
            (self.vtable.clear_energy)(self)
        }
    }

    pub fn unk2(&mut self) {
        unsafe {
            (self.vtable.unk2)(self)
        }
    }

    pub fn set_speed(&mut self, speed: &Vector2f) {
        unsafe {
            (self.vtable.set_speed)(self, speed)
        }
    }

    pub fn mul_accel(&mut self, mul: &Vector2f) {
        unsafe {
            (self.vtable.mul_accel)(self, mul)
        }
    }

}

#[repr(C)]
struct FlyData {
    pub turn_stick_x: f32,
    pub init_speed_x_mul: f32,
    pub speed_x_mul: f32,
    pub speed_x_max_mul: f32,
    pub speed_y_table_start: *const f32,
    pub speed_y_table_end: *const f32,
    pub speed_y_table_eos: *const f32,
    pub turn_param_start: *const i32,
    pub turn_param_end: *const i32,
    pub turn_param_eos: *const i32,
    pub shoot_fly_next_frame: i32
}

impl FlyData {
    pub fn get_from_fighter_kind(kind: i32) -> Option<&'static Self> {
        #[repr(C)]
        struct FlyDataResult {
            vtable: *const *const (),
            data: *const *const FlyData
        }

        unsafe {
            let accessor = *((utils::singletons::FighterParamAccessor2() as *const u8).add((kind as usize) * 0x38 + 0x70) as *const u64);
            let function: extern "C" fn(u64, u64) -> FlyDataResult = std::mem::transmute(*(*(accessor as *const *const u64)).add(0x2));
            let result = function(accessor, smash::hash40("fly_data"));
            if (*result.data).is_null() {
                return None;
            } else {
                return Some(&**result.data);
            }
        }

    }
}

use std::ops::{Deref, DerefMut};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EnergyControllerResetType {
    FallAdjust = 0x0,
    FallAdjustNoCap,
    StopCeil,
    WallJump,
    FlyAdjust,
    Dash,
    ShootDash,
    ShootBackDash,
    TurnRun,
    RevolveSlashAir,
    Turn,
    Free,
    FreeTest,
    ItemLift,
    SwimRise,
    Swim,
    SwimDrown,
    MoveGround,
    MoveAir,
    TurnNoStop,
    TurnNoStopAir,
    Ladder,
    DashBack,
}

#[repr(C)]
pub struct FighterKineticEnergyControl {
    parent: super::energy::KineticEnergy,
    pub lr: f32,
    pub accel_mul_x: f32,
    pub accel_add_x: f32,
    pub accel_mul_y: f32,
    pub accel_add_y: f32,
    pub _x9c: f32,
    pub _xa0: f32,
    pub unk: [u8; 4]
}

impl Deref for FighterKineticEnergyControl {
    type Target = super::energy::KineticEnergy;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterKineticEnergyControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[skyline::hook(offset = 0x6d3610)]
unsafe fn update(energy: &mut FighterKineticEnergyControl, boma: &mut BattleObjectModuleAccessor) {
    let reset_type = std::mem::transmute(energy.energy_reset_type);

    let mut stick = if boma.is_button_on(Buttons::CStickOverride) {
        Vector2f {
            x: ControlModule::get_sub_stick_x(boma),
            y: ControlModule::get_sub_stick_y(boma)
        }
    } else {
        Vector2f {
            x: ControlModule::get_stick_x(boma),
            y: ControlModule::get_stick_y(boma)
        }
    };

    let backup_max = energy.speed_max;
    let backup_brake = energy.speed_brake;

    if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0 {
        stick.x = 0.0;
    }

    let accel_add_x = if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    && WorkModule::is_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE)
    && !WorkModule::is_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL)
    {
        stick.x = 0.0;
        0.0
    } else if stick.x == 0.0 {
        0.0
    } else {
        energy.accel_add_x
    };

    let accel_add_y = if stick.y != 0.0 {
        energy.accel_add_y
    } else {
        0.0
    };

    let mut change_y = energy.accel.y;

    use EnergyControllerResetType::*;

    let mut do_standard_accel = true;

    let accel_diff = match reset_type {
        FallAdjust | FallAdjustNoCap | FlyAdjust | ShootDash | ShootBackDash | RevolveSlashAir | MoveGround | MoveAir => {
            accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x
        },
        WallJump => {
            if WorkModule::get_int(boma, *FIGHTER_STATUS_WALL_JUMP_WORK_INT_DISABLE_CONT_FRAME) == 0 {
                accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x
            } else {
                0.0
            }
        },
        Turn => {
            stick.x = 0.0;
            // Perfect Pivot
            if VarModule::is_flag(boma.object(), vars::common::instance::IS_SMASH_TURN)
            && VarModule::is_flag(boma.object(), vars::common::instance::CAN_PERFECT_PIVOT)
            && ControlModule::get_stick_x(boma).abs() < WorkModule::get_param_float(boma, hash40("common"), hash40("dash_stick_x")) {
                VarModule::off_flag(boma.object(), vars::common::instance::CAN_PERFECT_PIVOT);
                do_standard_accel = false;
                energy.speed.x = -energy.lr * WorkModule::get_param_float(boma, smash::hash40("dash_speed"), 0) * ParamModule::get_float(boma.object(), ParamType::Common, "perfect_pivot_speed_mul");
                0.0
            } else {
                0.0
            }
        },
        Dash | DashBack => loop {
            // Don't apply or change the speed by any ammount during the first keep frames of dash
            let keep_frame = WorkModule::get_param_int(boma, smash::hash40("common"), smash::hash40("dash_speed_keep_frame"));
            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DASH || reset_type == DashBack {
                if WorkModule::get_int(boma, *FIGHTER_STATUS_DASH_WORK_INT_COUNT) < keep_frame {
                    energy.speed_max.x = 0.0;
                    energy.speed_brake.x = 0.0;
                    stick.x = accel_add_x; // not sure if this is accurate but it's what I think I saw in the code
                    break 0.0;
                }
            } else if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_TURN_DASH {
                if WorkModule::get_int(boma, *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FROM_DASH_COUNT) < keep_frame {
                    energy.speed_max.x = 0.0;
                    energy.speed_brake.x = 0.0;
                    stick.x = accel_add_x;
                    break 0.0;
                }
            }

            // Late pivot
            if reset_type == Dash
            && boma.status_frame() == 0
            && ControlModule::get_stick_x(boma).abs() < WorkModule::get_param_float(boma, hash40("common"), hash40("dash_stick_x"))
            && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_TURN 
            && StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_DASH { // if you are in a backdash
                energy.speed.x = if VarModule::is_flag(boma.object(), vars::common::instance::CAN_PERFECT_PIVOT) {
                    // Late Perfect Pivot
                    energy.lr * WorkModule::get_param_float(boma, smash::hash40("dash_speed"), 0) * ParamModule::get_float(boma.object(), ParamType::Common, "perfect_pivot_speed_mul")
                } else {
                    0.0
                };
                break 0.0;
            }
            // Dashback
            let dashback_input = if reset_type == Dash {
                ControlModule::get_stick_x(boma) * PostureModule::lr(boma) <= ParamModule::get_float(boma.object(), ParamType::Common, "dashback_stick_x")
                && ControlModule::get_flick_x(boma) <= WorkModule::get_param_int(boma, hash40("common"), hash40("dash_flick_x"))
            } else {
                ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0
            };
            // Disables dashbacks when stick falls below threshold
            // For ease of moonwalking
            let moonwalk_disable_dashback_stick_y = ParamModule::get_float(boma.object(), ParamType::Common, "moonwalk_disable_dashback_stick_y");
            if stick.y <= moonwalk_disable_dashback_stick_y
            && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) {
                WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
            }
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH)
            && dashback_input {
                energy.speed.x *= ParamModule::get_float(boma.object(), ParamType::Common, "dash_end_speed_mul");
                break 0.0;
            }
            // Shield Stop
            if boma.is_pad_flag(PadFlag::GuardTrigger) && boma.is_button_off(Buttons::Catch) {
                energy.speed.x = 0.0;
                break 0.0;
            }

            // accel add
            break stick.x * energy.accel_mul_x + stick.x.signum() * energy.accel_add_x;
        },
        TurnRun => {
            let mut mul = stick.x * energy.accel_mul_x + accel_add_x * stick.x.signum();
            let mut brake = WorkModule::get_param_float(boma, smash::hash40("ground_brake"), 0)
                                    * WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("run_brake_brake_mul"));
            
            let ground_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88 / 0x8);
            let some_float = *(ground_module as *const f32).add(0x130 / 0x8);
            if some_float * energy.lr <= -0.1 {
                let turn_run_brake = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("turn_run_stop_brake_mul"));
                mul *= turn_run_brake;
                brake *= turn_run_brake;
            }
            energy.speed_brake.x = brake;
            if 0.0 <= mul * energy.lr {
                // If stick is at neutral
                do_standard_accel = false;
                energy.accel.x = 0.0;
                energy.accel.y = change_y;
                energy.speed_max.x = 0.0;
                0.0
            } else {
                if energy.speed.x.abs() >= energy.speed_max.x
                && mul.abs() > brake {
                    // Boost run
                    do_standard_accel = false;
                    energy.accel.x = mul - (brake * mul.signum());
                    energy.speed_max.x = -1.0;
                    0.0
                } else {
                    mul
                }
            }
        },
        Free => {
            change_y = accel_add_y * stick.y.signum() + stick.y * energy.accel_mul_y;
            energy.speed_max.y = stick.y * energy.speed_max.y;
            accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x
        },
        ItemLift => loop {
            if WorkModule::is_flag(boma, *FIGHTER_STATUS_ITEM_LIFT_WORK_FLAG_STOP) {
                stick.x = 0.0;
                break 0.0;
            }

            if accel_add_x * PostureModule::lr(boma) <= 0.0 {
                energy.speed_max.x = 0.0;
                energy.speed_brake.x = 0.0;
                stick.x = accel_add_x; // not sure if this is accurate but it's what I think I saw in the code
                break 0.0;
            }

            let stick_rate = WorkModule::get_float(boma, *FIGHTER_STATUS_ITEM_LIFT_WORK_FLOAT_STICK_RATE);
            energy.speed_max.x *= stick_rate;
            break (accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x) * stick_rate;
        },
        Swim => {
            let speed_mul = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("swim_speed_mul"));
            energy.speed_max.x = stick.x.abs() * speed_mul;
            energy.speed_max.y = -1.0;
            accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x
        },
        SwimDrown => {
            let speed_mul = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("swim_drown_speed_x_mul"))
                                    * WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("swim_speed_mul")); 
            energy.speed_max.x = stick.x * speed_mul;
            energy.speed_max.y = -1.0;
            accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x
        },
        TurnNoStop | TurnNoStopAir => {
            if reset_type == TurnNoStop || reset_type == TurnNoStopAir {
                if (!TurnModule::is_turn(boma) || energy.accel_mul_x == 0.0) && energy.speed.x == 0.0 {
                    energy.parent.enable = false;
                    return;
                }
                if ControlModule::reverse_x_frame(boma) != 0 {
                    stick.x = -stick.x;
                }
            }

            (accel_add_x * stick.x.signum() + stick.x * energy.accel_mul_x) * WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("turn_speed_mul"))
        },
        Ladder => {
            let ladder_y = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ladder_stick_y"));
            let (speed_max, accel_y) = if ladder_y <= stick.y.abs() {
                if stick.y <= 0.0 {
                    let down_max = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ladder_speed_d_max"));
                    // lerp the down_max
                    let down_max = ((stick.y.abs() - ladder_y) / (1.0 - ladder_y)) * down_max;
                    let attack_mul = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ladder_attack_speed_mul"));
                    (down_max * attack_mul, -down_max * attack_mul)
                } else {
                    let up_max = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ladder_speed_d_max"));
                    // lerp the down_max
                    let up_max = ((stick.y - ladder_y) / (1.0 - ladder_y)) * up_max;
                    let attack_mul = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ladder_attack_speed_mul"));
                    (up_max * attack_mul, up_max * attack_mul)
                }
            } else {
                (0.0, 0.0)
            };
            do_standard_accel = false;
            energy.accel.x = 0.0;
            energy.accel.y = accel_y;
            energy.speed_max.x = 0.0;
            energy.speed_max.y = speed_max;
            0.0
        }
        _ => 0.0
    };

    if do_standard_accel {
        energy.accel.x = accel_diff;
        energy.speed_max.x *= stick.x.abs();
    
        if energy.unk[1] != 0 {
            if !(((energy._x9c != 0.0 && (stick.x <= 0.0 || energy._xa0 <= 0.0 || energy.speed_max.x.abs() <= energy._x9c.abs()))
            && (stick.x >= 0.0 || energy._xa0 >= 0.0 || energy.speed_max.x.abs() <= energy._x9c.abs()))
            && ((stick.x <= 0.0 || 0.0 <= energy._xa0) && (0.0 <= stick.x || energy._xa0 <= 0.0)))
            {
                energy._x9c = energy.speed_max.x;
                energy._xa0 = stick.x;
            }

        }
    }

    energy.process(boma);

    let status_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x8);
    if !*(status_module as *const bool).add(0x12a) {
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
            let horizontal_limit = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("common_air_speed_x_limit"));
            let vertical_limit = if energy.speed.y <= 0.0 {
                WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("air_speed_down_limit"))
            } else {
                WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("air_speed_up_limit"))
            };

            if horizontal_limit < energy.speed.x.abs() {
                energy.speed.x = vertical_limit * energy.speed.x.signum();
            }

            if vertical_limit < energy.speed.y.abs() {
                energy.speed.y = vertical_limit * energy.speed.y.signum();
            }
        } else if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
            let speed_limit = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ground_speed_limit"));
            if speed_limit < energy.speed.x.abs() {
                energy.speed.x = speed_limit * energy.speed.x.signum();
            }
        }
    }

    energy.speed_max = backup_max;
    energy.speed_brake = backup_brake;
}

#[skyline::hook(offset = 0x6d4040)]
unsafe fn initialize(energy: &mut FighterKineticEnergyControl, boma: &mut BattleObjectModuleAccessor) {
    use EnergyControllerResetType::*;
    let reset_type = std::mem::transmute(energy.energy_reset_type);
    match reset_type {
        FallAdjust | FallAdjustNoCap | StopCeil | WallJump => {
            let mut stable_speed = WorkModule::get_param_float(boma, smash::hash40("air_speed_x_stable"), 0);
            if reset_type == StopCeil {
                stable_speed *= WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("stop_ceil_speed_x_stable_mul"));
            }

            energy.speed_max = PaddedVec2::new(stable_speed, -1.0);
            energy.speed_brake = PaddedVec2::new(WorkModule::get_param_float(boma, smash::hash40("air_brake_x"), 0), 0.0);
            let air_x_speed_max = if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT) && energy.unk[2] == 0 {
                WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("air_speed_x_limit"))
            } else {
                -1.0
            };
            energy.speed_limit = PaddedVec2::new(air_x_speed_max, 0.0);
            energy.accel_mul_x = WorkModule::get_param_float(boma, smash::hash40("air_accel_x_mul"), 0);
            energy.accel_add_x = WorkModule::get_param_float(boma, smash::hash40("air_accel_x_add"), 0);
        },
        FlyAdjust => {
            let kind = app::utility::get_kind(boma);

            let fly_data = if let Some(data) = FlyData::get_from_fighter_kind(kind) {
                data
            } else {
                return;
            };

            let stable = boma.get_param_float("air_speed_x_stable", "");
            let brake = boma.get_param_float("air_brake_x", "");
            let limit = boma.get_param_float("common", "air_speed_x_limit");
            let accel_mul = boma.get_param_float("air_accel_x_mul", "") * fly_data.speed_x_mul;
            let accel_add = boma.get_param_float("air_accel_x_add", "");

            energy.speed_max = PaddedVec2::new(stable, -1.0);
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
            energy.speed_limit = PaddedVec2::new(limit, 0.0);
            energy.accel_mul_x = accel_mul;
            energy.accel_add_x = accel_add;
        },
        Dash | TurnRun | DashBack => {
            energy.speed_limit = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ground_speed_limit")),
                0.0
            );
            energy.speed_max = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("run_speed_max"), 0),
                -1.0
            );
            let brake = WorkModule::get_param_float(boma, smash::hash40("ground_brake"), 0)
                                * WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("run_brake_brake_mul"));
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
            energy.accel_mul_x = WorkModule::get_param_float(boma, smash::hash40("run_accel_mul"), 0);
            energy.accel_add_x = WorkModule::get_param_float(boma, smash::hash40("run_accel_add"), 0);
        },
        ShootDash | ShootBackDash => {
            energy.speed_limit = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ground_speed_limit")),
                0.0
            );
            energy.speed_max = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("run_speed_max"), 0),
                -1.0
            );
            let brake = WorkModule::get_param_float(boma, smash::hash40("ground_brake"), 0)
                                * WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("run_brake_brake_mul"));
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
        },
        RevolveSlashAir => {
            let speed_max = WorkModule::get_param_float(boma, smash::hash40("air_speed_x_stable"), 0)
                                    * WorkModule::get_param_float(boma, smash::hash40("param_special_hi"), smash::hash40("rslash_air_max_x_mul"));

            energy.speed_max = PaddedVec2::new(speed_max, -1.0);
            energy.speed_brake = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("air_brake_x"), 0),
                0.0
            );
            energy.speed_limit = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("air_speed_x_limit")),
                0.0
            );
            energy.accel_mul_x = WorkModule::get_param_float(boma, smash::hash40("param_special_hi"), smash::hash40("rslash_air_max_x_mul"));
        },
        Turn | TurnNoStop => {
            energy.speed_max = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("run_speed_max"), 0),
                -1.0
            );
            energy.speed_limit = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ground_speed_limit")),
                0.0
            );
            let brake = WorkModule::get_param_float(boma, smash::hash40("ground_brake"), 0)
                                * WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("run_brake_brake_mul"));
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
            energy.accel_mul_x = WorkModule::get_param_float(boma, smash::hash40("walk_accel_mul"), 0);
            energy.accel_add_x = WorkModule::get_param_float(boma, smash::hash40("walk_accel_add"), 0);
        },
        Free => {
            let speed_max = WorkModule::get_param_float(boma, smash::hash40("air_speed_x_stable"), 0);
            let speed_brake = WorkModule::get_param_float(boma, smash::hash40("air_brake_x"), 0);
            let speed_limit = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("air_speed_x_limit"));
            let mul = WorkModule::get_param_float(boma, smash::hash40("air_accel_x_mul"), 0);
            let add = WorkModule::get_param_float(boma, smash::hash40("air_accel_x_add"), 0);
            energy.speed_max = PaddedVec2::new(speed_max, speed_max);
            energy.speed_brake = PaddedVec2::new(speed_brake, speed_brake);
            energy.speed_limit = PaddedVec2::new(speed_limit, speed_limit);
            energy.accel_mul_x = mul;
            energy.accel_mul_y = mul;
            energy.accel_add_x = add;
            energy.accel_add_y = add;
        },
        ItemLift => {
            let scale = PostureModule::scale(boma);
            energy.speed_max = PaddedVec2::new(
                scale * WorkModule::get_param_float(boma, smash::hash40("item_lift_speed_max"), 0),
                -1.0
            );
            energy.speed_limit = PaddedVec2::new(
                scale * WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ground_speed_limit")),
                0.0
            );
            let brake = WorkModule::get_param_float(boma, smash::hash40("ground_brake"), 0)
                                * WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("run_brake_brake_mul"));
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
            energy.accel_mul_x = scale * WorkModule::get_param_float(boma, smash::hash40("item_lift_accel_mul"), 0);
            energy.accel_add_x = scale * WorkModule::get_param_float(boma, smash::hash40("item_lift_accel_add"), 0);
        },
        Swim => {
            energy.speed_brake = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("swim_brake")),
                0.0
            );
            energy.accel_mul_x = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("swim_accel_mul"));
        },
        SwimDrown => {
            energy.speed_brake = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("swim_brake")),
                0.0
            );
            energy.accel_mul_x = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("swim_accel_mul"))
                                    * WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("swim_drown_speed_x_mul"));
        },
        TurnNoStopAir => {
            energy.speed_max = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("air_speed_x_stable"), 0),
                -1.0
            );
            energy.speed_limit = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("air_speed_x_limit")),
                0.0
            );
            energy.speed_brake = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("air_brake_x"), 0),
                0.0
            );
            energy.accel_mul_x = WorkModule::get_param_float(boma, smash::hash40("air_accel_x_mul"), 0);
            energy.accel_add_x = WorkModule::get_param_float(boma, smash::hash40("air_accel_x_add"), 0);
        },
        Ladder => {
            let up_speed = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ladder_speed_u_max"));
            let down_speed = WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ladder_speed_d_max"));
            energy.speed_brake = PaddedVec2::new(0.0, up_speed.max(down_speed));
        }
        _ => {}
    }
}

#[skyline::hook(offset = 0x6d4ba0)]
unsafe fn setup(energy: &mut FighterKineticEnergyControl, reset_type: EnergyControllerResetType, initial_speed: &Vector3f, unk: u64, boma: &mut BattleObjectModuleAccessor) {
    energy.clear_energy();

    energy.accel = PaddedVec2::zeros();
    energy.speed_max = PaddedVec2::zeros();
    energy.speed_brake = PaddedVec2::zeros();
    energy.speed_limit = PaddedVec2::new(-1.0, -1.0);
    energy.speed = PaddedVec2::new(initial_speed.x, initial_speed.y);
    energy.energy_reset_type = reset_type as u32;
    energy.accel_mul_x = 0.0;
    energy.accel_add_x = 0.0;
    energy.accel_mul_y = 0.0;
    energy.accel_add_y = 0.0;
    energy.lr = PostureModule::lr(boma);
    energy.unk[3] = 1;
    
    use EnergyControllerResetType::*;
    match reset_type {
        FallAdjust | FallAdjustNoCap | StopCeil | WallJump => {
            energy.unk[2] = if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE) {
                1
            } else {
                0
            };
            if reset_type != FallAdjustNoCap
            && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT)
            && energy.unk[2] == 0 {
                let stable_speed = WorkModule::get_param_float(boma, smash::hash40("air_speed_x_stable"), 0);
                if stable_speed < energy.speed.x.abs() {
                    energy.speed = PaddedVec2::new(stable_speed * energy.speed.x.signum(), 0.0);
                }
            }
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        },
        FlyAdjust => {
            let kind = smash::app::utility::get_kind(boma);

            let fly_data = if let Some(data) = FlyData::get_from_fighter_kind(kind) {
                data
            } else {
                return;
            };

            let stable_speed = WorkModule::get_param_float(boma, smash::hash40("air_speed_x_stable"), 0);
            let mut speed = *energy.get_speed();

            let sum = speed.x + speed.y;
            let cap = initial_speed.x * fly_data.speed_x_max_mul;

            if cap.abs() < sum.abs() {
                energy.speed.x = cap.abs() * speed.x.signum();
            }
        }, // not reached in game afaik
        Dash | DashBack => {
            let dash_speed = if reset_type == DashBack {
                -energy.lr * WorkModule::get_param_float(boma, smash::hash40("dash_speed"), 0)
            } else {
                energy.lr * WorkModule::get_param_float(boma, smash::hash40("dash_speed"), 0)
            };
            energy.speed.x = if energy.speed.x * energy.lr >= 0.0 {
                if reset_type == DashBack {
                    dash_speed + energy.speed.x
                } else {
                    dash_speed
                }
            } else {
                if reset_type == DashBack {
                    dash_speed
                } else {
                    dash_speed + energy.speed.x
                }
            };
        },
        ShootDash => {
            energy.speed.x = if 0.0 <= energy.speed.x * energy.lr {
                energy.lr * WorkModule::get_param_float(boma, smash::hash40("shoot_dash_speed_f"), 0)
            } else {
                energy.speed.x + energy.lr * WorkModule::get_param_float(boma, smash::hash40("shoot_dash_speed_f"), 0)
            };
        },
        ShootBackDash => {
            energy.speed.x = if 0.0 <= energy.speed.x * energy.lr {
                -energy.lr * WorkModule::get_param_float(boma, smash::hash40("shoot_dash_speed_b"), 0)
            } else {
                energy.speed.x - energy.lr * WorkModule::get_param_float(boma, smash::hash40("shoot_dash_speed_b"), 0)
            };
        },
        RevolveSlashAir => {
            energy.speed.x *= WorkModule::get_param_float(boma, smash::hash40("rslash_air_spd_x_mul"), 0);
        },
        Free => {
            energy.speed = PaddedVec2::zeros();
        },
        MoveGround => {
            let new_speed = super::energy::KineticEnergy::adjust_speed_for_ground_normal(&energy.speed, boma);
            energy.speed = new_speed;
        }        
        _ => {}
    }

    energy.initialize(boma);
}

pub fn install() {
    skyline::install_hooks!(
        update,
        initialize,
        setup
    );
}