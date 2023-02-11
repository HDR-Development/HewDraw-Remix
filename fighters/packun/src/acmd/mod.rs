use super::*;
mod aerials;
mod tilts;
mod other;
mod smashes;
mod specials;
mod throws;
mod ground;

pub struct StanceInfo {
    label: i32,
    damage_bite: f32,
    damage_head: f32,
    damage_other: f32,
    bkb_normals: i32,
    delta_angle: u64
}

impl From<i32> for StanceInfo {
    fn from(other: i32) -> Self {
        match other {
            0 => StanceInfo { // Regular
                label: 0,
                damage_bite: 1.0,
                damage_head: 1.0,
                damage_other: 1.0,
                bkb_normals: 0,
                delta_angle: 0
            },
            1 => StanceInfo { // Putrid
                label: 1,
                damage_bite: 0.8,
                damage_head: 0.75,
                damage_other: 0.9,
                bkb_normals: 10,
                delta_angle: 0
            },
            2 => StanceInfo { // Prickly
                label: 2,
                damage_bite: 1.05,
                damage_head: 1.4,
                damage_other: 1.2,
                bkb_normals: 0,
                delta_angle: 8
            },
            _ => StanceInfo { // same as regular
                label: 3,
                damage_bite: 1.0,
                damage_head: 1.0,
                damage_other: 1.0,
                bkb_normals: 0,
                delta_angle: 0
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