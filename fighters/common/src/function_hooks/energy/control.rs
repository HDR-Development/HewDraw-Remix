use super::*;
use crate::consts::*;
use crate::consts::globals::*;
use std::ops::{Deref, DerefMut};

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

#[skyline::hook(offset = 0x6d3630)]
unsafe fn control_update(energy: &mut FighterKineticEnergyControl, boma: &mut BattleObjectModuleAccessor) {
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
        Dash | DashBack => loop {
            // Don't apply or change the speed by any ammount during the first keep frames of dash
            let dash_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_DASH_WORK_INT_COUNT);
            let keep_frame = WorkModule::get_param_int(boma, smash::hash40("common"), smash::hash40("dash_speed_keep_frame"));
            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DASH || reset_type == DashBack {
                if dash_frame < keep_frame {
                    energy.speed_max.x = 0.0;
                    energy.speed_brake.x = 0.0;
                    stick.x = accel_add_x; // not sure if this is accurate but it's what I think I saw in the code
                    break 0.0;
                }
                else if dash_frame == keep_frame {
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
                }
            } else if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_TURN_DASH {
                if WorkModule::get_int(boma, *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FROM_DASH_COUNT) < keep_frame {
                    energy.speed_max.x = 0.0;
                    energy.speed_brake.x = 0.0;
                    stick.x = accel_add_x;
                    break 0.0;
                }
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
                energy.accel.y = 0.0;
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
            energy.accel.y = accel_add_y * stick.y.signum() + stick.y * energy.accel_mul_y;
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
        Turn | TurnNoStop | TurnNoStopAir => {
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
        let mut set_speed_max = true;

        energy.accel.x = accel_diff;
        let speed_max = energy.speed_max.x * stick.x.abs();
    
        if energy.unk[1] != 0 {
            if !(((energy._x9c != 0.0 && (stick.x <= 0.0 || energy._xa0 <= 0.0 || speed_max.abs() <= energy._x9c.abs()))
            && (stick.x >= 0.0 || energy._xa0 >= 0.0 || speed_max.abs() <= energy._x9c.abs()))
            && ((stick.x <= 0.0 || 0.0 <= energy._xa0) && (0.0 <= stick.x || energy._xa0 <= 0.0)))
            {
                energy._x9c = speed_max;
                energy._xa0 = stick.x;
            }

        }

        if set_speed_max {
            energy.speed_max.x = speed_max;
        }
    }

    // Double air brake value when above max horizontal jump speed
    if boma.status_frame() > 0 {
        let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
        let ratio = VarModule::get_float(boma.object(), vars::common::instance::JUMP_SPEED_RATIO);
        // get the multiplier for any special mechanics that require additional jump speed max (meta quick, etc)
        let mut jump_speed_max_mul = VarModule::get_float(boma.object(), vars::common::instance::JUMP_SPEED_MAX_MUL);
        if jump_speed_max_mul < 0.1 || jump_speed_max_mul > 3.0 {
            // if its not between 0.1 and 3.0, it is likely not a real value and we should ignore it
            jump_speed_max_mul = 1.0;
        }
        let jump_speed_x_max = run_speed_max * ratio * jump_speed_max_mul;
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR 
        && energy.speed.x.abs() >= jump_speed_x_max {
            energy.speed_brake.x *= 2.0;
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

#[skyline::hook(offset = 0x6d4060)]
unsafe fn control_initialize(energy: &mut FighterKineticEnergyControl, boma: &mut BattleObjectModuleAccessor) {
    use EnergyControllerResetType::*;
    let reset_type = std::mem::transmute(energy.energy_reset_type);
    match reset_type {
        FallAdjust | FallAdjustNoCap | StopCeil | WallJump => {
            let mut stable_speed = WorkModule::get_param_float(boma, smash::hash40("air_speed_x_stable"), 0);
            let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
            let ratio = VarModule::get_float(boma.object(), vars::common::instance::JUMP_SPEED_RATIO);
            // get the multiplier for any special mechanics that require additional jump speed max (meta quick, etc)
            let mut jump_speed_max_mul = VarModule::get_float(boma.object(), vars::common::instance::JUMP_SPEED_MAX_MUL);
            if jump_speed_max_mul < 0.1 || jump_speed_max_mul > 3.0 {
                // if its not between 0.1 and 3.0, it is likely not a real value and we should ignore it
                jump_speed_max_mul = 1.0;
            }
            let jump_speed_x_max = run_speed_max * ratio * jump_speed_max_mul;

            if reset_type == StopCeil {
                stable_speed *= WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("stop_ceil_speed_x_stable_mul"));
            }

            energy.speed_max = PaddedVec2::new(stable_speed, -1.0);
            energy.speed_brake = PaddedVec2::new(WorkModule::get_param_float(boma, smash::hash40("air_brake_x"), 0), 0.0);
            let air_x_speed_max = if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT) && energy.unk[2] == 0 {
                WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("air_speed_x_limit"))
            } else {
                if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP {
                    jump_speed_x_max
                } else {
                    -1.0
                }
            };
            energy.speed_limit = PaddedVec2::new(air_x_speed_max, 0.0);
            energy.accel_mul_x = WorkModule::get_param_float(boma, smash::hash40("air_accel_x_mul"), 0);
            energy.accel_add_x = WorkModule::get_param_float(boma, smash::hash40("air_accel_x_add"), 0);
            VarModule::set_float(boma.object(), vars::common::instance::JUMP_SPEED_MAX_MUL, 1.0);
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
                WorkModule::get_param_float(boma, smash::hash40("walk_speed_max"), 0),
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

#[skyline::hook(offset = 0x6d4bc0)]
unsafe fn control_setup(energy: &mut FighterKineticEnergyControl, reset_type: EnergyControllerResetType, initial_speed: &Vector3f, unk: u64, boma: &mut BattleObjectModuleAccessor) {
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

    if boma.is_situation(*SITUATION_KIND_AIR) {
        // characters whose special moves should conserve momentum
        let should_conserve_special_momentum =
        ( [*FIGHTER_KIND_MARIO, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_GANON, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_FOX, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_WOLF]
            .contains(&boma.kind()) && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) )
        || ( boma.kind() == *FIGHTER_KIND_DIDDY && boma.is_status_one_of(&[*FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_SHOOT]) );

        if should_conserve_special_momentum {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        }
    }
    
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
        control_update,
        control_initialize,
        control_setup
    );
}