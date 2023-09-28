use super::*;
mod aerials;
mod tilts;
mod other;
mod smashes;
mod specials;
mod throws;
mod ground;

#[repr(C)]
pub struct StanceInfo {
    label: i32,
    damage_bite: f32,
    damage_head: f32,
    damage_other: f32,
    da_speed: f32
}

impl From<i32> for StanceInfo {
    fn from(other: i32) -> Self {
        match other {
            0 => StanceInfo { // Regular
                label: 0,
                damage_bite: 0.9,
                damage_head: 0.9,
                damage_other: 0.9,
                da_speed: 0.8
            },
            1 => StanceInfo { // Putrid
                label: 1,
                damage_bite: 0.85,
                damage_head: 0.7,
                damage_other: 0.8,
                da_speed: (0.8 * 0.94)
            },
            2 => StanceInfo { // Prickly
                label: 2,
                damage_bite: 1.05,
                damage_head: 1.3,
                damage_other: 1.2,
                da_speed: (0.8 * 0.84)
            },
            _ => StanceInfo { // same as regular
                label: 3,
                damage_bite: 0.9,
                damage_head: 0.9,
                damage_other: 0.9,
                da_speed: 0.8
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