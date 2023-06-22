use super::*;
use crate::consts::*;
use crate::consts::globals::*;
use std::ops::{Deref, DerefMut};


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EnergyMotionResetType {
    GroundTransLoop = 0x0,
    GroundTransLoopGekikara,
    GroundTrans,
    GroundTransIgnoreNorm,
    AirTrans,
    AirTransAngle,
    AirTransY,
    AirTransAngleSuperJumpPunch,
    AirTrans2nd,
    CliffTransIntp,
    CliffTrans,
    CliffTransGround,
    LadderMove,
    LadderTrans
}

impl EnergyMotionResetType {
    pub fn is_ground(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, GroundTransLoop | GroundTransLoopGekikara | GroundTrans | GroundTransIgnoreNorm)
    }

    pub fn is_air(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, AirTrans | AirTransAngle | AirTransY | AirTransAngleSuperJumpPunch | AirTrans2nd)
    }

    pub fn is_cliff(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, CliffTransIntp | CliffTrans | CliffTransGround)
    }

    pub fn is_ladder(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, LadderMove | LadderTrans)
    }

    pub fn is_2nd(self) -> bool {
        use EnergyMotionResetType::*;
        matches!(self, AirTrans2nd)
    }
}

#[repr(C)]
pub struct FighterKineticEnergyMotion {
    parent: super::energy::KineticEnergy,
    pub lr: f32,
    pub angle: f32,
    pub angle_whole: f32,
    pub angle_intp_end: f32,
    pub angle_intp_frames_remaining: i32,
    pub speed_mul: f32,
    pub prev_speed: PaddedVec2,
    pub speed_mul_2nd: PaddedVec2,
    pub update_flag: bool,
    // ...
}

impl Deref for FighterKineticEnergyMotion {
    type Target = super::energy::KineticEnergy;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterKineticEnergyMotion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

impl FighterKineticEnergyMotion {
    /// Calls a MotionModule vtable function to update the trans move speed (2nd)
    pub fn update_trans_move_speed_2nd(boma: &mut BattleObjectModuleAccessor) {
        unsafe {
            let motion_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88 / 0x8);
            let motion_module_vtable = *(motion_module as *const *const u64);
            let function: extern "C" fn(u64) = std::mem::transmute(*motion_module_vtable.add(0x220 / 0x8));
            function(motion_module);
        }
    }

    /// Checks if the motion (2nd) is updating the kinetic energy
    pub fn is_motion_2nd_updating_energy(boma: &mut BattleObjectModuleAccessor) -> bool {
        unsafe {
            let motion_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88 / 0x8);
            let motion_module_vtable = *(motion_module as *const *const u64);
            let function: extern "C" fn(u64) -> bool = std::mem::transmute(*motion_module_vtable.add(0x1f0 / 0x8));
            function(motion_module)
        }
    }

    /// Checks if the motion is updating the kinetic energy
    pub fn is_main_motion_updating_energy(boma: &mut BattleObjectModuleAccessor) -> bool {
        unsafe {
            let motion_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88 / 0x8);
            let motion_module_vtable = *(motion_module as *const *const u64);
            let function: extern "C" fn(u64) -> bool = std::mem::transmute(*motion_module_vtable.add(0x1e8 / 0x8));
            function(motion_module)
        }
    }

    pub fn trans_move_speed_correct(boma: &mut BattleObjectModuleAccessor) -> Vector3f {
        unsafe {
            let func: extern "C" fn(&mut BattleObjectModuleAccessor) -> energy::Vec3 = std::mem::transmute(MotionModule::trans_move_speed as *const ());
            let vec = func(boma);
            Vector3f {
                x: vec.x,
                y: vec.y,
                z: vec.z
            }
        }
    }

    pub fn trans_move_speed_2nd_correct(boma: &mut BattleObjectModuleAccessor) -> Vector3f {
        unsafe {
            let func: extern "C" fn(&mut BattleObjectModuleAccessor) -> energy::Vec3 = std::mem::transmute(MotionModule::trans_move_speed_2nd as *const ());
            let vec = func(boma);
            Vector3f {
                x: vec.x,
                y: vec.y,
                z: vec.z
            }
        }
    }

    /// Sets some of the main behavioral values of the KineticEnergy and performs processing on it
    /// # Arguments
    /// * `accel` - The acceleration of the energy
    /// * `max_speed` - The maximum speed of the energy
    /// * `speed` - The speed that we are attempting to accelerate to
    pub fn set_values_and_process(&mut self, accel: PaddedVec2, max_speed: PaddedVec2, speed: PaddedVec2, boma: &mut BattleObjectModuleAccessor) {
        self.accel = accel;
        self.speed_max = max_speed;
        self.process(boma);
        self.active_flag = true;
        self.prev_speed = speed;
    }

    /// Gets the translation based on the specified energy reset type
    /// # Arguments
    /// * `boma` - The BattleObjectModuleAccessor
    /// * `reset_type` - The reset type of the current energy
    /// # Returns
    /// The translation as a Vec2
    pub fn get_translation_by_reset_type(boma: &mut BattleObjectModuleAccessor, reset_type: EnergyMotionResetType) -> PaddedVec2 {
        let translation = unsafe {
            if reset_type.is_2nd() {
                Self::update_trans_move_speed_2nd(boma);
                Self::trans_move_speed_2nd_correct(boma)
            } else {
                MotionModule::update_trans_move_speed(boma);
                Self::trans_move_speed_correct(boma)
            }
        };

        PaddedVec2::new(translation.z, translation.y)
    }

    /// Checks if the animation is updating the kinetic energy, depending on the EnergyMotionResetType
    /// # Arguments
    /// * `boma` - The BattleObjectModuleAccessor
    /// * `reset_type` - The reset type of the current energy
    pub fn is_motion_updating_energy(boma: &mut BattleObjectModuleAccessor, reset_type: EnergyMotionResetType) -> bool {
        if reset_type.is_2nd() {
            Self::is_motion_2nd_updating_energy(boma)
        } else {
            Self::is_main_motion_updating_energy(boma)
        }
    }
}

