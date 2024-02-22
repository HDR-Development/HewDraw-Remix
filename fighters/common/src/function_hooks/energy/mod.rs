use super::*;
mod control;
mod motion;
mod stop;

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
        #[skyline::from_offset(0x47b4f0)]        
        extern "C" fn adjust_speed_for_ground_normal_internal(speed: Vec2, boma: &mut BattleObjectModuleAccessor) -> Vec2;

        unsafe {
            let result = adjust_speed_for_ground_normal_internal(Vec2 { x: speed.x, y: speed.y }, boma);
            PaddedVec2::new(result.x, result.y)
        }
    }

    pub fn process(&mut self, boma: &mut BattleObjectModuleAccessor) {
        unsafe {
            #[skyline::from_offset(0x47bf90)]
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

pub fn install() {
    control::install();
    motion::install();
    stop::install();
}