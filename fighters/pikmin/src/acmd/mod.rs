use super::*;
use smash::lib::lua_const::*;

mod aerials;
mod ground;
mod other;
mod smashes;
mod specials;
mod throws;
mod tilts;

#[repr(C)]
pub struct PikminInfo {
    damage: f32,
    //kbg: i32,
    //bkb: i32,
    shield_damage: f32,
    hitlag: f32,
    attr: &'static str,
    sound: i32,
    delta_angle: u64
}

impl From<i32> for PikminInfo {
    fn from(other: i32) -> Self {
        match other {
            0 => PikminInfo { // Red
                damage: 1.0,
                //kbg: 1,
                //bkb: 1,
                shield_damage: 0.1,
                delta_angle: 0,
                hitlag: 1.1,
                attr: "collision_attr_fire",
                sound: *COLLISION_SOUND_ATTR_FIRE
            },
            1 => PikminInfo { // yellow
                damage: 0.75,
                //kbg: 1,
                //bkb: 1,
                shield_damage: 0.0,
                delta_angle: 8,
                hitlag: 1.0,
                attr: "collision_attr_elec",
                sound: *COLLISION_SOUND_ATTR_ELEC
            },
            2 => PikminInfo { // Blue
                damage: 0.8,
                //kbg: 1,
                //bkb: 1,
                shield_damage: 0.0,
                delta_angle: 5,
                hitlag: 1.0,
                attr: "collision_attr_water",
                sound: *COLLISION_SOUND_ATTR_WATER
            },
            3 => PikminInfo { // White
                damage: 0.6,
                //kbg: 1,
                //bkb: 1,
                shield_damage: 0.75,
                delta_angle: 8,
                hitlag: 1.0,
                attr: "collision_attr_purple",
                sound: *COLLISION_SOUND_ATTR_FIRE
            },
            _ => PikminInfo { // Violet (Rock), also default
                damage: 0.8,
                //kbg: 1,
                //bkb: 1,
                shield_damage: 0.5,
                delta_angle: 0,
                hitlag: 1.2,
                attr: "collision_attr_normal",
                sound: *COLLISION_SOUND_ATTR_KICK
            },
        }
    }
}

pub fn install() {
    aerials::install();
    tilts::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
    ground::install();
}