// This function references BattleObjectWorld, which is defo for the ledge positions
#[skyline::from_offset(0x6941c0)]
extern "C" fn handle_cliff(boma: &mut BattleObjectModuleAccessor, vec: &Vector4f) -> energy::Vec4;

#[skyline::hook(offset = 0x6d5c90)]
unsafe fn motion_update(energy: &mut FighterKineticEnergyMotion, boma: &mut BattleObjectModuleAccessor) {
    use EnergyMotionResetType::*;
    let reset_type = std::mem::transmute(energy.energy_reset_type);

    energy.active_flag = true;
    if !FighterKineticEnergyMotion::is_motion_updating_energy(boma, reset_type) {
        if reset_type == LadderMove {
            // If we are on a ladder, we need to **immediately** stop moving if the MotionModule is no longer updating our position
            // By setting the acceleration to negative of our speed, we are immediately stopping our movement. This should not be applied
            // to any other state as you will experience absolutely 0 momentum in odd situations.
            //
            // A bug that I encountered when reimplementing was setting this for the grounded states as well, which caused things like jumpsquat
            // and landing to immediately stop all momentum
            energy.set_values_and_process(
                PaddedVec2::new(-energy.speed.x, -energy.speed.y),
                PaddedVec2::zeros(),
                PaddedVec2::zeros(),
                boma
            );
            return;
        }

        // Set our grounded speed limit if we are on the ground
        // This is applied in situations like landing (which includes wavetech in HDR)
        if reset_type.is_ground() {
            energy.speed_limit = PaddedVec2::new(
                WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ground_speed_limit")),
                0.0
            );
        }

        // Basically we are setting our maximum speed to 0.0, which means that we are going to start slowing down to that speed with the use of only
        // our brake value
        // For ground, this is `ground_brake`, for example. That's the only thing applied here.
        // If you wanted to, say apply double traction in this situation you could double the energy.brake temporarily and restore it afterwards
        // as done in the control kinetic energy for some situations
        energy.set_values_and_process(
            PaddedVec2::zeros(),
            PaddedVec2::zeros(),
            PaddedVec2::zeros(),
            boma
        );

        return;
    }

    // begin block for calculating move speed based on animation

    let mut move_speed = FighterKineticEnergyMotion::get_translation_by_reset_type(boma, reset_type);

    // It appears that some motions can set the angle to change over a set number of frames
    // This will decrease the angle by the same amount each frame until there are no more frames remaining
    if energy.angle_intp_frames_remaining >= 1 {
        energy.angle_whole += (energy.angle_intp_end - energy.angle_whole) / energy.angle_intp_frames_remaining as f32;
        energy.angle_intp_frames_remaining -= 1;
    };

    // `angle_whole` is the angle to rotate the energy by, regardless of whether or not its reset type is AirTransAngle
    if energy.angle_whole != 0.0 {
        move_speed.x = move_speed.x * energy.angle_whole.cos() - move_speed.y * energy.angle_whole.sin();
        move_speed.y = move_speed.y * energy.angle_whole.cos() + move_speed.x * energy.angle_whole.sin();
    }

    // PostureModule::lr is used when the animation is looping or when it depends on the fighter's angle, probably because other
    // reset types are allowed to change the LR of the fighter
    let lr = if matches!(reset_type, GroundTransLoop | GroundTransLoopGekikara | AirTransAngle | AirTransAngleSuperJumpPunch) {
        PostureModule::lr(boma)
    } else {
        energy.lr
    };

    // energy.speed_mul is a singular scalar, whereas speed_mul_2nd is a scalar with components
    move_speed.x *= lr * energy.speed_mul * energy.speed_mul_2nd.x;
    move_speed.y *= energy.speed_mul * energy.speed_mul_2nd.y;

    // end calculating move_speed from trans

    // the following flag is set for the same reset types mentioned in the above LR check, except for AirTransAngleSuperJumpPunch
    energy.active_flag = false;

    if boma.status_frame() == 0 {
        move_speed.x = energy.prev_speed.x;
        if reset_type.is_air() || reset_type.is_cliff() {
            move_speed.y = energy.prev_speed.y;
        }
    }

    //println!("{}", move_speed.x);

    let speed = match reset_type {
        // It appears that when grounded and your animation is controlling your kinetic energy,
        // there is no grounded speed limit.
        //
        // This is the same for all of the grounded reset types
        GroundTransLoop => {
            energy.active_flag = true;
            energy.speed_limit = PaddedVec2::new(-1.0, 0.0);
            move_speed
        },
        
        GroundTransIgnoreNorm => {
            energy.speed_limit = PaddedVec2::new(-1.0, 0.0);
            move_speed
        },

        // I'm still not quit sure what this "active_flag" is, but it's unset for these moves and reset for other moves?
        // Enabling it for this and the previous reset_types doesn't appear to have any different behavior off a few quick tests
        GroundTrans => {
            energy.speed_limit = PaddedVec2::new(-1.0, 0.0);
            energy::KineticEnergy::adjust_speed_for_ground_normal(&move_speed, boma)
        },

        // Haven't quite figured out where this gets used yet, and the work const has a few hits so I'm just not quite sure
        GroundTransLoopGekikara => {
            energy.active_flag = true;
            energy.speed_limit = PaddedVec2::new(-1.0, 0.0);
            let some_rate = WorkModule::get_float(boma, 0x1000009);
            let motion_rate = MotionModule::rate(boma);
            if some_rate != 0.0 && motion_rate / some_rate != 0.0 {
                PaddedVec2::new(
                    move_speed.x * some_rate / motion_rate,
                    move_speed.y * some_rate / motion_rate
                )
            } else {
                PaddedVec2::zeros()
            }
        },

        // When you are in the air your speed doesn't have any special calculations
        AirTrans | AirTrans2nd => move_speed,

        // This multiplies by the angle set with (afaik) app::sv_kinetic_energy::set_angle
        // Set angle whole is used regardless of the energy reset type
        AirTransAngle => {
            PaddedVec2::new(
                move_speed.x * energy.angle.cos() - move_speed.y * energy.angle.sin(),
                move_speed.y * energy.angle.cos() + move_speed.x * energy.angle.sin()
            )
        },

        // Here we zero out the X speed and literally only use the Y speed. Epic!
        AirTransY => PaddedVec2::new(0.0, move_speed.y),

        // This is the most complex (aside from cliff stuff) and it appears to adjust your momentum depending on where you are holding on the stick
        // It looks like it can change variadically throughout the animation, which is most likely unique for these kinds of moves
        AirTransAngleSuperJumpPunch => {
            let stick_x = ControlModule::get_stick_x(boma);
            let dir = WorkModule::get_float(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
            let angle = if stick_x.abs() <= dir {
                energy.angle
            } else {
                let interp = (stick_x.abs() - dir) / (1.0 - dir);
                let interp = interp * WorkModule::get_float(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
                let new_angle = if stick_x <= 0.0 {
                    interp.to_radians()
                } else {
                    -interp.to_radians()
                };
                if energy.angle.abs() < new_angle.abs() {
                    energy.angle = new_angle;
                }
                energy.angle
            };
            PaddedVec2::new(
                move_speed.x * angle.cos() - move_speed.y * energy.angle.sin(),
                move_speed.y * angle.cos() + move_speed.x * energy.angle.sin()
            )
        },

        // Cliff functions require using a dedicated function, probably to figure out where the fighter needs to move
        // to in order to complete the cliff catch
        // These likely happen in a very brief, perhaps only 1 frame, window
        CliffTransIntp | CliffTrans | CliffTransGround => {

            let motion_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88 / 0x8);
            let motion_vtable = *(motion_module as *const *const u64);
            let some_func: extern "C" fn(u64) -> energy::Vec4 = std::mem::transmute(*motion_vtable.add(0x230 / 0x8));
            let vec = some_func(motion_module);
            let vec = Vector4f {
                x: vec.x,
                y: vec.y,
                z: vec.z,
                w: vec.w
            };
            if reset_type == CliffTransGround {
                energy.active_flag = true;
            }
            let vec = handle_cliff(boma, &vec);
            if reset_type == CliffTransIntp {
                let frame = WorkModule::get_int(boma, 0x11000005);
                let interpolated = 1.0 / (frame + 1) as f32;
                PaddedVec2::new(vec.x * interpolated, vec.y * interpolated)
            } else {
                PaddedVec2::new(vec.x, vec.y)
            }
        },

        // LadderMove appears to be for when you are actually moving up/down the later
        LadderMove => {
            energy.active_flag = true;
            let stick_y = ControlModule::get_stick_y(boma);
            let speed_y = if 0.5 <= stick_y.abs() {
                if stick_y <= 0.0 {
                    -WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ladder_speed_d_max")) * MotionModule::rate(boma)
                } else {
                    WorkModule::get_param_float(boma, smash::hash40("common"), smash::hash40("ladder_speed_u_max")) * MotionModule::rate(boma)
                }
            } else {
                0.0
            };

            PaddedVec2::new(0.0, speed_y)
        },

        // As opposed to LadderMove, LadderTrans is likely for when you are getting on/off the ladder
        // 
        // The reason I say this, is due to a bug in reimplementation, when you would get off the ladder you
        // would meet god in the top blastzone
        LadderTrans => {
            energy.active_flag = true;
            let ladder_end_y = WorkModule::get_float(boma, *FIGHTER_STATUS_LADDER_WORK_FLOAT_LADDER_END_Y);
            let ladder_end_start_y = WorkModule::get_float(boma, *FIGHTER_STATUS_LADDER_WORK_FLOAT_LADDER_END_START_Y);
            let mut vec = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            MotionModule::trans_tra(boma, &mut vec, true, true);
            let speed_y = (ladder_end_y + vec.y) - ladder_end_start_y;
            WorkModule::add_float(boma, speed_y, *FIGHTER_STATUS_LADDER_WORK_FLOAT_LADDER_END_START_Y);
            PaddedVec2::new(0.0, speed_y)
        }
        // _ => {}
    };

    // It is unclear to me why this specific case is handled so explicitly, but it is
    if reset_type.is_ground() && energy.update_flag && speed.x == 0.0 && energy.prev_speed.x == 0.0 {
        energy.set_values_and_process(
            PaddedVec2::zeros(),
            PaddedVec2::zeros(),
            speed,
            boma
        );
        return;
    }

    // energy.update_flag determines whether or not we have gone for at least a frame of movement
    // Here are the two possible situations for when this code gets run:
    // 1.) This is the first frame of motion energy since our energy change. I will take, for example,
    //      transition from run into jump squat. On the first frame of jump squat, you don't have "previous speed",
    //      aside from the speed that existed for the entire character on the frame before jump squat
    //      so that is what you want to change from
    // 2.) This is not the first frame of motion energy, so you have a previous frame's energy to change from
    let speed_to_change_from = if energy.update_flag {
        energy.prev_speed
    } else {
        energy.update_flag = true;
        energy.speed
    };

    // Since acceleration is just the difference in speed between two frames, just subtract where we want to be 
    // and where we were/are
    energy.set_values_and_process(
        PaddedVec2::new(speed.x - speed_to_change_from.x, speed.y - speed_to_change_from.y),
        PaddedVec2::new(-1.0, -1.0),
        speed,
        boma
    );
}

pub fn install() {
    skyline::install_hooks!(
        motion_update
    );
}