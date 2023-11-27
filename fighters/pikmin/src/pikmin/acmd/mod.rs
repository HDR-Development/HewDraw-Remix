use super::*;
use smash::lib::lua_const::*;

mod aerials;
mod other;
mod smashes;
mod specials;
mod throws;

#[repr(C)]
pub struct PikminInfo {
    dmg: f32,
    kbg: i32,
    bkb: i32,
    shield_dmg: f32,
    hitlag: f32,
    attr: &'static str,
    sound: i32,
    angle: u64,
    color: Vector3f
}

impl From<i32> for PikminInfo {
    fn from(other: i32) -> Self {
        match other {
            0 => PikminInfo { // Red
                dmg: 1.0,
                kbg: 8,
                bkb: 0,
                shield_dmg: 0.1,
                angle: 0,
                hitlag: 1.1,
                attr: "collision_attr_fire",
                sound: *COLLISION_SOUND_ATTR_FIRE,
                color: Vector3f{x: 1.0, y: 0.05, z: 0.0}
            },
            1 => PikminInfo { // yellow
                dmg: 0.75,
                kbg: 0,
                bkb: 0,
                shield_dmg: 0.0,
                angle: 8,
                hitlag: 1.0,
                attr: "collision_attr_elec",
                sound: *COLLISION_SOUND_ATTR_ELEC,
                color: Vector3f{x: 1.0, y: 1.0, z: 0.14}
            },
            2 => PikminInfo { // Blue
                dmg: 0.8,
                kbg: 0,
                bkb: 0,
                shield_dmg: 0.0,
                angle: 5,
                hitlag: 1.0,
                attr: "collision_attr_water",
                sound: *COLLISION_SOUND_ATTR_WATER,
                color: Vector3f{x: 0.1, y: 0.4, z: 1.0}
            },
            3 => PikminInfo { // White
                dmg: 0.6,
                kbg: 0,
                bkb: 0,
                shield_dmg: 0.75,
                angle: 8,
                hitlag: 1.0,
                attr: "collision_attr_purple",
                sound: *COLLISION_SOUND_ATTR_FIRE,
                color: Vector3f{x: 1.0, y: 1.0, z: 1.0}
            },
            _ => PikminInfo { // Violet (Rock), also default
                dmg: 0.8,
                kbg: 0,
                bkb: 0,
                shield_dmg: 0.5,
                angle: 0,
                hitlag: 1.2,
                attr: "collision_attr_normal",
                sound: *COLLISION_SOUND_ATTR_KICK,
                color: Vector3f{x: 0.36, y: 0.0, z: 1.0}
            },
        }
    }
}

pub fn install() {
    aerials::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
}