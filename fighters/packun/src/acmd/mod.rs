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
    damage_nspecial: f32,
    damage_sspecial: f32,
    flinchless_sspecial: bool,
    bkb_sspecial: i32,
    rehit_sspecial: i32,
    damage_dspecial: f32,
    delta_angle: u64,
    ptooie_size: f32
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
                damage_nspecial: 1.0,
                damage_sspecial: 1.0,
                flinchless_sspecial: true,
                bkb_sspecial: 0,
                rehit_sspecial: 1,
                damage_dspecial: 1.0,
                delta_angle: 0,
                ptooie_size: 1.0
            },
            1 => StanceInfo { // Toxic
                label: 1,
                damage_bite: 1.1,
                damage_head: 0.75,
                damage_other: 0.9,
                bkb_normals: 20,
                damage_nspecial: 0.9,
                damage_sspecial: 1.5,
                flinchless_sspecial: true,
                bkb_sspecial: 0,
                rehit_sspecial: 1,
                damage_dspecial: 1.1,
                delta_angle: 0,
                ptooie_size: 1.0
            },
            2 => StanceInfo { // Brute
                label: 2,
                damage_bite: 1.05,
                damage_head: 1.4,
                damage_other: 1.2,
                bkb_normals: 0,
                damage_nspecial: 1.25,
                damage_sspecial: 0.95,
                flinchless_sspecial: false,
                bkb_sspecial: 40,
                rehit_sspecial: 2,
                damage_dspecial: 1.3,
                delta_angle: 8,
                ptooie_size: 1.5
            },
            _ => StanceInfo { // same as regular
                label: 3,
                damage_bite: 1.0,
                damage_head: 1.0,
                damage_other: 1.0,
                bkb_normals: 0,
                damage_nspecial: 1.0,
                damage_sspecial: 1.0,
                flinchless_sspecial: true,
                bkb_sspecial: 0,
                rehit_sspecial: 1,
                damage_dspecial: 1.0,
                delta_angle: 0,
                ptooie_size: 1.0
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