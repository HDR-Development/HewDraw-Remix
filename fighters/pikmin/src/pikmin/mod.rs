use super::*;
pub mod acmd;
pub mod status;

#[repr(C)]
pub struct PikminInfo {
    dmg: f32,
    shield_dmg: f32,
    hitlag: f32,
    attr: Hash40,
    attr_special: Hash40,
    sound: i32,
    angle: u64,
    color: Vector3f,
    cling_frame: i32
}

impl From<i32> for PikminInfo {
    fn from(other: i32) -> Self {
        match other {
            0 => PikminInfo { // Red
                dmg: 1.0,
                shield_dmg: 0.5,
                angle: 0,
                hitlag: 1.0,
                attr: Hash40::new("collision_attr_fire"),
                attr_special: Hash40::new("collision_attr_fire"),
                sound: *COLLISION_SOUND_ATTR_FIRE,
                color: Vector3f{x: 1.0, y: 0.05, z: 0.0},
                cling_frame: 30 * 4
            },
            1 => PikminInfo { // yellow
                dmg: 0.94,
                shield_dmg: 0.0,
                angle: 8,
                hitlag: 1.25,
                attr: Hash40::new("collision_attr_elec"),
                attr_special: Hash40::new("collision_attr_elec"),
                sound: *COLLISION_SOUND_ATTR_ELEC,
                color: Vector3f{x: 1.0, y: 1.0, z: 0.14},
                cling_frame: 30 * 6
            },
            2 => PikminInfo { // Blue
                dmg: 1.0,
                shield_dmg: 0.0,
                angle: 5,
                hitlag: 1.0,
                attr: Hash40::new("collision_attr_water"),
                attr_special: Hash40::new("collision_attr_water"),
                sound: *COLLISION_SOUND_ATTR_WATER,
                color: Vector3f{x: 0.1, y: 0.4, z: 1.0},
                cling_frame: 30 * 4
            },
            3 => PikminInfo { // White
                dmg: 0.75,
                shield_dmg: 0.75,
                angle: 8,
                hitlag: 1.0,
                attr: Hash40::new("collision_attr_purple"),
                attr_special: Hash40::new("collision_attr_flower"),
                sound: *COLLISION_SOUND_ATTR_FIRE,
                color: Vector3f{x: 1.0, y: 1.0, z: 1.0},
                cling_frame: 30 * 2
            },
            _ => PikminInfo { // Violet (Rock), also default
                dmg: 1.2,
                shield_dmg: 0.1,
                angle: 0,
                hitlag: 1.0,
                attr: Hash40::new("collision_attr_normal"),
                attr_special: Hash40::new("collision_attr_normal"),
                sound: *COLLISION_SOUND_ATTR_KICK,
                color: Vector3f{x: 0.36, y: 0.0, z: 1.0},
                cling_frame: 30 * 999
            },
        }
    }
}

pub fn install(is_runtime: bool) {
    acmd::install();
    status::install();
}